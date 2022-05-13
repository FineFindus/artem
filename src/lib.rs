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

use crate::options::{Option, TargetType};

/// Takes an image and returns it as an ascii art string.
///
/// The result can be changed using th `options` argument
/// # Examples
/// ```no_run
/// use artem::options::OptionBuilder;
///
/// let img = image::open("examples/abraham_lincoln.jpg").unwrap();
/// let converted_image = artem::convert(img, OptionBuilder::new().build());
/// ```
///It uses the [`Option`] to set specific options on how to convert the image.
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

    if std::mem::discriminant(&options.target)
        == std::mem::discriminant(&TargetType::HtmlFile(true, true))
    {
        trace!("Adding html top part");
        output.push_str(&target::html::html_top());
    }

    if options.border {
        //add top part of border before conversion
        output.push('╔');
        output.push_str("═".repeat(columns as usize).as_str());
        output.push_str("╗\n");
        trace!("Adding top part of border");
    }

    info!("Starting conversion to ascii");

    //convert source img to a target string
    let target = source_img
        .pixels()
        .into_iter()
        .step_by(tile_width as usize)
        .filter_map(|(x, y, _)| {
            if y % tile_height == 0 && x % tile_width == 0 {
                //preallocate vector with the with space for all pixels in the tile
                let mut pixels = Vec::with_capacity((tile_height * tile_width) as usize);

                //get all pixel of the tile
                for p_x in 0..tile_width {
                    for p_y in 0..tile_height {
                        pixels.push(source_img.get_pixel(x + p_x, y + p_y))
                    }
                }

                //convert pixels to a char/string
                let mut char = pixel::correlating_char(
                    &pixels,
                    options.density.as_str(),
                    options.invert,
                    options.target,
                );

                //add border at the start
                //this cannot be done in single if-else, since the image might only be a single pixel wide
                if x == 0 {
                    //add outer border (left)
                    if options.border {
                        char = format!("{}{}", "║", char);
                    }
                }

                //add a break at line end
                if x == source_img.width() - tile_width {
                    //add outer border (right)
                    if options.border {
                        char.push('║');
                    }

                    char.push('\n');
                }

                Some(char)
            } else {
                //only read tiles
                None
            }
        })
        .reduce(|acc, value| acc + value.as_str());

    match target {
        Some(value) => output.push_str(&value),
        //this none case should never appear
        None => {
            util::fatal_error("Failed to convert image.", Some(70));
        }
    };

    if options.border {
        //add bottom part of border after conversion
        output.push('╚');
        output.push_str("═".repeat((columns) as usize).as_str());
        output.push('╝');
    } else {
        //last char is a new line char, remove it
        //don't use trim, since it can remove "whitespace" which include spaces
        //these might be used to represent part of the image
        output.remove(output.len() - 1);
    }

    //compare it, ignoring the enum value such as true, true
    if std::mem::discriminant(&options.target)
        == std::mem::discriminant(&TargetType::HtmlFile(true, true))
    {
        trace!("Adding html bottom part");
        output.push_str(&target::html::html_bottom());
    }

    //return output
    output
}
