use std::{fs::File, io::Write, ops::Div, panic, sync::Arc, thread};

use colored::*;
use image::{DynamicImage, GenericImageView, Rgba};

//import cli
mod cli;
//import utilities
mod util;

fn main() {
    //get args from cli
    let matches = cli::build_cli().get_matches();

    //density char map
    let density = if matches.is_present("density") {
        match matches.value_of("density").unwrap() {
            "short" | "s" | "0" => r#"Ñ@#W$9876543210?!abc;:+=-,._ "#,
            "flat" | "f" | "1" => r#"MWNXK0Okxdolc:;,'...   "#,
            "long" | "l" | "2" => {
                r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#
            }
            _ => matches.value_of("density").unwrap(),
        }
    } else {
        //density map from jp2a
        r#"MWNXK0Okxdolc:;,'...   "#
    };

    //this should be save to unwrap since the input has to be non-null
    let img = match image::open(matches.value_of("INPUT").unwrap()) {
        Ok(img) => Arc::new(img),
        //Todo use error function
        Err(_) => panic!("Image not found"),
    };

    //get target size from args
    //only one arg should be present
    let target_size = if matches.is_present("height") {
        //use max terminal height
        (terminal_size::terminal_size().unwrap().1 .0 as f64 * 2f64).floor() as u32
    } else if matches.is_present("width") {
        //use max terminal width
        terminal_size::terminal_size().unwrap().0 .0 as u32
    } else {
        //use given input size
        match matches
            .value_of("size")
            .unwrap() //this should always be at least "80", so it should be safe to unwrap
            .parse::<u32>()
        {
            Ok(v) => v.clamp(
                20,  //min should be 20 to ensure a somewhat visible picture
                230, //img above 230 might not be displayed properly
            ),
            Err(_) => panic!("Could not work with size input value"),
        }
    };

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
        Err(_) => panic!("Could not work with ratio input value"),
    };

    //number rof threads used to convert the image
    let thread_count: u32 = match matches
        .value_of("threads")
        .unwrap() //this should always be at least "4", so it should be safe to unwrap
        .parse::<u32>()
    {
        Ok(v) => v,
        Err(_) => panic!("Could not work with size input value"),
    };

    //check if no colors should be used or the if a output file will be used
    //since text documents don`t support ansi ascii colors
    let color = if matches.is_present("no-color") || matches.is_present("output-file") {
        //print the "normal" non-colored conversion
        false
    } else {
        //print colored terminal conversion, this should already respect truecolor support/use ansi colors if not supported
        true
    };

    //convert the img to ascii string
    let output = convert_img(img, thread_count, density, scale, target_size, color);

    //create and write to output file
    if matches.is_present("output-file") && matches.value_of("output-file").is_some() {
        let mut file = match File::create(matches.value_of("output-file").unwrap()) {
            Ok(f) => f,
            Err(_) => panic!("Could not create file"),
        };

        match file.write(output.as_bytes()) {
            Ok(result) => println!(
                "Written {} bytes to {}",
                result,
                matches.value_of("output-file").unwrap()
            ),
            Err(_) => panic!("Could not write to file"),
        };
    } else {
        //print the img to the terminal
        println!("{}", output);
    }
}

///Converts the given image to an ascii representation
fn convert_img(
    img: Arc<DynamicImage>,
    thread_count: u32,
    density: &str,
    scale: f64,
    target_size: u32,
    color: bool,
) -> String {
    //get img dimensions
    let width = img.width();
    let height = img.height();

    //clamp image width to a maximum of 80
    let columns = if width > target_size {
        target_size
    } else {
        width
    };

    //calculate tiles
    let tile_width = width / columns;
    let tile_height = (tile_width as f64 / scale).floor() as u32;

    let rows = height / tile_height;

    //output string
    let mut output = String::new();

    //split the img into tile for each thread
    let thread_tiles = rows
        / thread_count.clamp(
            1,    //there has to be at least 1 thread to convert the img
            rows, //there should no be more threads than rows
        );
    //collect threads handles
    let mut handles = Vec::with_capacity(thread_count as usize);
    //split the img into chunks for each thread
    for chunk in 0..thread_count {
        //arc clone img and density
        let thread_img = Arc::clone(&img);
        let thread_density = density.to_owned();

        //create a thread for this img chunk
        let handle = thread::spawn(move || {
            //create thread string
            let mut thread_output = String::new();

            //go through the thread img chunk
            for row in chunk * thread_tiles..(chunk + 1) * thread_tiles {
                // for row in 0..rows {
                for col in 0..columns {
                    //get a single tile
                    let tile_row = row * tile_height;
                    let tile_col = col * tile_width;

                    //create a pixel block from multiple pixels
                    //preallocate vector with the correct size
                    let mut pixel_block: Vec<Rgba<u8>> =
                        Vec::with_capacity((tile_height * tile_width) as usize);

                    //go through each pixel in the tile
                    for x in tile_row..(tile_row + tile_height) {
                        for y in tile_col..(tile_col + tile_width) {
                            //add pixel to block
                            pixel_block.push(thread_img.get_pixel(y, x));
                        }
                    }

                    //get and display density char, it returns a normal and a colored string
                    let char = get_pixel_density(pixel_block, thread_density.as_str(), color);
                    //append the char for the output
                    thread_output.push_str(char.as_str());
                }
                //add new line
                if row != (chunk + 1) * thread_tiles - 1 || chunk != thread_count - 1 {
                    thread_output.push('\n');
                }
            }
            thread_output
        });
        handles.push(handle);
    }

    for handle in handles {
        //get thread result
        let result = handle.join().unwrap();
        //add output together
        output.push_str(result.as_str());
    }

    output
}

///Convert a pixel block to a char.
///The converted char will be returned as a string and as a colored string.
fn get_pixel_density(block: Vec<Rgba<u8>>, density: &str, color: bool) -> String {
    let mut block_avg: f64 = 0f64;
    //color as f64 for square rooting later
    let mut red: f64 = 0f64;
    let mut blue: f64 = 0f64;
    let mut green: f64 = 0f64;

    //average all pixel in a block
    //it might be possible to use a better algorithm for this
    for pixel in &block {
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
    let density_index = util::map_range((0f64, 255f64), (density.len() as f64, 0f64), block_avg)
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
            density_char
                .to_string()
                .truecolor(red.floor() as u8, green.floor() as u8, blue.floor() as u8)
                .to_string()
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
