use std::{env, process};

use once_cell::sync::Lazy;

///Returns if the terminal supports truecolor mode.
///
/// It checks the `COLORTERM` environment variable,
/// if it is either set to
/// `truecolor` or `24bit` true is returned.
///
/// In all other cases false will be returned.
///
/// # Examples
/// ```
/// use artem::util::SUPPORTS_TRUECOLOR;
/// # use std::env;
///
/// # env::set_var("COLORTERM", "truecolor");
/// //only true when run in a shell that supports true color
/// let color_support = *SUPPORTS_TRUECOLOR;
/// assert!(color_support);
/// ```
pub static SUPPORTS_TRUECOLOR: Lazy<bool> = Lazy::new(|| {
    env::var("COLORTERM").is_ok_and(|value| value.contains("truecolor") || value.contains("24bit"))
});

#[cfg(test)]
mod test_color_support {
    use super::*;

    #[test]
    #[ignore = "Requires truecolor support"]
    fn true_when_env_is_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert!(*SUPPORTS_TRUECOLOR);
    }

    #[test]
    #[ignore = "Requires truecolor support"]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert!(*SUPPORTS_TRUECOLOR);
    }

    #[test]
    fn false_with_different_env_false() {
        env::set_var("COLORTERM", "false");
        assert!(!*SUPPORTS_TRUECOLOR);
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "kjasdlkdjaskd");
        assert!(!*SUPPORTS_TRUECOLOR);
    }
}

///Remap a value from one range to another.
///
/// If the value is outside of the specified range, it will still be
/// converted as if it was in the range. This means it could be much larger or smaller than expected.
/// This can be fixed by using the `clamp` function after the remapping.
///
///# Examples
/// ```
/// use artem::util::map_range;
///
/// let remapped = map_range((0f32, 10f32), (0f32, 20f32), 2f32);
/// assert_eq!(4f32, remapped);
/// ```
pub fn map_range(from_range: (f32, f32), to_range: (f32, f32), value: f32) -> f32 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}

#[cfg(test)]
mod test_map_range {
    use super::*;

    #[test]
    fn remap_values() {
        //remap 2 to 4
        assert_eq!(4f32, map_range((0f32, 10f32), (0f32, 20f32), 2f32));
    }

    #[test]
    fn remap_values_above_range() {
        //remap 21 to 42, since the value will be doubled
        assert_eq!(42f32, map_range((0f32, 10f32), (0f32, 20f32), 21f32));
    }

    #[test]
    fn remap_values_below_range() {
        //remap -1 to -2, since the value will be doubled
        assert_eq!(-2f32, map_range((0f32, 10f32), (0f32, 20f32), -1f32));
    }
}

///Function for fatal errors.
///
///A fatal error is an error, from which the program can no recover, meaning the only option left is to print
/// an error message letting the user know what went wrong. For example if a non-existing file was passed in,
/// this program can not work correctly and should print an error message and exit.
///
/// This function will print the passed in error message as well as a exit message, then it will exit the program with the exit code.
/// If non is specified, it will use exit code 1 by default.
/// A list of exit code can be found here: <https://www.freebsd.org/cgi/man.cgi?query=sysexits&apropos=0&sektion=0&manpath=FreeBSD+4.3-RELEASE&format=html>
///
/// # Examples
/// ```no_run
/// use std::fs::File;
/// use artem::util::fatal_error;
///
/// let f = File::open("hello.txt");
/// let f = match f {
///     Ok(file) => file,
///     Err(error) => fatal_error(&error.to_string(), Some(66)),
/// };
/// ```
pub fn fatal_error(message: &str, code: Option<i32>) -> ! {
    //This function never returns, since it always exit the program
    log::error!("{}", message);
    log::error!("Artem exited with code: {}", code.unwrap_or(1));
    process::exit(code.unwrap_or(1));
}

/// Return a spacer string, which can be used to center the ascii image in the middle of the terminal.
///
/// When the terminal width is not existing, for example when the output is not a terminal, the returned string will be empty.
///
/// # Example
/// ```
/// # use artem::util::spacing_horizontal;
/// let  spacing = spacing_horizontal(10);
/// ```
pub fn spacing_horizontal(width: u32) -> String {
    let term_width = terminal_size::terminal_size()
        .map(|dimensions| dimensions.0 .0 as u32)
        .unwrap_or_default();
    " ".repeat(term_width.saturating_sub(width).saturating_div(2) as usize)
}

#[cfg(test)]
mod test_spacing_horizontal {
    use super::*;

    //can not be unit tested, since the terminal can have different sizes

    #[test]
    fn empty_return_large_input() {
        assert_eq!("", spacing_horizontal(u32::MAX))
    }
}

/// Return a spacer string, which can be used to center the ascii image in the middle of the terminal.
///
/// When the terminal height is not existing, for example when the output is not a terminal, the returned string will be empty.
///
/// # Example
/// ```
/// # use artem::util::spacing_vertical;
/// let  spacing = spacing_vertical(10);
/// ```
pub fn spacing_vertical(height: u32) -> String {
    let term_height = terminal_size::terminal_size()
        .map(|dimensions| dimensions.1 .0 as u32)
        .unwrap_or_default();
    log::trace!("H: {term_height}, h: {height}");
    "\n".repeat(term_height.saturating_sub(height).saturating_div(2) as usize)
}

#[cfg(test)]
mod test_spacing_vertical {
    use super::*;

    //can not be unit tested, since the terminal can have different sizes

    #[test]
    fn empty_return_large_input() {
        assert_eq!("", spacing_vertical(u32::MAX))
    }
}
