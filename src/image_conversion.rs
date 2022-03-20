use std::{ops::Div, sync::Arc, thread};

use colored::Colorize;
use image::{DynamicImage, GenericImageView, Rgba};
use log::{debug, info, trace};

use crate::{conversion_options::ConversionOption, util};

///Returns the given image as ascii representation string.
///
/// # Examples
/// ```
/// let img = image::open(img_path).unwrap();
/// let converted_image = convert_img(img, ConversionOptionBuilder::new().build());
/// ```
///It uses the [ConversionOption] to set specific options on how to convert the image.
pub fn convert_img(img: DynamicImage, options: ConversionOption) -> String {
    debug!("Using Color: {}", options.color);
    debug!("Using colored background: {}", options.background_color);
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
                        options.color,
                        options.invert,
                        options.background_color,
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

    output.trim_end().to_string()
}

/// Convert a pixel block to a char (as a String) from the given density string.
///
/// # Examples
///
/// ```
/// //example pixels, use them from the directly if possible
/// let pixels = vec![
///     Rgba::<u8>::from([255, 255, 255, 255]),
///     Rgba::<u8>::from([0, 0, 0, 255]),
/// ];
///
/// assert_eq!(".", get_pixel_density(&pixels, " .k#", false, false, false));
/// ```
///
/// To use color, use the `color` argument, if only the background should be colored, use the `on_background_color` arg instead.
///
/// The `invert` arg, inverts the mapping from pixel luminosity to density string.
fn get_pixel_density(
    block: &[Rgba<u8>],
    density: &str,
    color: bool,
    invert: bool,
    on_background_color: bool,
) -> String {
    let (red, blue, green, luminosity) = get_pixel_color_luminosity(block);

    //swap to range for white to black values
    //convert from rgb values (0 - 255) to the density string index (0 - string length)
    let density_index = util::map_range(
        (0f64, 255f64),
        if invert {
            (0f64, density.len() as f64)
        } else {
            (density.len() as f64, 0f64)
        },
        luminosity,
    )
    .floor()
    .clamp(0f64, density.len() as f64);

    //get correct char from map, default to a space
    let density_char = density.chars().nth(density_index as usize).unwrap_or(' ');
    //return if needed a colored or non colored string
    if color {
        //check if true color is supported
        if util::supports_truecolor() {
            //return true color string
            if on_background_color {
                density_char
                    .to_string()
                    .on_truecolor(red, green as u8, blue as u8)
                    .to_string()
            } else {
                density_char
                    .to_string()
                    .truecolor(red, green, blue as u8)
                    .to_string()
            }
        } else {
            //otherwise use basic (8 color) ansi color
            util::rgb_to_ansi(density_char.to_string().as_str(), red, green, blue).to_string()
        }
    } else {
        density_char.to_string()
    }
}
#[cfg(test)]
mod test_pixel_density {
    use super::*;

    #[test]
    fn empty_returns_last_char() {
        let pixels: Vec<Rgba<u8>> = Vec::new();
        assert_eq!("#", get_pixel_density(&pixels, "# ", false, false, false));
    }

    #[test]
    fn invert_returns_first_instead_of_last_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!(" ", get_pixel_density(&pixels, "# ", false, true, false));
    }

    #[test]
    fn medium_density_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!("k", get_pixel_density(&pixels, "#k. ", false, false, false));
    }

    #[test]
    fn dark_density_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!("#", get_pixel_density(&pixels, "#k. ", false, false, false));
    }

    #[test]
    fn colored_char() {
        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[38;2;0;0;255m \u{1b}[0m", //blue color
            get_pixel_density(&pixels, "#k. ", true, false, false)
        );
    }

    #[test]
    fn colored_background_char() {
        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[48;2;0;0;255m \u{1b}[0m",
            get_pixel_density(&pixels, "#k. ", true, false, true)
        );
    }
}

/// Returns the rbg colors as well as the luminosity of multiple pixel.
///
/// First the average pixel color will be calculated, the based on those result the luminosity
/// will be calculated, suing the formula `0.21 * red + 0.72 * green + 0.07 * blue`.
///
/// If the input block is empty, all pixels are seen and calculated as if there were black.
///
/// # Examples
///
/// ```
/// let pixels: Vec<Rgba<u8>> = Vec::new();
/// assert_eq!((0, 0, 0, 0.0), get_pixel_color_luminosity(&pixels));
/// ```
///
/// The formula for calculating the rbg colors is based an a minutephysics video <https://www.youtube.com/watch?v=LKnqECcg6Gw><br>
/// whilst the luminosity formulas is from <http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/>
fn get_pixel_color_luminosity(block: &[Rgba<u8>]) -> (u8, u8, u8, f64) {
    //color as f64 for square rooting later
    let mut red: f64 = 0f64;
    let mut blue: f64 = 0f64;
    let mut green: f64 = 0f64;

    //average all pixel in a block
    for pixel in block {
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;

        //rgb values have to squared and rooted to get avg color
        red += r * r;
        blue += b * b;
        green += g * g;
    }

    //block average color
    red = red.div(block.len() as f64).sqrt();
    blue = blue.div(block.len() as f64).sqrt();
    green = green.div(block.len() as f64).sqrt();

    //calculate luminosity from avg. pixel color
    let luminosity = 0.21 * red + 0.72 * green + 0.07 * blue;

    (
        red.round() as u8,
        blue.round() as u8,
        green.round() as u8,
        luminosity,
    )
}

#[cfg(test)]
mod test_pixel_color_luminosity {
    use super::*;

    #[test]
    fn red_green() {
        let pixels = vec![
            Rgba::<u8>::from([255, 0, 0, 255]),
            Rgba::<u8>::from([0, 255, 0, 255]),
        ];

        assert_eq!(
            (180, 0, 180, 167.69037315838978), //float values... https://imgs.xkcd.com/comics/e_to_the_pi_minus_pi.png
            get_pixel_color_luminosity(&pixels)
        );
    }

    #[test]
    fn green_blue() {
        let pixels = vec![
            Rgba::<u8>::from([0, 255, 0, 255]),
            Rgba::<u8>::from([0, 0, 255, 255]),
        ];

        assert_eq!(
            (0, 180, 180, 142.44666107003002),
            get_pixel_color_luminosity(&pixels)
        );
    }

    #[test]
    fn empty_input() {
        let pixels: Vec<Rgba<u8>> = Vec::new();
        let (r, g, b, l) = get_pixel_color_luminosity(&pixels);
        assert_eq!(0, r);
        assert_eq!(0, g);
        assert_eq!(0, b);
        assert!(l.is_nan())
    }
}
