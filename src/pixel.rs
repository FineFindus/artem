use image::Rgba;

use crate::{config, target, util};

/// Convert a pixel block to a char (as a String) from the given density string.
///
/// # Panics
///
/// Panics if either the given pixel block or the density is empty.
///
/// # Examples
///
/// ```compile_fail, compile will fail, this is an internal example
/// use image::Rgba;
/// use artem::config::TargetType;
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
    target: config::TargetType,
) -> String {
    assert!(!block.is_empty());
    assert!(!density.is_empty());

    let (red, green, blue) = average_color(block);

    //calculate luminosity from avg. pixel color
    let luminosity = luminosity(red, green, blue);

    //use chars length to support unicode chars
    let length = density.chars().count();

    //swap to range for white to black values
    //convert from rgb values (0 - 255) to the density string index (0 - string length)
    let density_index = util::map_range(
        (0f32, 255f32),
        if invert {
            (0f32, length as f32)
        } else {
            (length as f32, 0f32)
        },
        luminosity,
    )
    .floor()
    .clamp(0f32, length as f32 - 1.0);

    //get correct char from map
    assert!((density_index as usize) < length);
    let density_char = density
        .chars()
        .nth(density_index as usize)
        .expect("Failed to get char");

    //return the correctly formatted/colored string depending on the target
    match target {
        //if no color, use default case
        config::TargetType::Shell(true, background_color)
        | config::TargetType::AnsiFile(background_color) => {
            target::ansi::colored_char(red, green, blue, density_char, background_color)
        }
        config::TargetType::HtmlFile(color, background_color) => {
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
    fn invert_returns_first_instead_of_last_char() {
        let pixels = vec![
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([255, 255, 255, 255]),
            Rgba::<u8>::from([0, 0, 0, 255]),
        ];
        assert_eq!(
            " ",
            correlating_char(&pixels, "# ", true, config::TargetType::Shell(false, false))
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
                config::TargetType::Shell(false, false)
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
                config::TargetType::Shell(false, false)
            )
        );
    }

    #[test]
    #[ignore = "Requires truecolor support"]
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
                config::TargetType::Shell(true, false)
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
                config::TargetType::Shell(true, false)
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
            correlating_char(&pixels, "#k. ", false, config::TargetType::AnsiFile(false))
        );
    }

    #[test]
    #[ignore = "Requires truecolor support"]
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
                config::TargetType::Shell(true, true)
            )
        );
    }

    #[test]
    #[ignore = "Requires truecolor support"]
    fn colored_background_char_ansi() {
        //set needed env vars
        env::set_var("COLORTERM", "truecolor");
        //force color, this is not printed to the terminal anyways
        env::set_var("CLICOLOR_FORCE", "1");
        let pixels = vec![Rgba::<u8>::from([0, 0, 255, 255])];
        assert_eq!(
            "\u{1b}[48;2;0;0;255m \u{1b}[0m",
            correlating_char(&pixels, "#k. ", false, config::TargetType::AnsiFile(true))
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
            correlating_char(&pixels, "#k. ", false, config::TargetType::File)
        );
    }

    #[test]
    fn white_has_no_tag() {
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
                config::TargetType::HtmlFile(true, false)
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
            "<span style=\"color: #0000FF\">.</span>",
            correlating_char(
                &pixels,
                "#k:.",
                false,
                config::TargetType::HtmlFile(true, false)
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
                config::TargetType::HtmlFile(true, true)
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
                config::TargetType::HtmlFile(false, false)
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
    let sum = block
        .iter()
        .map(|pixel| {
            (
                pixel.0[0] as f32 * pixel.0[0] as f32,
                pixel.0[1] as f32 * pixel.0[1] as f32,
                pixel.0[2] as f32 * pixel.0[2] as f32,
            )
        })
        .fold((0f32, 0f32, 0f32), |acc, value| {
            (acc.0 + value.0, acc.1 + value.1, acc.2 + value.2)
        });
    (
        (sum.0 / block.len() as f32).sqrt() as u8,
        (sum.1 / block.len() as f32).sqrt() as u8,
        (sum.2 / block.len() as f32).sqrt() as u8,
    )
}

#[cfg(test)]
mod test_avg_color {
    use super::*;

    #[test]
    fn red_green() {
        let pixels = vec![
            Rgba::<u8>::from([255, 0, 0, 255]),
            Rgba::<u8>::from([0, 255, 0, 255]),
        ];

        assert_eq!((180, 180, 0), average_color(&pixels));
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
/// assert_eq!(97f32, luminosity);
/// ```
///
/// The formula/weighting for the colors comes from <http://www.johndcook.com/blog/2009/08/24/algorithms-convert-color-grayscale/>
pub fn luminosity(red: u8, green: u8, blue: u8) -> f32 {
    (0.21 * red as f32) + (0.72 * green as f32) + (0.07 * blue as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn luminosity_black_is_zero() {
        assert_eq!(0f32, luminosity(0, 0, 0))
    }

    #[test]
    fn luminosity_white_is_255() {
        assert_eq!(255.00002, luminosity(255, 255, 255))
    }

    #[test]
    fn luminosity_rust_color_is_255() {
        assert_eq!(97.32f32, luminosity(154, 85, 54))
    }
}
