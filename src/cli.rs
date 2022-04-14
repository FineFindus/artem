use clap::{Arg, Command, ValueHint};

/// Get arguments from the command line.
///
/// It uses clap to build and return a [Command] struct, which then can be used
/// configuration.
///
/// # Examples
/// ```
/// //get clap matches
/// let matches = build_cli();
/// //for example check if an arg is present
/// matches.is_present("arg");
/// ```
///   
pub fn build_cli() -> Command<'static> {
    Command::new("artem")
        .version("0.6.1")
        .about(
            "artem is a small cli program written in rust to easily convert images to ascii art.",
        )
        .arg(
            Arg::new("INPUT")
                .help("Path to the target image. Does NOT alter the original image")
                .required(true)
                .value_hint(ValueHint::FilePath)
        )
        .arg(
            Arg::new("density")
                .short('c')
                .long("characters")
                .takes_value(true)
                .value_hint(ValueHint::Other)
                //use "\" to keep this readable but still as a single line string
                .help("Change the characters that are used to display the image.\
                The first character should have the highest 'darkness' and the last should have the least (recommended to be the space character ' ').\
                A lower detail map is recommend for smaller images."),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .default_value("80")
                .value_hint(ValueHint::Other)
                .conflicts_with_all(&["height", "width"])
                .help("Change the size of the output image.\
                The minimum size is 20, the maximum 230. Values outside of the range will be\
                ignored and changed to the nearest usable value. This argument is conflicting with --width and --height"),
        )
        .arg(
            Arg::new("height")
                .short('h')
                .long("height")
                .conflicts_with("width")
                .help("Use the terminal maximum terminal height to display the image.\
                This argument is conflicting with --size and --width "),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .help("Use the terminal maximum terminal height to display the image.\
                This argument is conflicting with --size and --height "),
        )
        .arg(
            Arg::new("scale")
                .long("ratio")
                .takes_value(true)
                .default_value("0.42")
                .value_hint(ValueHint::Other)
                .help("Change the ratio between height and width, since Ascii chars are a bit higher than long.\
                The default value is 0.43, min is 0 and max 2. It is not recommend to change this setting."),
        ).arg(
            Arg::new("flipX")
                .long("flipX")
                .help("Flip the image along the X axis"),
        ).arg(
            Arg::new("flipY")
                .long("flipY")
                .help("Flip the image along the Y axis"),
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .help("Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an .ansi extension, or an .html file, to convert the output to html."),
        )
        .arg(
            Arg::new("threads")
                .long("thread")
                .takes_value(true)
                .default_value("4")
                .value_hint(ValueHint::Other)
                .help("OutputNumber of threads used to convert the image. A larger number can lead to grater performance. Defaults to 4"),
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
                .help("Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. This argument is mutually exclusive with the no-color argument."),
        )
        .arg(
            Arg::new("border")
                .long("border")
                .help("Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, since it respects the user given size."),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Do not use color when printing the image to the terminal."),
        )
        .arg(
            Arg::new("outline")
                .long("outline")
                .help("Only create an outline of the image. This uses filters, so it will take more resources/time to complete, especially on larger images. It might not produce the desired output, it is advised to use this only on images with a clear distinction between foreground and background"),
        )
        .arg(
            Arg::new("verbosity")
                .long("verbose")
                .takes_value(false)
                .possible_values(["trace", "debug", "info", "warn", "error"])
                .help("Choose the verbosity of the logging level."),
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
