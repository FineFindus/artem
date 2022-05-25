use std::{env, process};

use log::error;

///Returns if the terminal supports truecolor mode.
///
/// It checks the `COLORTERM` environnement variable,
/// if it is either set to
/// `truecolor` or `24bit` true is returned.
///
/// In all other cases false will be returned.
///
/// # Examples
/// ```
/// use artem::util::supports_truecolor;
/// # use std::env;
///
/// # env::set_var("COLORTERM", "truecolor");
/// //only true when run in a shell that supports true color
/// let color_support = supports_truecolor();
/// assert!(color_support);
/// ```
pub fn supports_truecolor() -> bool {
    match env::var("COLORTERM") {
        Ok(value) => value.contains("truecolor") || value.contains("24bit"),
        Err(_) => false, //not found, true colors are not supported
    }
}

#[cfg(test)]
mod test_color_support {
    use super::*;

    #[test]
    fn true_when_env_is_truecolor() {
        env::set_var("COLORTERM", "truecolor");
        assert!(supports_truecolor());
    }

    #[test]
    fn true_when_env_is_24bit() {
        env::set_var("COLORTERM", "24bit");
        assert!(supports_truecolor());
    }

    #[test]
    fn false_with_different_env() {
        env::set_var("COLORTERM", "false");
        assert!(!supports_truecolor());
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
///A fatal error is an error, from which the program can no recover, meaning the only option left ist to print
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
///     Err(error) => fatal_error(error.to_string().as_str(), Some(66)),
/// };
/// ```
pub fn fatal_error(message: &str, code: Option<i32>) -> ! {
    //This function never returns, since it always exit the program
    error!("{}", message);
    error!("Artem exited with code: {}", code.unwrap_or(1));
    process::exit(code.unwrap_or(1));
}

/// Calculate image dimension related values.
///
/// This calculates the number of columns, rows, and the tile dimensions (tile_width, tile_height) for these
/// values based on a target_size. It returns them as a tuple, the elements are in the previously named order.
/// The dimension property can be used to change what dimension will be scaled. Since terminal character are a bit higher the wide,
/// Width and Height of the output needs to be based on either one, so the other can be calculated.
///
/// # Examples
/// ```
/// use artem::util::{ResizingDimension, calculate_dimensions};
///
/// assert_eq!(
/// (100, 46, 5, 11),
/// //image with a size of 512x512, split into 100 columns with no border
/// calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Width));
/// ```
pub fn calculate_dimensions(
    target_size: u32,
    height: u32,
    width: u32,
    scale: f32,
    border: bool,
    dimension: ResizingDimension,
) -> (u32, u32, u32, u32) {
    match dimension {
        ResizingDimension::Width => {
            //calculate dimensions based on columns
            let mut columns = if width > target_size {
                target_size
            } else {
                width
            };

            if border {
                //remove a bit of space for the border
                columns = columns.saturating_sub(2).max(1); //should be at last 1
            }

            //calculate tiles
            let tile_width = width / columns;
            let tile_height = (tile_width as f32 / scale).floor() as u32;

            let rows = height / tile_height;

            //.max(1) is used to ensure that the values are at least 1
            //a value of 0 could cause an error (but not crash) later on
            (columns.max(1), rows.max(1), tile_width, tile_height)
        }

        ResizingDimension::Height => {
            let mut rows = if height > target_size {
                // minus 1, since the user input line is included
                target_size - 1
            } else {
                height
            };

            //calculate tiles
            let tile_height = height / rows;
            let tile_width = (tile_height as f32 * scale).ceil() as u32;

            let mut columns = width / tile_width;

            if border {
                //remove a bit of space for the border
                columns = columns.saturating_sub(2);
                rows = rows.saturating_sub(2);
            }

            //.max(1) is used to ensure that the values are at least 1
            //a value of 0 could cause an error (but not crash) later on
            (columns.max(1), rows.max(1), tile_width, tile_height)
        }
    }
}

#[cfg(test)]
mod test_calculate_dimensions {
    use super::*;

    #[test]
    fn calculate_dimensions_width() {
        assert_eq!(
            (100, 46, 5, 11),
            calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_dimensions_width_119() {
        assert_eq!(
            (119, 56, 4, 9),
            calculate_dimensions(119, 512, 512, 0.42, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_dimensions_height() {
        assert_eq!(
            (170, 99, 3, 5),
            calculate_dimensions(100, 512, 512, 0.42, false, ResizingDimension::Height)
        );
    }

    #[test]
    fn calculate_dimensions_height_1x1_img() {
        assert_eq!(
            (1, 1, 1, 1),
            calculate_dimensions(100, 1, 1, 0.42, false, ResizingDimension::Height)
        );
    }

    #[test]
    fn calculate_dimensions_width_1x1_img() {
        assert_eq!(
            (1, 1, 1, 2),
            calculate_dimensions(100, 1, 1, 0.42, false, ResizingDimension::Width)
        );
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_height_zero() {
        calculate_dimensions(0, 512, 512, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_width_zero() {
        calculate_dimensions(0, 512, 512, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_width_zero() {
        calculate_dimensions(100, 512, 0, 0.42, false, ResizingDimension::Width);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_height_zero() {
        calculate_dimensions(100, 0, 512, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    #[should_panic]
    fn calculate_dimensions_img_width_height_zero() {
        calculate_dimensions(100, 0, 0, 0.42, false, ResizingDimension::Height);
    }

    #[test]
    fn calculate_dimensions_scale_zero() {
        assert_eq!(
            (100, 1, 5, 4294967295),
            calculate_dimensions(100, 512, 512, 0f32, false, ResizingDimension::Width)
        );
    }

    #[test]
    fn calculate_border_smaller_columns() {
        assert_eq!(
            (98, 1, 5, 4294967295),
            calculate_dimensions(100, 512, 512, 0f32, true, ResizingDimension::Width)
        );
    }
}

///Preferred image resize direction
///
///This changes which dimensions should be used when resizing the image.
///For example, to fully use one dimension (e.g. width), the height can not be scaled
///up as well, since it already would be larger than the maximum terminal height.
///By default width will be used.
///
/// # Examples
/// ```
/// use artem::util::ResizingDimension;
///
/// assert_eq!(ResizingDimension::Width, ResizingDimension::default());
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ResizingDimension {
    Width,
    Height,
}
//Implement `Default` as Width
impl Default for ResizingDimension {
    fn default() -> Self {
        ResizingDimension::Width
    }
}

#[cfg(test)]
mod test_dimensions_enum {
    use super::*;

    #[test]
    fn default_is_width() {
        assert_eq!(ResizingDimension::Width, ResizingDimension::default());
    }
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
    let term_width = match terminal_size::terminal_size() {
        Some(value) => value.0 .0 as u32,
        None => 0,
    };
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
    let term_height = match terminal_size::terminal_size() {
        Some(value) => value.1 .0 as u32,
        None => 0,
    };
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
