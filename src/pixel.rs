use std::ops::Div;

use colored::Colorize;
use image::Rgba;

use crate::{conversion_options, util};

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
/// assert_eq!(".", get_pixel_density(&pixels, "#k. ", false, ConversionTargetType::default()));
/// ```
///
/// To use color, use the `color` argument, if only the background should be colored, use the `on_background_color` arg instead.
///
/// The `invert` arg, inverts the mapping from pixel luminosity to density string.
pub fn get_pixel_density(
    block: &[Rgba<u8>],
    density: &str,
    invert: bool,
    target: conversion_options::ConversionTargetType,
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

    //return the correctly formatted/colored string depending on the target
    match target {
        //if no color, use default case
        conversion_options::ConversionTargetType::Shell(true, background_color) => {
            get_colored_string(red, green, blue, density_char, background_color)
        }
        conversion_options::ConversionTargetType::AnsiFile(background_color) => {
            //ansi file is always colored
            get_colored_string(red, green, blue, density_char, background_color)
        }
        conversion_options::ConversionTargetType::HtmlFile(color, background_color) => {
            if color {
                get_html(red, green, blue, density_char, background_color)
            } else {
                density_char.to_string()
            }
        }
        //all other case, including a plain text file and shell without colors
        _ => density_char.to_string(),
    }
}

#[cfg(test)]
mod test_pixel_density {
    use std::env;

    use super::*;

    #[test]
    fn empty_returns_last_char() {
        let pixels: Vec<Rgba<u8>> = Vec::new();
        assert_eq!(
            "#",
            get_pixel_density(
                &pixels,
                "# ",
                false,
                conversion_options::ConversionTargetType::Shell(false, false)
            )
        );
    }

    #[test]
    fn invert_returns_first_instead_of_last_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!(
            " ",
            get_pixel_density(
                &pixels,
                "# ",
                true,
                conversion_options::ConversionTargetType::Shell(false, false)
            )
        );
    }

    #[test]
    fn medium_density_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!(
            "k",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::Shell(false, false)
            )
        );
    }

    #[test]
    fn dark_density_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!(
            "#",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::Shell(false, false)
            )
        );
    }

    #[test]
    fn colored_char() {
        //set needed env vars
        env::set_var("COLORTERM", "truecolor");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[38;2;0;0;255m \u{1b}[0m", //blue color
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::Shell(true, false)
            )
        );
    }

    #[test]
    fn ansi_colored_char_shell() {
        //set no color support
        env::set_var("COLORTERM", "false");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");
        //just some random color
        let pixels = vec![Rgba::<u8>::from([123, 42, 244, 255])];
        assert_eq!(
            "\u{1b}[35m.\u{1b}[0m",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::Shell(true, false)
            )
        );
    }

    #[test]
    fn ansi_colored_char_ansi() {
        //set no color support
        env::set_var("COLORTERM", "false");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");
        let pixels = vec![Rgba::<u8>::from([123, 42, 244, 255])];
        assert_eq!(
            "\u{1b}[35m.\u{1b}[0m",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::AnsiFile(false)
            )
        );
    }

    #[test]
    fn colored_background_char_shell() {
        //set needed env vars
        env::set_var("COLORTERM", "truecolor");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[48;2;0;0;255m \u{1b}[0m",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::Shell(true, true)
            )
        );
    }

    #[test]
    fn colored_background_char_ansi() {
        //set needed env vars
        env::set_var("COLORTERM", "truecolor");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");
        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[48;2;0;0;255m \u{1b}[0m",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::AnsiFile(true)
            )
        );
    }

    #[test]
    fn target_file_returns_non_colored_string() {
        //force color, this is not printed to the terminal anyways
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            " ",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::File
            )
        );
    }

    #[test]
    fn target_html_colored_string() {
        //force color, this is not printed to the terminal anyways
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "<span style=\"color: #0000FF\"> </span>",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::HtmlFile(true, false)
            )
        );
    }

    #[test]
    fn target_html_background_string() {
        //force color, this is not printed to the terminal anyways
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "<span style=\"background-color: #0000FF\"> </span>",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::HtmlFile(true, true)
            )
        );
    }

    #[test]
    fn target_html_no_color() {
        //force color, this is not printed to the terminal anyways
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");

        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            " ",
            get_pixel_density(
                &pixels,
                "#k. ",
                false,
                conversion_options::ConversionTargetType::HtmlFile(false, false)
            )
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
/// The formula for calculating the rbg colors is based an a minutephysics video <https://www.youtube.com/watch?v=LKnqECcg6Gw>
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
    let luminosity = get_luminosity(red, green, blue);

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

/// Returns the luminosity of the given rgb colors as an integer.
/// It converts the rgb values to floats, adds them with weightings and then returns them
/// rounded to the nearest integer.
///
/// # Examples
/// ```
/// let luminosity = get_luminosity(154, 85, 54);
/// assert_eq!(97, luminosity);
/// ```
///
/// The formula/weighting for the colors comes from <http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/>
fn get_luminosity(red: f64, green: f64, blue: f64) -> f64 {
    (0.21 * red) + (0.72 * green) + (0.07 * blue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminosity_black_is_zero() {
        assert_eq!(0f64, get_luminosity(0f64, 0f64, 0f64))
    }

    #[test]
    fn luminosity_white_is_255() {
        assert_eq!(
            254.99999999999997f64,
            get_luminosity(255f64, 255f64, 255f64)
        )
    }

    #[test]
    fn luminosity_rust_color_is_255() {
        assert_eq!(97.32f64, get_luminosity(154f64, 85f64, 54f64))
    }
}
/// Returns an colored string with the given colors.
///
/// Checks if true_colors are supported, by checking the `COLORTERM` environnement variable,
/// it then returns the given char as a colored string, either using true colors or ansi colors as a fallback.
/// Background colors are only supported when true colors are enabled.
/// # Examples
/// ```
/// println!("{}", get_colored_string(100, 100, 100, 'x', false));
/// ```
fn get_colored_string(red: u8, green: u8, blue: u8, char: char, background_color: bool) -> String {
    if util::supports_truecolor() {
        //return true color string
        if background_color {
            char.to_string().on_truecolor(red, green, blue).to_string()
        } else {
            char.to_string().truecolor(red, green, blue).to_string()
        }
    } else {
        //otherwise use basic (8 color) ansi color
        util::rgb_to_ansi(char.to_string().as_str(), red, green, blue).to_string()
    }
}

#[cfg(test)]
mod test_colored_string {
    use std::env;

    use super::*;

    #[test]
    fn rust_color_no_background() {
        //ensure that colors will be used
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "x".truecolor(154, 85, 54).to_string(),
            get_colored_string(154, 85, 54, 'x', false)
        );
    }

    #[test]
    fn rust_color_with_background() {
        //ensure that colors will be used
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "x".on_truecolor(154, 85, 54).to_string(),
            get_colored_string(154, 85, 54, 'x', true)
        );
    }

    #[test]
    fn rust_color_ansi_no_background() {
        //set true color support to false
        env::set_var("COLORTERM", "false");
        //ensure that colors will be used
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "\u{1b}[33mx\u{1b}[0m",
            get_colored_string(154, 85, 54, 'x', false)
        );
    }

    #[test]
    fn rust_color_ansi_with_background() {
        //set true color support to false
        env::set_var("COLORTERM", "false");
        //ensure that colors will be used
        env::set_var("CLICOLOR_FORCE", "1");
        //ansi does not support background, so it is the same as without
        assert_eq!(
            "\u{1b}[33mx\u{1b}[0m",
            get_colored_string(154, 85, 54, 'x', true)
        );
    }
}

/// Returns an html string representation of the given char with optional background color support.
///
/// Creates an <span> element with style attribute, which sets the (background) color to the
/// given rgb inputs.
///
/// # Examples
/// ```
/// println!("{}", get_html(100, 100, 100, 'x', false));
/// ```
fn get_html(red: u8, green: u8, blue: u8, char: char, background_color: bool) -> String {
    if background_color {
        format!(
            "<span style=\"background-color: #{:02X?}{:02X?}{:02X?}\">{}</span>",
            red, green, blue, char
        )
    } else {
        format!(
            "<span style=\"color: #{:02X?}{:02X?}{:02X?}\">{}</span>",
            red, green, blue, char
        )
    }
}

#[cfg(test)]
mod test_html_string {
    use super::*;

    #[test]
    fn black_no_background() {
        assert_eq!(
            "<span style=\"color: #000000\">x</span>",
            get_html(0, 0, 0, 'x', false)
        )
    }

    #[test]
    fn black_with_background() {
        assert_eq!(
            "<span style=\"background-color: #000000\">x</span>",
            get_html(0, 0, 0, 'x', true)
        )
    }

    #[test]
    fn rust_color_no_background() {
        assert_eq!(
            "<span style=\"color: #9A5536\">x</span>",
            get_html(154, 85, 54, 'x', false)
        )
    }

    #[test]
    fn rust_color_with_background() {
        assert_eq!(
            "<span style=\"background-color: #9A5536\">x</span>",
            get_html(154, 85, 54, 'x', true)
        )
    }
}
