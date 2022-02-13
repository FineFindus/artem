use std::{fs::File, io::Write};

use image::{GenericImageView, Rgba};

fn main() {
    //density chars
    // let density = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
    let density = r#"Ñ@#W$9876543210?!abc;:+=-,._ "#;
    // let img = image::open("gloria48.jpg").unwrap();
    // let img = image::open("art.jpg").unwrap();
    // let img = image::open("homer.png").unwrap();
    let img = image::open("samsung.jpg").unwrap();

    let mut file = File::create("foo.txt").unwrap();

    let width = img.width();
    let height = img.height();

    let columns = 80;
    let scale = 0.43;

    //calculate tiles
    let tile_width = width / columns;
    let tile_height = (tile_width as f64 / scale).floor() as u32;

    let rows = height / tile_height;

    for row in 0..rows {
        for col in 0..columns {
            //get a single tile
            let tile_row = row * tile_height;
            let tile_col = col * tile_width;
            //create a pixel block from multiple pixels
            let mut pixel_block: Vec<Rgba<u8>> = Vec::new();
            //crop image to smaller block
            let crop = img.crop_imm(tile_col, tile_row, tile_width, tile_height);
            for pixel in crop.pixels() {
                //add pixel to block
                pixel_block.push(pixel.2);
            }
            //get and display density char
            let char = get_pixel_density(pixel_block, density);
            file.write(char.to_string().as_bytes()).unwrap();
        }
        file.write(b"\n").unwrap();
    }
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn get_pixel_density(block: Vec<Rgba<u8>>, density: &str) -> char {
    let mut block_avg: f64 = 0f64;
    for pixel in &block {
        let r = pixel.0[0] as f64;
        let g = pixel.0[1] as f64;
        let b = pixel.0[2] as f64;
        //avg color http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
        let pixel_avg = 0.21 * r + 0.72 * g + 0.07 * b;
        block_avg += pixel_avg;
    }

    block_avg /= block.len() as f64;
    // let pixel_avg = (r + g + b) / 3f64;

    // let density_index = map_range((0f64, 255f64), (density.len() as f64, 0f64), block_avg)
    //swap to range for white to black values
    let density_index = map_range((0f64, 255f64), (0f64, density.len() as f64), block_avg)
        .floor()
        .clamp(0f64, density.len() as f64);
    // println!("density_index: {}", density_index);
    println!("char: {:?}", density.chars().nth(density_index as usize));

    let density_char = density.chars().nth(density_index as usize);
    if density_char.is_some() {
        density_char.unwrap()
    } else {
        ' '
    }
}
