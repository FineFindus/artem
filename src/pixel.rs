use std::ops::Div;

use image::Rgba;

use crate::{options, target, util};

/// Convert a pixel block to a char (as a String) from the given density string.
///
/// # Examples
///
/// ```compile_fail, compile will fail, this is an internal example
/// use image::Rgba;
/// use artem::options::TargetType;
///
/// //example pixels, use them from the directly if possible
/// let pixels = vec![
///     Rgba::<u8>::from([255, 255, 255, 255]),
///     Rgba::<u8>::from([0, 0, 0, 255]),
/// ];
///
/// assert_eq!(".", correlating_char(&pixels, "#k. ", false, TargetType::default()));
/// ```
///
/// To use color, use the `color` argument, if only the background should be colored, use the `on_background_color` arg instead.
///
/// The `invert` arg, inverts the mapping from pixel luminosity to density string.
pub fn correlating_char(
    block: &[Rgba<u8>],
    density: &str,
    invert: bool,
    target: options::TargetType,
) -> String {
    let (red, blue, green) = average_color(block);

    //calculate luminosity from avg. pixel color
    let luminosity = luminosity(red, green, blue);

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
        options::TargetType::Shell(true, background_color) => {
            target::ansi::colored_char(red, green, blue, density_char, background_color)
        }
        options::TargetType::AnsiFile(background_color) => {
            //ansi file is always colored
            target::ansi::colored_char(red, green, blue, density_char, background_color)
        }
        options::TargetType::HtmlFile(color, background_color) => {
            if color {
                target::html::colored_char(red, green, blue, density_char, background_color)
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
            " ",
            correlating_char(
                &pixels,
                "# ",
                false,
                options::TargetType::Shell(false, false)
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
            correlating_char(
                &pixels,
                "# ",
                true,
                options::TargetType::Shell(false, false)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::Shell(false, false)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::Shell(false, false)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::Shell(true, false)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::Shell(true, false)
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
            correlating_char(&pixels, "#k. ", false, options::TargetType::AnsiFile(false))
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::Shell(true, true)
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
            correlating_char(&pixels, "#k. ", false, options::TargetType::AnsiFile(true))
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
            correlating_char(&pixels, "#k. ", false, options::TargetType::File)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::HtmlFile(true, false)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::HtmlFile(true, true)
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
            correlating_char(
                &pixels,
                "#k. ",
                false,
                options::TargetType::HtmlFile(false, false)
            )
        );
    }
}

/// Returns the average rbg color of multiple pixel.
///
/// If the input block is empty, all pixels are seen and calculated as if there were black.
///
/// # Examples
///
/// ```compile_fail, compile will fail, this is an internal example
/// let pixels: Vec<Rgba<u8>> = Vec::new();
/// assert_eq!((0, 0, 0, 0.0), get_pixel_color_luminosity(&pixels));
/// ```
///
/// The formula for calculating the rbg colors is based an a minutephysics video <https://www.youtube.com/watch?v=LKnqECcg6Gw>
fn average_color(block: &[Rgba<u8>]) -> (u8, u8, u8) {
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

    //calculate average color according to √color1² + color2², round result
    red = red.div(block.len() as f64).sqrt().round();
    blue = blue.div(block.len() as f64).sqrt().round();
    green = green.div(block.len() as f64).sqrt().round();

    //convert to u8, since rgb is only u8
    (red as u8, blue as u8, green as u8)
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
            (180, 0, 180), //float values... https://imgs.xkcd.com/comics/e_to_the_pi_minus_pi.png
            average_color(&pixels)
        );
    }

    #[test]
    fn green_blue() {
        let pixels = vec![
            Rgba::<u8>::from([0, 255, 0, 255]),
            Rgba::<u8>::from([0, 0, 255, 255]),
        ];

        assert_eq!((0, 180, 180), average_color(&pixels));
    }

    #[test]
    fn empty_input() {
        let pixels: Vec<Rgba<u8>> = Vec::new();
        let (r, g, b) = average_color(&pixels);
        assert_eq!(0, r);
        assert_eq!(0, g);
        assert_eq!(0, b);
    }
}

/// Returns the luminosity of the given rgb colors as an float.
///
/// It converts the rgb values to floats, adds them with weightings and then returns them
/// as a float value.
///
/// # Examples
///
/// ```compile_fail, compile will fail, this is an internal example
/// use artem::pixel;
///
/// let luminosity = luminosity(154, 85, 54);
/// assert_eq!(97f64, luminosity);
/// ```
///
/// The formula/weighting for the colors comes from <http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/>
pub fn luminosity(red: u8, green: u8, blue: u8) -> f64 {
    (0.21 * red as f64) + (0.72 * green as f64) + (0.07 * blue as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminosity_black_is_zero() {
        assert_eq!(0f64, luminosity(0, 0, 0))
    }

    #[test]
    fn luminosity_white_is_255() {
        assert_eq!(254.99999999999997f64, luminosity(255, 255, 255))
    }

    #[test]
    fn luminosity_rust_color_is_255() {
        assert_eq!(97.32f64, luminosity(154, 85, 54))
    }
}
