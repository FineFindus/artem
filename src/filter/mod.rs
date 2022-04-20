use std::{
    sync::{Arc, Mutex},
    thread,
    time::Instant,
};

use image::{DynamicImage, GenericImageView, GrayImage, ImageBuffer, RgbImage};
use log::{debug, info, trace};

use crate::util;

/// Filter an image using a technic similar to canny edge detection.
///
/// The image will first be blurred and then as grayscale converted using the sobel operators.
/// It will not use non-maximum suppression, since this would make the lines thinner, which is something that is not actually wanted,
/// since the ascii image will look much better if it is based off thicker lines.
///
/// When `hysteresis` is set to true, it will additionally use the hysteresis method to improve the outlines of the image.
/// This will result in thinner lines, with less imperfections, but at the cost of less good looking ascii chars,
/// since it will mostly consist of dots.
///
/// # Example
/// ```compile_fail, compile will fail, this is an internal example
///  let outlined_image = edge_detection_filter(img, 4);
/// ```
pub fn edge_detection_filter(
    img: DynamicImage,
    thread_count: u32,
    hysteresis: bool,
) -> DynamicImage {
    //blur
    let blurred_img = blur(img, 6.4f64, thread_count);
    //apply sobel
    let sobel_img = apply_sobel_kernel(blurred_img, thread_count);
    //double threshold and hysteresis
    if hysteresis {
        edge_tracking(sobel_img)
    } else {
        sobel_img
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
/// ```compile_fail, compile will fail, this is an internal example
/// let blurred = blur(image, 1.4f64, 4)
/// ```
fn blur(img: DynamicImage, sigma: f64, thread_count: u32) -> DynamicImage {
    info!("Blurring image");
    //measure timing for this step
    trace!("Started time tracking for blurring");
    let now = Instant::now();

    debug!("Creating gauss kernel");
    let kernel = create_gauss_kernel(sigma);

    let offset = (kernel.len() / 2) as u32;

    let (width, height) = img.dimensions();

    //clamp threads to be non-zero and not more than the img
    let thread_count = thread_count.clamp(1, height);
    debug!("Using {} threads for blurring", thread_count);

    //create empty target img
    debug!("Creating target blur image");
    let destination_img: Arc<Mutex<RgbImage>> =
        Arc::new(Mutex::new(ImageBuffer::new(img.width(), img.height())));

    //create arc wrapper around img
    let img = Arc::new(img);

    let thread_chunk = height / thread_count;
    trace!("Blur Thread Chunks: {}", thread_chunk);

    //collect threads handles
    let mut handles = Vec::with_capacity(thread_count as usize);
    trace!("Allocated blur thread handles");

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
                    // for k_y in 0..kernel_len {
                    for (k_y, row) in kernel.iter().enumerate() {
                        for (k_x, kernel_value) in row.iter().enumerate() {
                            // for k_x in 0..kernel_len {
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
                                kernel_values_r += pixel.0[0] as f64 * kernel_value;
                                kernel_values_g += pixel.0[1] as f64 * kernel_value;
                                kernel_values_b += pixel.0[2] as f64 * kernel_value;
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
                    "Successfully blurred image in {:3} ms",
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
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
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
            let value = (f64::exp(-(r * r) / s)) / (std::f64::consts::PI * s);
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
/// ```compile_fail, compile will fail, this is an internal example
/// let outline = edge_detection(image, 4)
/// ```
fn apply_sobel_kernel(img: DynamicImage, thread_count: u32) -> DynamicImage {
    info!("Creating outline image");
    //create stop watch
    trace!("Started time tracking for sobel");
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
    debug!("Using {} threads for sobel", thread_count);

    //create empty target img
    debug!("Creating target sobel image");
    let destination_img: Arc<Mutex<GrayImage>> =
        Arc::new(Mutex::new(ImageBuffer::new(img.width(), img.height())));

    let img = Arc::new(img);

    let thread_chunk = height / thread_count;
    trace!("Sobel Thread Chunks: {}", thread_chunk);

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
                            let pixel_gray =
                                crate::pixel::luminosity(pixel.0[0], pixel.0[1], pixel.0[2]);

                            //add rgb values
                            kernel_values_x += pixel_gray as f64 * kernel_x[k_x][k_y];
                            kernel_values_y += pixel_gray as f64 * kernel_y[k_x][k_y];
                        }
                    }
                    //usually in the canny edge detection algorithm, a non-maximum suppression would now be performed,
                    //to have thinner lines. In this case this is not needed, since thicker lines will produce a more clearly ascii like image.

                    let raw_kernel_value = ((kernel_values_x * kernel_values_x
                        + kernel_values_y * kernel_values_y)
                        .sqrt())
                    .round() as u8;

                    //add filtered pixel to new img
                    let mut unlocked = thread_dest_img.lock().unwrap();
                    unlocked.put_pixel(x, y, image::Luma([raw_kernel_value.saturating_mul(3)]));
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
mod test_sobel {
    use super::*;

    #[test]
    fn no_edge() {
        //create empty image with no edge
        let img = DynamicImage::ImageLuma8(ImageBuffer::new(3, 3));
        let edge_img = apply_sobel_kernel(img.clone(), 4);
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
        let edge_img = apply_sobel_kernel(img.clone(), 4);
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
        let edge_img = apply_sobel_kernel(img.clone(), 4);
        assert_eq!(img, edge_img);
    }
}

/// Apply double threshold and hysteresis to the image to remove small imperfections and complete
/// the edges.
///
/// The resulting image will have better edge detection, at the cost of a less good looking ascii image.
/// This method uses two thresholds to determine between strong, weak and irrelevant pixels.weak ones can be converted into strong ones,
/// if at least one neighboring pixel is strong. Every non-strong pixel will be removed.
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// let hysteresis_img = edge_tracking(img);
/// ```
fn edge_tracking(img: DynamicImage) -> DynamicImage {
    //start tracking to for this step
    trace!("Started time tracking for hysteresis");
    let now = Instant::now();

    debug!("Creating target hysteresis image");
    let mut target_img: GrayImage = ImageBuffer::new(img.width(), img.height());

    let upper_threshold = u8::MAX as f64 * 0.5;
    debug!("Upper threshold: {}", upper_threshold);
    let lower_threshold = u8::MAX as f64 * 0.3;
    debug!("Lower threshold: {}", lower_threshold);

    for y in 0..img.height() {
        for x in 0..img.width() {
            let target_pixel = img.get_pixel(x, y);
            let grayscale_pixel =
                crate::pixel::luminosity(target_pixel.0[0], target_pixel.0[1], target_pixel.0[2]);

            //check if pixel is at least weak or strong
            if grayscale_pixel >= upper_threshold {
                //pixel is already strong, set to completely white and continue continue loop
                target_img.put_pixel(x, y, image::Luma([255]));
                continue;
            } else if grayscale_pixel >= lower_threshold {
                //check if an adjacent pixel is strong
                let mut strong = false;

                'outer: for k_y in 0..3 {
                    for k_x in 0..3 {
                        //get pixel pos for kernel
                        let pixel_pos_x =
                            (x + k_x as u32).saturating_sub(1).clamp(0, img.width() - 1);
                        let pixel_pos_y = (y + k_y as u32)
                            .saturating_sub(1)
                            .clamp(0, img.height() - 1);

                        //get the adjacent pixel to target pixel, it will always be inside, since of the previous clamping
                        let pixel = img.get_pixel(pixel_pos_x, pixel_pos_y);
                        let pixel_gray =
                            crate::pixel::luminosity(pixel.0[0], pixel.0[1], pixel.0[2]);

                        if pixel_gray >= upper_threshold {
                            //adjacent pixel is strong, so target pixel should be strong as well
                            strong = true;
                            //no need to check for an second pixel, stop outer loop
                            break 'outer;
                        }
                    }
                }
                if strong {
                    //pixel has strong adjacent ones, make strong as well
                    target_img.put_pixel(x, y, image::Luma([255]))
                } else {
                    //no strong pixels around, pixel is irrelevant, remove
                    target_img.put_pixel(x, y, image::Luma([0]))
                }
            } else {
                //pixel is irrelevant, remove
                target_img.put_pixel(x, y, image::Luma([0]))
            }
        }
    }

    info!(
        "Successfully applied hysteresis to target image in {:3} ms",
        now.elapsed().as_millis()
    );
    DynamicImage::ImageLuma8(target_img)
}

#[cfg(test)]
mod test_edge_detection {
    use super::*;

    #[test]
    fn no_strong_results_in_black_img() {
        let img = DynamicImage::ImageLuma8(ImageBuffer::new(3, 3));
        let result = edge_tracking(img.clone());
        assert_eq!(img, result);
    }

    #[test]
    fn strong_pixels_stay() {
        //there is method to use raw pixel data, but it is not good enough documented, so I couldn't figure out how to use it
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([255u8])
            } else {
                image::Luma([0u8])
            }
        }));
        let result = edge_tracking(img.clone());
        assert_eq!(img, result);
    }

    #[test]
    fn weak_pixel_removed() {
        //there is method to use raw pixel data, but it is not good enough documented, so I couldn't figure out how to use it
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([126u8])
            } else {
                image::Luma([0u8])
            }
        }));
        let result = edge_tracking(img.clone());
        //result is equal to a black image
        assert_eq!(DynamicImage::ImageLuma8(ImageBuffer::new(3, 3)), result);
    }

    #[test]
    fn irrelevant_pixel_removed() {
        //there is method to use raw pixel data, but it is not good enough documented, so I couldn't figure out how to use it
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([76u8])
            } else {
                image::Luma([0u8])
            }
        }));
        let result = edge_tracking(img.clone());
        //result is equal to a black image
        assert_eq!(DynamicImage::ImageLuma8(ImageBuffer::new(3, 3)), result);
    }

    #[test]
    fn weak_pixel_with_strong_neighbor_is_converted() {
        //there is method to use raw pixel data, but it is not good enough documented, so I couldn't figure out how to use it
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([126u8])
            } else if x == 2 && y == 1 {
                image::Luma([255u8])
            } else {
                image::Luma([0u8])
            }
        }));

        let desired_result = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([255u8])
            } else if x == 2 && y == 1 {
                image::Luma([255u8])
            } else {
                image::Luma([0u8])
            }
        }));
        let result = edge_tracking(img.clone());
        //result is equal to a black image
        assert_eq!(desired_result, result);
    }
    #[test]
    fn irrelevant_pixel_with_strong_neighbor_is_removed() {
        //there is method to use raw pixel data, but it is not good enough documented, so I couldn't figure out how to use it
        let img = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([255u8])
            } else if x == 2 && y == 1 {
                image::Luma([40u8])
            } else {
                image::Luma([0u8])
            }
        }));

        let desired_result = DynamicImage::ImageLuma8(ImageBuffer::from_fn(3, 3, |x, y| {
            if x == 1 && y == 1 {
                image::Luma([255u8])
            } else {
                image::Luma([0u8])
            }
        }));
        let result = edge_tracking(img.clone());
        //result is equal to a black image
        assert_eq!(desired_result, result);
    }
}
