use clap::{Arg, Command, ValueHint};

/// Get arguments from the command line.
///
/// It uses clap to build and return a [`Command`] struct, which then can be used
/// configuration.
///
/// This is a non-public module and should only be used by the binary file.
///
/// # Examples
/// ```
/// //get clap matches
/// let matches = build_cli();
/// //for example check if an arg is present
/// matches.is_present("arg");
/// ```
pub fn build_cli() -> Command<'static> {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(
            Arg::new("INPUT")
                .help(
                    if cfg!(feature = "web_image")
                    {
                        //special help message with url help 
                        "Paths or URLs to the target image. If the input is an URL, the image is downloaded and then converted. The original image is NOT altered."
                    } else {
                        //normal help text with only paths
                        "Paths to the target image. The original image is NOT altered."
                    }

                )
                .required(true)
                .multiple_values(true)
                .value_hint(ValueHint::FilePath)
        )
        .arg(
            Arg::new("characters")
                .short('c')
                .long("characters")
                .takes_value(true)
                .value_hint(ValueHint::Other)
                //use "\" to keep this readable but still as a single line string
                .help("Change the characters that are used to display the image.\
                The first character should have the highest 'darkness' and the last should have the least (recommended to be a space ' '). \
                A lower detail map is recommend for smaller images. Included characters can be used with the argument 0 | 1 | 2."),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .default_value("80")
                .value_hint(ValueHint::Other)
                .conflicts_with_all(&["height", "width"])
                .help("Change the size of the output image. \
                The minimum size is 20, the maximum 230. Values outside of the range will be \
                ignored and changed to the nearest usable value. This argument is conflicting with --width and --height."),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .conflicts_with("width")
                .help("Use the terminal maximum terminal height to display the image. \
                This argument is conflicting with --size and --width."),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .help("Use the terminal maximum terminal width to display the image. \
                This argument is conflicting with --size and --height."),
        )
        .arg(
            Arg::new("scale")
                .long("ratio")
                .takes_value(true)
                .default_value("0.42")
                .value_hint(ValueHint::Other)
                .help("Change the ratio between height and width, since ASCII characters are a bit higher than long. \
                The value has to be between 0.1 and 1.0. It is not recommend to change this setting."),
        ).arg(
            Arg::new("flipX")
                .long("flipX")
                .help("Flip the image along the X-Axis/horizontally."),
        ).arg(
            Arg::new("flipY")
                .long("flipY")
                .help("Flip the image along the Y-Axis/vertically."),
        ).arg(
            Arg::new("centerX")
            .long("centerX")
            .help("Center the image along the X-Axis/horizontally in the terminal."),
        ).arg(
            Arg::new("centerY")
                .long("centerY")
                .help("Center the image along the Y-Axis/vertically in the terminal."),
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .help("Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an \
                .ansi extension, or an .html file, to convert the output to html. \
                .ansi files will consider environment variables when creating colored output, for example when COLORTERM is not set to truecolor,\
                the resulting file will fallback to 8-bit colors."),
        )
        .arg(
            Arg::new("invert-density")
                .long("invert")
                .help("Inverts the characters used for the image, so light characters will as dark ones. Can be useful if the image has a dark background."),
        )
        .arg(
            Arg::new("background-color")
                .long("background")
                .conflicts_with("no-color")
                .help("Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. \
                This argument is mutually exclusive with the no-color argument."),
        )
        .arg(
            Arg::new("border")
                .long("border")
                .help("Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, \
                since it respects the user given size."),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Do not use color when printing the image to the terminal."),
        )
        .arg(
            Arg::new("outline")
                .long("outline")
                .help("Only create an outline of the image. This uses filters, so it will take more resources/time to complete, especially on larger images. \
                It might not produce the desired output, it is advised to use this only on images with a clear distinction between foreground and background."),
        )
        .arg(
            Arg::new("hysteresis")
                .long("hysteresis")
                .alias("hys")
                .requires("outline")
                .help("When creating the outline use the hysteresis method, which will remove imperfection, but might not be as good looking in ascii form.\
                 This will require the --outline argument to be present as well."),
        )
        .arg(
            Arg::new("verbosity")
                .long("verbose")
                .takes_value(true)
                .possible_values(["trace", "debug", "info", "warn", "error", "off"])
                .help("Choose the verbosity of the logging level. Warnings and errors will always be shown by default. To completely disable them, \
                use the off argument."),
        )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn fail_missing_input() {
        let matches = build_cli().try_get_matches_from(["artem"]);
        assert!(matches.is_err());
    }

    #[test]
    fn success_input() {
        let matches = build_cli().try_get_matches_from(["artem", "../example/abraham_lincoln.jpg"]);
        assert!(matches.is_ok());
    }

    #[test]
    fn fail_conflicting_args_size_width() {
        //size and width conflict
        let matches = build_cli().try_get_matches_from([
            "artem",
            "../example/abraham_lincoln.jpg",
            "-s 20",
            "-w",
        ]);
        assert!(matches.is_err());
    }

    #[test]
    fn fail_conflicting_args_size_height() {
        //size and height conflict
        let matches = build_cli().try_get_matches_from([
            "artem",
            "../example/abraham_lincoln.jpg",
            "-s 20",
            "-h",
        ]);
        assert!(matches.is_err());
    }

    #[test]
    fn fail_conflicting_args_height_width() {
        //height and width conflict
        let matches = build_cli().try_get_matches_from([
            "artem",
            "../example/abraham_lincoln.jpg",
            "-h",
            "-w",
        ]);
        assert!(matches.is_err());
    }

    #[test]
    fn fail_conflicting_args_no_color_background() {
        //height and width conflict
        let matches = build_cli().try_get_matches_from([
            "artem",
            "../example/abraham_lincoln.jpg",
            "--no-color",
            "--backgrounds",
        ]);
        assert!(matches.is_err());
    }
}
