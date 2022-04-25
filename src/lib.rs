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

use std::{sync::Arc, thread};

use image::{DynamicImage, GenericImageView, Rgba};
use log::{debug, info, trace};

use crate::{
    options::{Option, TargetType},
    pixel::correlating_char,
};

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
        input_img = filter::edge_detection_filter(input_img, options.threads, options.hysteresis);
    }

    info!("Resizing image to fit new dimensions");
    //use the thumbnail method, since its way faster, it may result in artifacts, but the ascii art will be pixelate anyway
    let img = Arc::new(input_img.thumbnail_exact(columns * tile_width, rows * tile_height));

    debug!("Resized Image Width: {}", img.width());
    debug!("Resized Image Height: {}", img.height());

    //output string
    let mut output = String::new();
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

    //clamp threads
    let thread_count = options.threads.clamp(
        1,    //there has to be at least 1 thread to convert the img
        rows, //there should no be more threads than rows
    );
    debug!("Threads: {thread_count}");

    //split the img into tile for each thread
    let thread_tiles = (rows as f64 / thread_count as f64).ceil() as u32;
    debug!("Thread Tile Height: {thread_tiles}");
    //collect threads handles
    let mut handles = Vec::with_capacity(thread_count as usize);
    trace!("Allocated thread handles");

    //split the img into chunks for each thread
    for chunk in util::range(0, thread_count, options.transform_y) {
        //arc clone img and density
        let thread_img = Arc::clone(&img);
        let thread_density = options.density.to_owned();

        //create a thread for this img chunk
        trace!("Creating thread: {chunk}");
        let handle = thread::spawn(move || {
            //create thread string
            let mut thread_output = String::new();

            //create a pixel block from multiple pixels
            //preallocate vector with the correct size, since all tiles should be the same size,
            //this vector can be reused for all tiles in a thread
            let mut pixel_block: Vec<Rgba<u8>> =
                Vec::with_capacity((tile_height * tile_width) as usize);

            //go through the thread img chunk
            for row in util::range(
                (chunk * thread_tiles).clamp(0, rows), //after max rows, no more pixels exist
                // chunk_end,
                ((chunk + 1) * thread_tiles).clamp(thread_tiles, rows),
                options.transform_y,
            ) {
                if options.border {
                    //add bottom part before image
                    thread_output.push('║');
                }

                for col in util::range(0, columns, options.transform_x) {
                    //get a single tile
                    let tile_row = row * tile_height;
                    let tile_col = col * tile_width;

                    //go through each pixel in the tile
                    for y in tile_row..(tile_row + tile_height) {
                        for x in tile_col..(tile_col + tile_width) {
                            //add pixel to block
                            pixel_block.push(thread_img.get_pixel(x, y));
                        }
                    }

                    //get and display density char, it returns a normal and a colored string
                    let char = correlating_char(
                        &pixel_block,
                        &thread_density,
                        options.invert,
                        options.target,
                    );

                    //clear the vec for the next iteration
                    pixel_block.clear();
                    //append the char for the output
                    thread_output.push_str(char.as_str());
                }

                if options.border {
                    //add bottom part after image
                    thread_output.push('║');
                }

                //add new line
                thread_output.push('\n');
            }
            trace!("Thread {chunk} finished");
            thread_output
        });
        trace!("Appending handle of thread {chunk}");
        handles.push(handle);
    }

    for handle in handles {
        //get thread result
        let result = match handle.join() {
            Ok(string) => string,
            Err(_) => util::fatal_error("Error encountered when converting image", Some(1)),
        };
        //add output together
        trace!("Appending output of thread");
        output.push_str(result.as_str());
    }

    if options.border {
        //add bottom part of border after conversion
        output.push('╚');
        output.push_str("═".repeat((columns) as usize).as_str());
        output.push('╝');
    }

    //compare it, ignoring the enum value such as true, true
    if std::mem::discriminant(&options.target)
        == std::mem::discriminant(&TargetType::HtmlFile(true, true))
    {
        trace!("Adding html bottom part");
        output.push_str(&target::html::html_bottom());
    }

    output.trim_end().to_string()
}
