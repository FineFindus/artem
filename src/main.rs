use std::{fs::File, io::Write, ops::Div, path::Path, sync::Arc, thread};

use colored::*;
use conversion_options::ConversionOption;
use image::{DynamicImage, GenericImageView, Rgba};
use log::{debug, info, trace, warn, LevelFilter};

use crate::conversion_options::ConversionOptionBuilder;

//import cli
mod cli;
//import utilities
mod util;

//condense all arguments into a single struct
mod conversion_options;

fn main() {
    //get args from cli
    let matches = cli::build_cli().get_matches();

    //get log level from args
    let log_level = match matches.value_of("verbosity") {
        Some("trace") => LevelFilter::Trace,
        Some("debug") => LevelFilter::Debug,
        Some("info") => LevelFilter::Info,
        Some("warn") => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        _ => LevelFilter::Error,
    };

    //enable logging
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log_level)
        .init();
    trace!("Started logger with trace");

    let mut options_builder = ConversionOptionBuilder::new();

    //this should be save to unwrap since the input has to be non-null
    let img_path = matches.value_of("INPUT").unwrap();
    //check if file exist
    if !Path::new(img_path).exists() {
        util::fatal_error(format!("File {img_path} does not exist").as_str(), Some(66));
    } else if !Path::new(img_path).is_file() {
        util::fatal_error(format!("{img_path} is not a file").as_str(), Some(66));
    }

    //try to open img
    let img = match image::open(img_path) {
        Ok(img) => img,
        Err(err) => util::fatal_error(err.to_string().as_str(), Some(66)),
    };

    //density char map
    let density = if matches.is_present("density") {
        match matches.value_of("density").unwrap() {
            "short" | "s" | "0" => r#"Ñ@#W$9876543210?!abc;:+=-,._ "#,
            "flat" | "f" | "1" => r#"MWNXK0Okxdolc:;,'...   "#,
            "long" | "l" | "2" => {
                r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#
            }
            _ => {
                info!("Using user provided characters");
                matches.value_of("density").unwrap()
            }
        }
    } else {
        //density map from jp2a
        info!("Using default characters");
        r#"MWNXK0Okxdolc:;,'...   "#
    };
    debug!("Characters used: \"{density}\"");
    options_builder = options_builder.density(density);

    //set the default resizing dimension to width
    options_builder = options_builder.dimension(util::ResizingDimension::Width);

    //get target size from args
    //only one arg should be present
    let target_size = if matches.is_present("height") {
        //use max terminal height
        trace!("Using terminal height as target size");
        //change dimension to height
        options_builder = options_builder.dimension(util::ResizingDimension::Height);
        terminal_size::terminal_size().unwrap().1 .0 as u32
    } else if matches.is_present("width") {
        //use max terminal width
        trace!("Using terminal width as target size");
        terminal_size::terminal_size().unwrap().0 .0 as u32
    } else {
        //use given input size
        trace!("Using user input size as target size");
        match matches
            .value_of("size")
            .unwrap() //this should always be at least "80", so it should be safe to unwrap
            .parse::<u32>()
        {
            Ok(v) => v.clamp(
                20,  //min should be 20 to ensure a somewhat visible picture
                230, //img above 230 might not be displayed properly
            ),
            Err(_) => util::fatal_error("Could not work with size input value", Some(65)),
        }
    };
    debug!("Target Size: {target_size}");
    options_builder = options_builder.target_size(target_size);

    //best ratio between height and width is 0.43
    let scale = match matches
        .value_of("scale")
        .unwrap() //this should always be at least "0.43", so it should be safe to unwrap
        .parse::<f64>()
    {
        Ok(v) => v.clamp(
            0f64, //a negative scale is not allowed
            1f64, //even a scale above 0.43 is not looking good
        ),
        Err(_) => util::fatal_error("Could not work with ratio input value", Some(65)),
    };
    debug!("Scale: {scale}");
    options_builder = options_builder.scale(scale);

    //number rof threads used to convert the image
    let thread_count: u32 = match matches
        .value_of("threads")
        .unwrap() //this should always be at least "4", so it should be safe to unwrap
        .parse::<u32>()
    {
        Ok(v) => v,
        Err(_) => util::fatal_error("Could not work with thread input value", Some(65)),
    };
    options_builder = options_builder.thread_count(thread_count);

    if !matches.is_present("no-color") && matches.is_present("output-file") {
        warn!("Output-file flag is present, ignoring colors")
    }

    let invert = matches.is_present("invert-density");
    debug!("Invert is set to: {invert}");
    options_builder = options_builder.invert(invert);

    let on_background_color = matches.is_present("background-color");
    debug!("BackgroundColor is set to: {on_background_color}");
    options_builder = options_builder.on_background(on_background_color);

    //check if no colors should be used or the if a output file will be used
    //since text documents don`t support ansi ascii colors
    let color = if matches.is_present("no-color") || matches.is_present("output-file") {
        //print the "normal" non-colored conversion
        info!("Using non-colored ascii");
        false
    } else {
        //print colored terminal conversion, this should already respect truecolor support/use ansi colors if not supported
        info!("Using colored ascii");
        let truecolor = util::supports_truecolor();
        if !truecolor {
            if on_background_color {
                warn!("Background flag will be ignored, since truecolor is not supported.")
            }
            warn!("Truecolor is not supported. Using ansi color")
        } else {
            info!("Using truecolor ascii")
        }
        true
    };
    options_builder = options_builder.color(color);

    //get flag for border around image
    let border = matches.is_present("border");
    options_builder = options_builder.border(border);
    info!("Using border: {border}");

    //get flags for flipping along x axis
    let transform_x = matches.is_present("flipX");
    options_builder = options_builder.transform_x(transform_x);
    debug!("Flipping X-Axis: {transform_x}");

    //get flags for flipping along y axis
    let transform_y = matches.is_present("flipY");
    options_builder = options_builder.transform_y(transform_y);
    debug!("Flipping Y-Axis: {transform_y}");

    //convert the img to ascii string
    info!("Converting the img: {img_path}");
    let output = convert_img(img, options_builder.build());

    //create and write to output file
    if matches.is_present("output-file") && matches.value_of("output-file").is_some() {
        info!("Writing output to output file");
        let mut file = match File::create(matches.value_of("output-file").unwrap()) {
            Ok(f) => f,
            Err(_) => util::fatal_error("Could not create file", Some(73)),
        };
        trace!("Created output file");

        match file.write(output.as_bytes()) {
            Ok(result) => {
                info!("Written ascii chars to output file");
                println!(
                    "Written {result} bytes to {}",
                    matches.value_of("output-file").unwrap()
                )
            }
            Err(_) => util::fatal_error("Could not write to file", Some(74)),
        };
    } else {
        //print the img to the terminal
        info!("Printing output");
        println!("{output}");
    }
}

///Converts the given image to an ascii representation
fn convert_img(img: DynamicImage, options: ConversionOption) -> String {
    debug!("Using Color: {}", options.color);
    debug!("Using colored background: {}", options.on_background_color);
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
    let thread_count = options.thread_count.clamp(
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
                        thread_density.as_str(),
                        options.color,
                        options.invert,
                        options.on_background_color,
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

///Convert a pixel block to a char.
///The converted char will be returned as a string and as a colored string.
fn get_pixel_density(
    block: &[Rgba<u8>],
    density: &str,
    color: bool,
    invert: bool,
    on_background_color: bool,
) -> String {
    let mut block_avg: f64 = 0f64;
    //color as f64 for square rooting later
    let mut red: f64 = 0f64;
    let mut blue: f64 = 0f64;
    let mut green: f64 = 0f64;

    //average all pixel in a block
    //it might be possible to use a better algorithm for this
    for pixel in block {
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;
        //save the pixel values
        //rgb values have to squared and rooted to get avg color (https://sighack.com/post/averaging-rgb-colors-the-right-way)
        red += r * r;
        blue += b * b;
        green += g * g;
        //luminosity color http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
        let pixel_luminosity = 0.21 * r + 0.72 * g + 0.07 * b;
        block_avg += pixel_luminosity;
    }

    //block average luminosity
    block_avg /= block.len() as f64;
    //block average color
    red = red.div(block.len() as f64).sqrt();
    blue = blue.div(block.len() as f64).sqrt();
    green = green.div(block.len() as f64).sqrt();

    //swap to range for white to black values
    //convert from rgb values (0 - 255) to the density string index (0 - string length)
    let density_index = util::map_range(
        (0f64, 255f64),
        if invert {
            (0f64, density.len() as f64)
        } else {
            (density.len() as f64, 0f64)
        },
        block_avg,
    )
    .floor()
    .clamp(0f64, density.len() as f64);

    //todo use directional chars
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
                    .on_truecolor(red.floor() as u8, green.floor() as u8, blue.floor() as u8)
                    .to_string()
            } else {
                density_char
                    .to_string()
                    .truecolor(red.floor() as u8, green.floor() as u8, blue.floor() as u8)
                    .to_string()
            }
        } else {
            //otherwise use basic (8 color) ansi color
            util::convert_rgb_ansi(
                density_char.to_string().as_str(),
                red as u8,
                green as u8,
                blue as u8,
            )
            .to_string()
        }
    } else {
        density_char.to_string()
    }
}
