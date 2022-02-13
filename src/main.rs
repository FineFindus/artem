use std::{fs::File, io::Write};

use image::{GenericImageView, Rgba};

fn main() {
    //density chars
    // let density = r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#;
    let density = r#"Ñ@#W$9876543210?!abc;:+=-,._ "#;
    let img = image::open("gloria48.jpg").unwrap();

    let mut file = File::create("foo.txt").unwrap();

    // let width = img.dimensions().0;
    // let height = img.dimensions().1;

    for row in 0..img.dimensions().0 {
        for col in 0..img.dimensions().1 {
            let pixel = img.get_pixel(row, col);
            let char = get_pixel_density(pixel, density);
            file.write(char.to_string().as_bytes()).unwrap();
        }
        file.write(b"\n").unwrap();
    }
}

fn map_range(from_range: (f64, f64), to_range: (f64, f64), s: f64) -> f64 {
    to_range.0 + (s - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

fn get_pixel_density(pixel: Rgba<u8>, density: &str) -> char {
    let r = pixel.0[0] as f64;
    let g = pixel.0[1] as f64;
    let b = pixel.0[2] as f64;
    //avg color http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/
    let pixel_avg = 0.21 * r + 0.72 * g + 0.07 * b;
    // let pixel_avg = (r + g + b) / 3f64;

    // let density_index = map(pixel_avg, 0f64, 255f64, density.len() as f64, 0f64)
    let density_index = map_range((0f64, 255f64), (density.len() as f64, 0f64), pixel_avg)
        .floor()
        .clamp(0f64, density.len() as f64);
    println!("density_index: {}", density_index);
    println!("char: {:?}", density.chars().nth(density_index as usize));

    let density_char = density.chars().nth(density_index as usize);
    if density_char.is_some() {
        density_char.unwrap()
    } else {
        ' '
    }
}
