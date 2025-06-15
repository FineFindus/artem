use colored::{ColoredString, Colorize};

/// Returns an colored string with the given colors.
///
/// Checks if true_colors are supported, by checking the `COLORTERM` environnement variable,
/// it then returns the given char as a colored string, either using true colors or ansi colors as a fallback.
/// Background colors are only supported when true colors are enabled.
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// println!("{}", get_colored_string(100, 100, 100, 'x', false));
/// ```
pub fn colored_char(red: u8, green: u8, blue: u8, char: char, background_color: bool) -> String {
    if *crate::SUPPORTS_TRUECOLOR {
        // return true color string
        if background_color {
            char.to_string().on_truecolor(red, green, blue).to_string()
        } else {
            char.to_string().truecolor(red, green, blue).to_string()
        }
    } else {
        // otherwise use basic (8 color) ansi color
        rgb_to_ansi(&char.to_string(), red, green, blue).to_string()
    }
}

#[cfg(test)]
mod test_colored_string {
    use std::env;

    use super::*;

    #[test]
    #[ignore = "Requires truecolor support"]
    fn rust_color_no_background() {
        // ensure that colors will be used
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "x".truecolor(154, 85, 54).to_string(),
            colored_char(154, 85, 54, 'x', false)
        );
    }

    #[test]
    #[ignore = "Requires truecolor support"]
    fn rust_color_with_background() {
        // ensure that colors will be used
        env::set_var("COLORTERM", "truecolor");
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "x".on_truecolor(154, 85, 54).to_string(),
            colored_char(154, 85, 54, 'x', true)
        );
    }

    #[test]
    fn rust_color_ansi_no_background() {
        // set true color support to false
        env::set_var("COLORTERM", "false");
        // ensure that colors will be used
        env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(
            "\u{1b}[33mx\u{1b}[0m",
            colored_char(154, 85, 54, 'x', false)
        );
    }

    #[test]
    fn rust_color_ansi_with_background() {
        // set true color support to false
        env::set_var("COLORTERM", "false");
        // ensure that colors will be used
        env::set_var("CLICOLOR_FORCE", "1");
        // ansi does not support background, so it is the same as without
        assert_eq!("\u{1b}[33mx\u{1b}[0m", colored_char(154, 85, 54, 'x', true));
    }
}

/// Converts the given input string to an ansi colored string
///
/// It tries to match the ANSI-Color as closely as possible by calculating the distance between all
/// 8 colors and the given input color from `r`, `b` and `b`, then returning the nearest.
/// It will not be 100% accurate, since every terminal has slightly different
/// ANSI-Colors. It used the VGA-Colors as ANSI-Color.
///
/// # Examples
/// ```compile_fail, compile will fail, this is an internal example
/// // convert black to ansi black color
/// assert_eq!("input".black(), rgb_to_ansi("input", 0, 0, 0));
/// ```
fn rgb_to_ansi(input: &str, r: u8, g: u8, b: u8) -> ColoredString {
    // get RGB values and convert them to i32, since later on the could negative when subtracting
    let r = r as i32;
    let g = g as i32;
    let b = b as i32;

    // vga colors as example ansi color
    // from https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
    let vga_colors = [
        [0, 0, 0],       // black
        [170, 0, 0],     // red
        [0, 170, 0],     // green
        [170, 85, 0],    // yellow
        [0, 0, 170],     // blue
        [170, 0, 170],   // magenta
        [0, 170, 170],   // cyan
        [170, 170, 170], // white
        [128, 128, 128], // bright black/gray
        [255, 0, 0],     // bright red
        [0, 255, 0],     // bright green
        [255, 255, 0],   // bright yellow
        [0, 0, 255],     // bright blue
        [255, 0, 255],   // bright magenta
        [0, 255, 255],   // bright cyan
        [255, 255, 255], // bright white
    ];

    // find nearest color
    let mut smallest_distance = i32::MAX;
    let mut smallest_distance_index: u8 = 7;
    // maybe there is a better method for this
    for (index, vga_color) in vga_colors.iter().enumerate() {
        let distance =
            (r - vga_color[0]).pow(2) + (g - vga_color[1]).pow(2) + (b - vga_color[2]).pow(2);

        if distance < smallest_distance {
            smallest_distance = distance;
            smallest_distance_index = index as u8;
        }
    }

    // convert string to matching color
    match smallest_distance_index {
        0 => input.black(),
        1 => input.red(),
        2 => input.green(),
        3 => input.yellow(),
        4 => input.blue(),
        5 => input.magenta(),
        6 => input.cyan(),
        7 => input.white(),
        8 => input.bright_black(),
        9 => input.bright_red(),
        10 => input.bright_green(),
        11 => input.bright_yellow(),
        12 => input.bright_blue(),
        13 => input.bright_magenta(),
        14 => input.bright_cyan(),
        15 => input.bright_white(),
        _ => input.normal(),
    }
}

#[cfg(test)]
mod test_convert_rgb_ansi {
    use super::*;

    #[test]
    fn convert_vga_normal_values() {
        // convert black to ansi black color
        assert_eq!("input".black(), rgb_to_ansi("input", 0, 0, 0));
        // convert red to ansi red color
        assert_eq!("input".red(), rgb_to_ansi("input", 170, 0, 0));
        // convert green to ansi green color
        assert_eq!("input".green(), rgb_to_ansi("input", 0, 170, 0));
        // convert yellow to ansi yellow color
        assert_eq!("input".yellow(), rgb_to_ansi("input", 170, 85, 0));
        // convert blue to ansi blue color
        assert_eq!("input".blue(), rgb_to_ansi("input", 0, 0, 170));
        // convert magenta to ansi magenta color
        assert_eq!("input".magenta(), rgb_to_ansi("input", 170, 0, 170));
        // convert cyan to ansi cyan color
        assert_eq!("input".cyan(), rgb_to_ansi("input", 0, 170, 170));
        // convert white to ansi white color
        assert_eq!("input".white(), rgb_to_ansi("input", 170, 170, 170));
    }

    #[test]
    fn convert_vga_bright_values() {
        // convert bright black to ansi bright black color
        assert_eq!("input".bright_black(), rgb_to_ansi("input", 128, 128, 128));
        // convert bright red to ansi bright red color
        assert_eq!("input".bright_red(), rgb_to_ansi("input", 255, 0, 0));
        // convert bright green to ansi bright green color
        assert_eq!("input".bright_green(), rgb_to_ansi("input", 0, 255, 0));
        // convert bright yellow to ansi bright yellow color
        assert_eq!("input".bright_yellow(), rgb_to_ansi("input", 255, 255, 0));
        // convert bright blue to ansi bright blue color
        assert_eq!("input".bright_blue(), rgb_to_ansi("input", 0, 0, 255));
        // convert bright magenta to ansi bright magenta color
        assert_eq!("input".bright_magenta(), rgb_to_ansi("input", 255, 0, 255));
        // convert bright cyan to ansi bright cyan color
        assert_eq!("input".bright_cyan(), rgb_to_ansi("input", 0, 255, 255));
        // convert bright white to ansi bright white color
        assert_eq!("input".bright_white(), rgb_to_ansi("input", 255, 255, 255));
    }

    #[test]
    fn rgb_blue() {
        // convert a blue RGB tone to ansi blue
        assert_eq!("input".blue(), rgb_to_ansi("input", 0, 0, 88));
    }
}
