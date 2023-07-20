//! # artem
//! artem is a small cli program written in rust to easily convert images to ascii art.
//! It uses the image-rs library to read images from different image formats, such as png, jpeg, etc.
//!
//! This file contains a library, which is used under the hood. This enabled benchmarking
//! using criterion.rs, since it requires an lib to operate.
//!
//! It is not supported to use this library without the command-line interface.
//!
//! # Example usage of cli
//! ```bash
//! artem examples/abraham_lincoln.jpg
//! ```

//import utilities, such as dimensions, value remapping, etc
pub mod util;

//condense all arguments into a single struct
pub mod options;

//functions for working with pixels
mod pixel;

//outlining filter
mod filter;
//functions for dealing with output targets/files
mod target;

use image::{DynamicImage, GenericImageView};
use log::{debug, info, trace};

pub use crate::options::OptionBuilder;
use crate::options::{Option, TargetType};

/// Takes an image and returns it as an ascii art string.
///
/// The result can be changed using the [`crate::options::Option`] argument
/// # Examples
/// ```no_run
/// use artem::options::OptionBuilder;
///
/// let img = image::open("examples/abraham_lincoln.jpg").unwrap();
/// let converted_image = artem::convert(img, OptionBuilder::new().build());
/// ```
pub fn convert(image: DynamicImage, options: Option) -> String {
    debug!("Using inverted color: {}", options.invert);
    //get img dimensions
    let input_width = image.width();
    let input_height = image.height();
    debug!("Input Image Width: {input_width}");
    debug!("Input Image Height: {input_height}");

    //calculate the needed dimensions
    let (columns, rows, tile_width, tile_height) = util::calculate_dimensions(
        options.target_size,
        input_height,
        input_width,
        options.scale,
        options.border,
        options.dimension,
    );
    debug!("Columns: {columns}");
    debug!("Rows: {rows}");
    debug!("Tile Width: {tile_width}");
    debug!("Tile Height: {tile_height}");

    let mut input_img = image;

    if options.outline {
        //create an outline using an algorithm loosely based on the canny edge algorithm
        input_img = filter::edge_detection_filter(input_img, options.hysteresis);
    }

    if options.transform_x {
        info!("Flipping image horizontally");
        input_img = input_img.fliph();
    }

    if options.transform_y {
        info!("Flipping image vertically");
        input_img = input_img.flipv();
    }

    info!("Resizing image to fit new dimensions");
    //use the thumbnail method, since its way faster, it may result in artifacts, but the ascii art will be pixelate anyway
    let source_img = input_img.thumbnail_exact(columns * tile_width, rows * tile_height);

    debug!("Resized Image Width: {}", source_img.width());
    debug!("Resized Image Height: {}", source_img.height());

    //output string
    let mut output = String::with_capacity((tile_width * tile_height) as usize);
    trace!("Created output string");

    if matches!(&options.target, &TargetType::HtmlFile(true, true)) {
        trace!("Adding html top part");
        output.push_str(&target::html::html_top());
    }

    trace!("Calculating horizontal spacing");
    let horizontal_spacing = if options.center_x {
        util::spacing_horizontal(if options.border {
            //two columns are missing because the border takes up two lines
            columns + 2
        } else {
            columns
        })
    } else {
        String::with_capacity(0)
    };

    if options.center_y && matches!(&options.target, &TargetType::Shell(true, true)) {
        trace!("Adding vertical top spacing");
        output.push_str(&util::spacing_vertical(if options.border {
            //two rows are missing because the border takes up two lines
            rows + 2
        } else {
            rows
        }));
    }

    if options.border {
        //add spacing for centering
        if options.center_x {
            output.push_str(&horizontal_spacing);
        }

        //add top part of border before conversion
        trace!("Adding top part of border");
        output.push('╔');
        output.push_str(&"═".repeat(columns as usize));
        output.push_str("╗\n");
    }

    info!("Starting conversion to ascii");
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
            let mut ascii_char = pixel::correlating_char(
                &pixels,
                &options.characters,
                options.invert,
                options.target,
            );

            //add border at the start
            //this cannot be done in single if-else, since the image might only be a single pixel wide
            if x == 0 {
                //add outer border (left)
                if options.border {
                    ascii_char.insert(0, '║');
                }

                //add spacing for centering the image
                if options.center_x {
                    ascii_char.insert_str(0, &horizontal_spacing);
                }
            }

            //add a break at line end
            if x == width - tile_width {
                //add outer border (right)
                if options.border {
                    ascii_char.push('║');
                }

                ascii_char.push('\n');
            }

            ascii_char
        })
        .collect::<String>();

    output.push_str(&target);

    if options.border {
        //add spacing for centering
        if options.center_x {
            output.push_str(&horizontal_spacing);
        }

        //add bottom part of border after conversion
        trace!("Adding bottom border");
        output.push('╚');
        output.push_str(&"═".repeat(columns as usize));
        output.push('╝');
    }

    //compare it, ignoring the enum value such as true, true
    if matches!(&options.target, &TargetType::HtmlFile(true, true)) {
        trace!("Adding html bottom part");
        output.push_str(&target::html::html_bottom());
    }

    if options.center_y && matches!(&options.target, &TargetType::Shell(true, true)) {
        trace!("Adding vertical bottom spacing");
        output.push_str(&util::spacing_vertical(if options.border {
            //two rows are missing because the border takes up two lines
            rows + 2
        } else {
            rows
        }));
    }

    //return output
    output
}
