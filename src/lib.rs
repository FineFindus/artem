//! # artem
//! `artem` is a program to convert images to ascii art.
//! While it's primary usages is through the command line, it also provides a rust crate.
//!
//! # Usage
//! To use it, load an image using the [image crate](https://crates.io/crates/image) and pass it to
//! artem. Addiontially the [`crate::convert`] function takes an [`crate::config::Config`], which can be used to configure
//! the resulting output. Whilst [`crate::config::Config`] implements [`Default`], it is
//! recommended to do the configuration through [`crate::config::ConfigBuilder`] instead.
//! ```
//! # let path = "./assets/images/standard_test_img.png";
//! let image = image::open(path).expect("Failed to open image");
//! let ascii_art = artem::convert(image, &artem::config::ConfigBuilder::new().build());
//! ```

//condense all arguments into a single struct
pub mod config;

//functions for working with pixels
mod pixel;

//outlining filter
mod filter;
//functions for dealing with output targets/files
mod target;

use std::sync::LazyLock;

use image::{DynamicImage, GenericImageView};

pub use crate::config::ConfigBuilder;
use crate::config::{Config, ResizingDimension, TargetType};

/// Takes an image and returns it as an ascii art string.
///
/// The result can be changed using the [`crate::config::Config`] argument
/// # Examples
/// ```no_run
/// use artem::config::ConfigBuilder;
///
/// let img = image::open("examples/abraham_lincoln.jpg").unwrap();
/// let converted_image = artem::convert(img, &ConfigBuilder::new().build());
/// ```
pub fn convert(image: DynamicImage, config: &Config) -> String {
    log::debug!("Using inverted color: {}", config.invert);
    //get img dimensions
    let input_width = image.width();
    let input_height = image.height();
    log::debug!("Input Image Width: {input_width}");
    log::debug!("Input Image Height: {input_height}");

    //calculate the needed dimensions
    let (columns, rows, tile_width, tile_height) = ResizingDimension::calculate_dimensions(
        config.target_size,
        input_height,
        input_width,
        config.scale,
        config.border,
        config.dimension,
    );
    log::debug!("Columns: {columns}");
    log::debug!("Rows: {rows}");
    log::debug!("Tile Width: {tile_width}");
    log::debug!("Tile Height: {tile_height}");

    let mut input_img = image;

    if config.outline {
        //create an outline using an algorithm loosely based on the canny edge algorithm
        input_img = filter::edge_detection_filter(input_img, config.hysteresis);
    }

    if config.transform_x {
        log::info!("Flipping image horizontally");
        input_img = input_img.fliph();
    }

    if config.transform_y {
        log::info!("Flipping image vertically");
        input_img = input_img.flipv();
    }

    log::info!("Resizing image to fit new dimensions");
    //use the thumbnail method, since its way faster, it may result in artifacts, but the ascii art will be pixelate anyway
    let source_img = input_img.thumbnail_exact(columns * tile_width, rows * tile_height);

    log::debug!("Resized Image Width: {}", source_img.width());
    log::debug!("Resized Image Height: {}", source_img.height());

    //output string
    let mut output = String::with_capacity((tile_width * tile_height) as usize);
    log::trace!("Created output string");

    if config.target == TargetType::HtmlFile {
        log::trace!("Adding html top part");
        output.push_str(&target::html::html_top());
    }

    log::trace!("Calculating horizontal spacing");
    let horizontal_spacing = if config.center_x {
        spacing_horizontal(if config.border {
            //two columns are missing because the border takes up two lines
            columns + 2
        } else {
            columns
        })
    } else {
        String::with_capacity(0)
    };

    if config.center_y && config.target == TargetType::Shell {
        log::trace!("Adding vertical top spacing");
        output.push_str(&spacing_vertical(if config.border {
            //two rows are missing because the border takes up two lines
            rows + 2
        } else {
            rows
        }));
    }

    if config.border {
        //add spacing for centering
        if config.center_x {
            output.push_str(&horizontal_spacing);
        }

        //add top part of border before conversion
        log::trace!("Adding top part of border");
        output.push('╔');
        output.push_str(&"═".repeat(columns as usize));
        output.push_str("╗\n");
    }

    log::info!("Starting conversion to ascii");
    let width = source_img.width();

    //convert source img to a target string
    let target = source_img
        .pixels()
        .step_by(tile_width as usize)
        .filter(|(x, y, _)| y % tile_height == 0 && x % tile_width == 0)
        .map(|(x, y, _)| {
            //pre-allocate vector with the with space for all pixels in the tile
            let mut pixels = Vec::with_capacity((tile_height * tile_width) as usize);

            //get all pixel of the tile
            for p_x in 0..tile_width {
                for p_y in 0..tile_height {
                    pixels.push(unsafe { source_img.unsafe_get_pixel(x + p_x, y + p_y) })
                }
            }

            //convert pixels to a char/string
            let mut ascii_char = pixel::correlating_char(&pixels, config);

            //add border at the start
            //this cannot be done in single if-else, since the image might only be a single pixel wide
            if x == 0 {
                //add outer border (left)
                if config.border {
                    ascii_char.insert(0, '║');
                }

                //add spacing for centering the image
                if config.center_x {
                    ascii_char.insert_str(0, &horizontal_spacing);
                }
            }

            //add a break at line end
            if x == width - tile_width {
                //add outer border (right)
                if config.border {
                    ascii_char.push('║');
                }

                ascii_char.push('\n');
            }

            ascii_char
        })
        .collect::<String>();

    output.push_str(&target);

    if config.border {
        //add spacing for centering
        if config.center_x {
            output.push_str(&horizontal_spacing);
        }

        //add bottom part of border after conversion
        log::trace!("Adding bottom border");
        output.push('╚');
        output.push_str(&"═".repeat(columns as usize));
        output.push('╝');
    }

    //compare it, ignoring the enum value such as true, true
    if config.target == TargetType::HtmlFile {
        log::trace!("Adding html bottom part");
        output.push_str(&target::html::html_bottom());
    }

    if config.center_y && config.target == TargetType::Shell {
        log::trace!("Adding vertical bottom spacing");
        output.push_str(&spacing_vertical(if config.border {
            //two rows are missing because the border takes up two lines
            rows + 2
        } else {
            rows
        }));
    }

    output
}

/// Return a spacer string, which can be used to center the ascii image in the middle of the terminal.
///
/// When the terminal width is not existing, for example when the output is not a terminal, the returned string will be empty.
fn spacing_horizontal(width: u32) -> String {
    let term_width = terminal_size::terminal_size()
        .map(|dimensions| dimensions.0 .0 as u32)
        .unwrap_or_default();
    " ".repeat(term_width.saturating_sub(width).saturating_div(2) as usize)
}

/// Return a spacer string, which can be used to center the ascii image in the middle of the terminal.
///
/// When the terminal height is not existing, for example when the output is not a terminal, the returned string will be empty.
fn spacing_vertical(height: u32) -> String {
    let term_height = terminal_size::terminal_size()
        .map(|dimensions| dimensions.1 .0 as u32)
        .unwrap_or_default();
    log::trace!("H: {term_height}, h: {height}");
    "\n".repeat(term_height.saturating_sub(height).saturating_div(2) as usize)
}

/// Returns if the terminal supports truecolor mode.
///
/// It checks the `COLORTERM` environment variable,
/// if it is either set to
/// `truecolor` or `24bit` true is returned.
///
/// In all other cases false will be returned.
///
/// # Examples
/// ```
/// use artem::SUPPORTS_TRUECOLOR;
/// # use std::env;
///
/// # env::set_var("COLORTERM", "truecolor");
/// //only true when run in a shell that supports true color
/// let color_support = *SUPPORTS_TRUECOLOR;
/// assert!(color_support);
/// ```
pub static SUPPORTS_TRUECOLOR: LazyLock<bool> = LazyLock::new(|| {
    std::env::var("COLORTERM")
        .is_ok_and(|value| value.contains("truecolor") || value.contains("24bit"))
});
