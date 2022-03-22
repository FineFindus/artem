use std::{sync::Arc, thread};

use image::{DynamicImage, GenericImageView, Rgba};
use log::{debug, info, trace};

use crate::{
    conversion_options::{ConversionOption, ConversionTargetType},
    pixel::get_pixel_density,
    util,
};

///Returns the given image as ascii representation string.
///
/// # Examples
/// ```
/// let img = image::open(img_path).unwrap();
/// let converted_image = convert_img(img, ConversionOptionBuilder::new().build());
/// ```
///It uses the [ConversionOption] to set specific options on how to convert the image.
pub fn convert_img(img: DynamicImage, options: ConversionOption) -> String {
    //TODO print color support
    // debug!("Using Color: {}", options.color);
    // debug!("Using colored background: {}", options.background_color);
    debug!("Using inverted color: {}", options.invert);
    //get img dimensions
    let input_width = img.width();
    let input_height = img.height();
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

    info!("Resizing image to fit new dimensions");
    //use the thumbnail method, since its way faster, it may result in artifacts, but the ascii art will be pixelate anyway
    let img = Arc::new(img.thumbnail_exact(columns * tile_width, rows * tile_height));

    debug!("Resized Image Width: {}", img.width());
    debug!("Resized Image Height: {}", img.height());

    //output string
    let mut output = String::new();
    trace!("Created output string");

    if std::mem::discriminant(&options.target)
        == std::mem::discriminant(&ConversionTargetType::HtmlFile(true, true))
    {
        trace!("Adding html top part");
        output.push_str(&push_html_top());
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

            //check so that only pixels in the image are accessed
            let chunk_end = if rows > (chunk + 1) * thread_tiles {
                (chunk + 1) * thread_tiles
            } else {
                rows
            };

            //go through the thread img chunk
            for row in util::range(chunk * thread_tiles, chunk_end, options.transform_y) {
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
                    let char = get_pixel_density(
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
        trace!("Adding bottom part of border");
    }

    if std::mem::discriminant(&options.target)
        == std::mem::discriminant(&ConversionTargetType::HtmlFile(true, true))
    {
        trace!("Adding html bottom part");
        output.push_str(&push_html_bottom());
    }

    output.trim_end().to_string()
}

///Returns the top part of the output html file.
///
/// This contains the html elements needed for a correct html file.
/// The title will be set to `Artem Ascii Image`, whilst the will be set to `Courier` ( a monospace font)
/// It will also have the pre tag for correct spacing/line breaking
///
/// # Examples
/// ```
/// let string = String::new();
/// string.push_str(&push_html_top())
/// ```
fn push_html_top() -> String {
    r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Artem Ascii Image</title>
        <style>* {font-family: Courier;}</style>
    </head>
    
    <body>
        <pre>"#
        .to_string()
}

#[cfg(test)]
mod test_push_html_top {
    use super::*;
    #[test]
    fn push_top_html_returns_correct_string() {
        assert_eq!(
            r#"<!DOCTYPE html>
    <html lang="en">
    
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Artem Ascii Image</title>
        <style>* {font-family: Courier;}</style>
    </head>
    
    <body>
        <pre>"#,
            push_html_top()
        )
    }
}

///Returns the bottom part of the output html file.
///
/// The matching closing tags fro [push_html_top]. It will close
/// the pres, body and html tag.
///
/// # Examples
/// ```
/// let string = String::new();
/// string.push_str(&push_html_top())
/// string.push_str(&push_html_bottom())
/// ```
fn push_html_bottom() -> String {
    "</pre></body></html>".to_string()
}

#[cfg(test)]
mod test_push_html_bottom {
    use super::*;

    #[test]
    fn push_bottom_html_returns_correct_string() {
        assert_eq!("</pre></body></html>", push_html_bottom())
    }
}
