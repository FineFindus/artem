use std::path::PathBuf;

use clap::{builder::PossibleValue, value_parser, Arg, ArgAction, Command, ValueEnum, ValueHint};

/// Get arguments from the command line.
///
/// It uses clap to build and return a [`Command`] struct, which then can be used
/// configuration.
///
/// This is a non-public module and should only be used by the binary file.
///
/// # Examples
/// ```
/// // get clap matches
/// let matches = build_cli();
/// // for example check if an arg is present
/// matches.is_present("arg");
/// ```
pub fn build_cli() -> Command {
    Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!("\n"))
        .about(clap::crate_description!())
        .arg(
            Arg::new("INPUT")
                .help(
                    if cfg!(feature = "web_image")
                    {
                        // special help message with url help 
                        "Paths or URLs to the target image. If the input is an URL, the image is downloaded and then converted. The original image is NOT altered."
                    } else {
                        // normal help text with only paths
                        "Paths to the target image. The original image is NOT altered."
                    }

                )
                .required(true)
                .value_hint(ValueHint::FilePath)
                // because of web images accept strings, which allows for URLs and files
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
                .num_args(..)
        )
        .arg(
            Arg::new("characters")
                .short('c')
                .long("characters")
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
                .value_hint(ValueHint::Other)
                // use "\" to keep this readable but still as a single line string
                .help("Change the characters that are used to display the image.\
                The first character should have the highest 'darkness' and the last should have the least (recommended to be a space ' '). \
                A lower detail map is recommend for smaller images. Included characters can be used with the argument 0 | 1 | 2. If no characters are passed in, the default set will be used."),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .value_parser(value_parser!(u32))
                .default_value("80")
                .value_hint(ValueHint::Other)
                .conflicts_with_all(["height", "width"])
                .help("Change the size of the output image. \
                The minimum size is 20. Lower values will be \
                ignored and changed to 20. This argument is conflicting with --width and --height."),
        )
        .arg(
            Arg::new("height")
                .long("height")
                .conflicts_with("width")
                .action(ArgAction::SetTrue)
                .help("Use the terminal maximum terminal height to display the image. \
                This argument is conflicting with --size and --width."),
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .action(ArgAction::SetTrue)
                .help("Use the terminal maximum terminal width to display the image. \
                This argument is conflicting with --size and --height."),
        )
        .arg(
            Arg::new("scale")
                .long("ratio")
                .value_parser(value_parser!(f32))
                .default_value("0.42")
                .value_hint(ValueHint::Other)
                .help("Change the ratio between height and width, since ASCII characters are a bit higher than long. \
                The value has to be between 0.1 and 1.0. It is not recommend to change this setting."),
        ).arg(
            Arg::new("flipX")
                .long("flipX")
                .action(ArgAction::SetTrue)
                .help("Flip the image along the X-Axis/horizontally."),
        ).arg(
            Arg::new("flipY")
                .long("flipY")
                .action(ArgAction::SetTrue)
                .help("Flip the image along the Y-Axis/vertically."),
        ).arg(
            Arg::new("centerX")
                .long("centerX")
                .action(ArgAction::SetTrue)
                .help("Center the image along the X-Axis/horizontally in the terminal."),
        ).arg(
            Arg::new("centerY")
                .long("centerY")
                .action(ArgAction::SetTrue)
                .help("Center the image along the Y-Axis/vertically in the terminal."),
        )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output")
                .value_parser(value_parser!(PathBuf))
                .value_hint(ValueHint::FilePath)
                .help("Output file for non-colored ascii. If the output file is a plaintext file, no color will be used. The use color, either use a file with an \
                .ansi extension, or an .svg/.html file, to convert the output to the respective format. \
                .ansi files will consider environment variables when creating colored output, for example when COLORTERM is not set to truecolor,\
                the resulting file will fallback to 8-bit colors."),
        )
        .arg(
            Arg::new("invert-density")
                .long("invert")
                .action(ArgAction::SetTrue)
                .help("Inverts the characters used for the image, so light characters will as dark ones. Can be useful if the image has a dark background."),
        )
        .arg(
            Arg::new("background-color")
                .long("background")
                .conflicts_with("no-color")
                .action(ArgAction::SetTrue)
                .help("Sets the background of the ascii as the color. This will be ignored if the terminal does not support truecolor. \
                This argument is mutually exclusive with the no-color argument."),
        )
        .arg(
            Arg::new("border")
                .long("border")
                .action(ArgAction::SetTrue)
                .help("Adds a decorative border surrounding the ascii image. This will make the image overall a bit smaller, \
                since it respects the user given size."),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .action(ArgAction::SetTrue)
                .help("Do not use color when printing the image to the terminal."),
        )
        .arg(
            Arg::new("outline")
                .long("outline")
                .action(ArgAction::SetTrue)
                .help("Only create an outline of the image. This uses filters, so it will take more resources/time to complete, especially on larger images. \
                It might not produce the desired output, it is advised to use this only on images with a clear distinction between foreground and background."),
        )
        .arg(
            Arg::new("hysteresis")
                .long("hysteresis")
                .alias("hys")
                .requires("outline")
                .action(ArgAction::SetTrue)
                .help("When creating the outline use the hysteresis method, which will remove imperfection, but might not be as good looking in ascii form.\
                 This will require the --outline argument to be present as well."),
        )
        .arg(
            Arg::new("verbosity")
                .long("verbose")
                .value_parser(value_parser!(Verbosity))
                .default_value("warn")
                .help("Choose the verbosity of the logging level. Warnings and errors will always be shown by default. To completely disable them, \
                use the off argument."),
        )
}
/// Verbosity enum for different logging levels.
///
/// This enum is used for accepting the `--verbose` argument with different logging levels.
///
/// This is basically a copy of the `log::LevelFilter`, with the
/// additional implemented `clap::ValueEnum`, which allows clap to parse values as this enum.
#[derive(Clone, Copy, Debug, Default)]
pub enum Verbosity {
    /// Corresponds to the `Off` log level.
    Off,
    /// Corresponds to the `Error` log level.
    Error,
    /// Corresponds to the `Warn` log level.
    #[default]
    Warn,
    /// Corresponds to the `Info` log level.
    Info,
    /// Corresponds to the `Debug` log level.
    Debug,
    /// Corresponds to the `Trace` log level.
    Trace,
}

impl ValueEnum for Verbosity {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Verbosity::Off,
            Verbosity::Error,
            Verbosity::Warn,
            Verbosity::Info,
            Verbosity::Debug,
            Verbosity::Trace,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            Verbosity::Off => PossibleValue::new("off").help("Do not show logs"),
            Verbosity::Error => PossibleValue::new("error").help("Only show errors"),
            Verbosity::Warn => PossibleValue::new("warn").help("Show errors and warnings"),
            Verbosity::Info => PossibleValue::new("info").help("Show info logs"),
            Verbosity::Debug => PossibleValue::new("debug").help("Show debug logs"),
            Verbosity::Trace => PossibleValue::new("trace").help("Show trace logs"),
        })
    }
}

impl std::fmt::Display for Verbosity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Verbosity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

impl From<Verbosity> for log::LevelFilter {
    fn from(value: Verbosity) -> Self {
        match value {
            Verbosity::Off => log::LevelFilter::Off,
            Verbosity::Error => log::LevelFilter::Error,
            Verbosity::Warn => log::LevelFilter::Warn,
            Verbosity::Info => log::LevelFilter::Info,
            Verbosity::Debug => log::LevelFilter::Debug,
            Verbosity::Trace => log::LevelFilter::Trace,
        }
    }
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
        // size and width conflict
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
        // size and height conflict
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
        // height and width conflict
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
        // height and width conflict
        let matches = build_cli().try_get_matches_from([
            "artem",
            "../example/abraham_lincoln.jpg",
            "--no-color",
            "--backgrounds",
        ]);
        assert!(matches.is_err());
    }
}
