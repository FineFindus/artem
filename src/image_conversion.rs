use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, RgbImage, Rgba};
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

    let mut input_img = img;

    if options.outline {
        input_img = blur(input_img, 6.4f64, options.threads);
        input_img = edge_detection(input_img, options.threads);
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

///Blur the given image using an gaussian blur, based on the given sigma.
///
/// The given `thread_count` increases the amount of threads used. It should be greater than one, but lower than the height
/// of the image. If that is not the case, the `thread_count` will be changed to fit the requirements.
///
/// This returns a new (blurred) image.
///
/// # Examples
/// ```
/// let blurred = blur(image, 1.4f64, 4)
/// ```
fn blur(img: DynamicImage, sigma: f64, thread_count: u32) -> DynamicImage {
    info!("Blurring image");
    //measure timing for this step
    let now = Instant::now();

    let kernel = create_gauss_kernel(sigma);

    let offset = (kernel.len() / 2) as u32;

    let (width, height) = img.dimensions();

    //clamp threads to be non-zero and not more than the img
    let thread_count = thread_count.clamp(1, height);

    //create empty target img
    let destination_img: Arc<Mutex<RgbImage>> =
        Arc::new(Mutex::new(ImageBuffer::new(img.width(), img.height())));

    let img = Arc::new(img);

    let thread_chunk = height / thread_count;

    //collect threads handles
    let mut handles = Vec::with_capacity(thread_count as usize);
    for chunk in 0..thread_count {
        //copy img from arv
        let thread_img = img.clone();
        let thread_dest_img = Arc::clone(&destination_img);

        //check so that only pixels in the image are accessed
        let chunk_end = if height > (chunk + 1) * thread_chunk {
            (chunk + 1) * thread_chunk
        } else {
            height
        };

        let handle = thread::spawn(move || {
            for y in (chunk * thread_chunk)..chunk_end {
                for x in 0..width {
                    //kernel values for rgb
                    let mut kernel_values_r = 0f64;
                    let mut kernel_values_g = 0f64;
                    let mut kernel_values_b = 0f64;

                    //iterate through the kernel for this pixel
                    for k_x in 0..kernel.len() {
                        for k_y in 0..kernel.len() {
                            //get pixel pos for kernel
                            let pixel_pos_x =
                                (x + k_x as u32).saturating_sub(offset).clamp(0, width - 1);
                            let pixel_pos_y =
                                (y + k_y as u32).saturating_sub(offset).clamp(0, height - 1);

                            //check if pixel is in img, since the kernel will overlap to outside pixels, if not ignored it
                            if thread_img.in_bounds(pixel_pos_x, pixel_pos_y) {
                                //get the current pixel
                                let pixel = thread_img.get_pixel(pixel_pos_x, pixel_pos_y);
                                //add rgb values
                                kernel_values_r += pixel.0[0] as f64 * kernel[k_x][k_y];
                                kernel_values_g += pixel.0[1] as f64 * kernel[k_x][k_y];
                                kernel_values_b += pixel.0[2] as f64 * kernel[k_x][k_y];
                            }
                        }
                    }

                    //add filtered pixel to new img
                    let mut unlocked = thread_dest_img.lock().unwrap();
                    unlocked.put_pixel(
                        x,
                        y,
                        image::Rgb([
                            (kernel_values_r as u8), // .saturating_mul(10)
                            (kernel_values_g as u8), // .saturating_mul(10)
                            (kernel_values_b as u8), // .saturating_mul(10)
                        ]),
                    );
                }
            }
            true
        });
        handles.push(handle);
    }
    for handle in handles {
        match handle.join() {
            Ok(true) => {}
            _ => util::fatal_error("Error encountered when blurring image", Some(1)),
        };
    }

    match Arc::try_unwrap(destination_img) {
        Ok(value) => match value.into_inner() {
            Ok(value) => {
                info!(
                    "Successfully outlined image in {:3} ms",
                    now.elapsed().as_millis()
                );
                DynamicImage::ImageRgb8(value)
            }
            _ => util::fatal_error("Error encountered when blurring image", Some(1)),
        },
        Err(_) => util::fatal_error("Error encountered when blurring image", Some(1)),
    }
}

#[cfg(test)]
mod test_blur {
    use super::*;

    #[test]
    #[should_panic]
    fn panic_sigma_0() {
        //create black image
        let img = DynamicImage::ImageRgb8(ImageBuffer::new(3, 3));
        blur(img, 0f64, 4);
    }

    #[test]
    #[should_panic]
    fn panic_sigma_negative() {
        let img = DynamicImage::ImageRgb8(ImageBuffer::new(3, 3));
        blur(img, -1f64, 4);
    }

    #[test]
    fn black_img_remains_black() {
        let img = DynamicImage::ImageRgb8(ImageBuffer::new(3, 3));
        let blur = blur(img.clone(), 1.4f64, 4);
        assert_eq!(img, blur);
    }
    #[test]
    fn img_middle_white() {
        //
        // █
        //
        let img = DynamicImage::ImageRgb8(ImageBuffer::from_fn(3, 3, |x, y| {
            if y == 1 && x == 1 {
                image::Rgb([255, 255, 255])
            } else {
                image::Rgb([0, 0, 0])
            }
        }));
        let blur = blur(img.clone(), 1.4f64, 4);
        assert_ne!(img, blur);
        let result = DynamicImage::ImageRgb8(ImageBuffer::from_fn(3, 3, |x, y| {
            if y == 1 && x == 1 {
                image::Rgb([39, 39, 39])
            } else if y != 1 && x != 1 {
                image::Rgb([23, 23, 23])
            } else {
                image::Rgb([30, 30, 30])
            }
        }));
        assert_eq!(result, blur);
    }
}

///Creates a gaussian kernel based on the given sigma.
///
/// This is based on the c++ implementation on <https://www.geeksforgeeks.org/gaussian-filter-generation-c/>
///
/// # Panics
/// This will panic if the given `sigma` is smaller or equal to zero.
/// # Examples
/// ```
/// let kernel = create_gauss_kernel(1.4f64);
/// ```
fn create_gauss_kernel(sigma: f64) -> [[f64; 3]; 3] {
    if sigma <= 0f64 {
        panic!("The given sigma {} was smaller or equal to zero", sigma)
    }
    let mut kernel = [[0f64; 3]; 3];

    let mut r = 2f64 * sigma * sigma;
    let s = r;

    let mut sum = 0f64;

    for x in -1..=1isize {
        for y in -1..=1isize {
            r = ((x * x + y * y) as f64).sqrt();
            let value = (f64::exp(-(r * r) / s)) / (PI * s);
            kernel[(x + 1) as usize][(y + 1) as usize] = value;
            sum += value;
        }
    }

    for row in kernel.iter_mut() {
        for value in row.iter_mut() {
            *value /= sum;
        }
    }

    kernel
}

#[cfg(test)]
mod test_create_gauss_kernel {
    use super::*;

    #[test]
    #[should_panic]
    fn sigma_zero_panics() {
        create_gauss_kernel(0f64);
    }

    #[test]
    #[should_panic]
    fn sigma_minus_one_panics() {
        create_gauss_kernel(-1f64);
    }

    #[test]
    fn sigma_1_4() {
        assert_eq!(
            [
                [
                    0.09235312168033234,
                    0.11919032075339754,
                    0.09235312168033234
                ],
                [0.11919032075339754, 0.1538262302650804, 0.11919032075339754],
                [
                    0.09235312168033234,
                    0.11919032075339754,
                    0.09235312168033234
                ]
            ],
            create_gauss_kernel(1.4f64)
        )
    }
}

/// Detect edges in an image by using the sobel operators.
///
/// The given `thread_count` increases the amount of threads used. It should be greater than one, but lower than the height
/// of the image. If that is not the case, the `thread_count` will be changed to fit the requirements.
///
/// This returns a new, grayscale image with only the edges in white visible.
///
/// # Examples
/// ```
/// let outline = edge_detection(image, 4)
/// ```
fn edge_detection(img: DynamicImage, thread_count: u32) -> DynamicImage {
    info!("Creating outline image");
    //create stop watch
    let now = Instant::now();
    //sobel kernels
    let kernel_x = &[
        [1f64, 2f64, 1f64],
        [0f64, 0f64, 0f64],
        [-1f64, -2f64, -1f64],
    ];

    let kernel_y = &[
        [1f64, 0f64, -1f64],
        [2f64, 0f64, -2f64],
        [1f64, 0f64, -1f64],
    ];

    let kernel_length = kernel_x.len();

    let offset = (kernel_length / 2) as u32;

    let (width, height) = img.dimensions();

    //clamp threads to be non-zero and not more than the img
    let thread_count = thread_count.clamp(1, height);

    //create empty target img
    let destination_img: Arc<Mutex<GrayImage>> =
        Arc::new(Mutex::new(ImageBuffer::new(img.width(), img.height())));

    let img = Arc::new(img);

    let thread_chunk = height / thread_count;

    //collect threads handles
    let mut handles = Vec::with_capacity(thread_count as usize);

    for chunk in 0..thread_count {
        //copy img from arv
        let thread_img = img.clone();
        let thread_dest_img = Arc::clone(&destination_img);

        //check so that only pixels in the image are accessed
        let chunk_end = if height > (chunk + 1) * thread_chunk {
            (chunk + 1) * thread_chunk
        } else {
            height
        };

        let handle = thread::spawn(move || {
            for y in (chunk * thread_chunk)..chunk_end {
                for x in 0..width {
                    //kernel values for rgb
                    let mut kernel_values_x = 0f64;
                    let mut kernel_values_y = 0f64;

                    //iterate through the kernel for this pixel
                    for k_y in 0..kernel_length {
                        for k_x in 0..kernel_length {
                            //get pixel pos for kernel
                            let pixel_pos_x =
                                (x + k_x as u32).saturating_sub(offset).clamp(0, width - 1);
                            let pixel_pos_y =
                                (y + k_y as u32).saturating_sub(offset).clamp(0, height - 1);

                            //get the current pixel, it will always be inside, since of the previous clamping
                            let pixel = thread_img.get_pixel(pixel_pos_x, pixel_pos_y);
                            let pixel_gray = crate::pixel::get_luminosity(
                                pixel.0[0] as f64,
                                pixel.0[1] as f64,
                                pixel.0[2] as f64,
                            );

                            //add rgb values
                            kernel_values_x += pixel_gray as f64 * kernel_x[k_x][k_y];
                            kernel_values_y += pixel_gray as f64 * kernel_y[k_x][k_y];
                        }
                    }
                    //add filtered pixel to new img
                    let mut unlocked = thread_dest_img.lock().unwrap();
                    unlocked.put_pixel(
                        x,
                        y,
                        image::Luma([((kernel_values_x * kernel_values_x
                            + kernel_values_y * kernel_values_y)
                            .sqrt()
                            .round() as u8)
                            .saturating_mul(3)]),
                    );
                }
            }
            true
        });
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(true) => {}
            _ => util::fatal_error("Error encountered when outlining image", Some(1)),
        };
    }

    match Arc::try_unwrap(destination_img) {
        Ok(value) => match value.into_inner() {
            Ok(value) => {
                info!(
                    "Successfully outlined image in {:3} ms",
                    now.elapsed().as_millis()
                );
                DynamicImage::ImageLuma8(value)
            }
            _ => util::fatal_error("Error encountered when outlining image", Some(1)),
        },
        Err(_) => util::fatal_error("Error encountered when outlining image", Some(1)),
    }
}

#[cfg(test)]
mod test_edge_detection {
    use super::*;

    #[test]
    fn no_edge() {
        //create empty image with no edge
        let img = DynamicImage::ImageLuma8(ImageBuffer::new(3, 3));
        let edge_img = edge_detection(img.clone(), 4);
        assert_eq!(img, edge_img);
    }

    #[test]
    fn edge_vertical() {
        //create empty image with vertical edge
        //█ █
        //█ █
        //█ █
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, _| {
            if x == 1 {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        }));
        let edge_img = edge_detection(img.clone(), 4);
        assert_eq!(img, edge_img);
    }

    #[test]
    fn edge_horizontal() {
        //create empty image with horizontal edge
        //███
        //
        //███
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |_, y| {
            if y == 1 {
                image::Luma([0u8])
            } else {
                image::Luma([255u8])
            }
        }));
        let edge_img = edge_detection(img.clone(), 4);
        assert_eq!(img, edge_img);
    }
}
