//! # artem
//! `artem` is a program to convert images to ascii art.
//! While it's primary usages is through the command line, it also provides a rust crate.
//!
//! # Usage
//! To use it, load an image using the [image crate](https://crates.io/crates/image) and pass it to
//! artem. Addiontially the [`crate::convert`] function takes an [`crate::config::Config`], which can be used to configure
//! the resulting output. Whilst [`crate::config::Config`] implements [`Default`], it is
//! recommended to do the configuration through [`crate::config::ConfigBuilder`] instead.
//! ```
//! # let path = "./assets/images/standard_test_img.png";
//! let image = image::open(path).expect("Failed to open image");
//! let ascii_art = artem::convert(image, &artem::config::ConfigBuilder::new().build());
//! ```

use std::{
    fs::File,
    io::{Read, Write},
    num::NonZeroU32,
    path::{Path, PathBuf},
};

use image::{DynamicImage, ImageDecoder, ImageError, ImageReader};

use artem::config::{self, ConfigBuilder, TargetType};

mod cli;

fn main() {
    // get args from cli
    let matches = cli::build_cli().get_matches();

    // get log level from args
    // enable logging
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(
            (*matches
                .get_one::<cli::Verbosity>("verbosity")
                .unwrap_or(&cli::Verbosity::Warn))
            .into(),
        )
        .init();
    log::trace!("Started logger with trace");

    // log enabled features
    log::trace!("Feature web_image: {}", cfg!(feature = "web_image"));

    let mut config_builder = ConfigBuilder::new();

    // at least one input must exist, so its safe to unwrap
    let input = matches.get_many::<String>("INPUT").unwrap();

    let mut img_paths = Vec::with_capacity(input.len());

    log::info!("Checking inputs");
    for value in input {
        #[cfg(feature = "web_image")]
        if value.starts_with("http") {
            log::debug!("Input {} is a URL", value);
            img_paths.push(value);
            continue;
        }

        let path = Path::new(value);
        // check if file exist and is a file (not a directory)
        if !path.exists() {
            fatal_error(&format!("File {value} does not exist"), Some(66));
        } else if !Path::new(path).is_file() {
            fatal_error(&format!("{value} is not a file"), Some(66));
        }
        log::debug!("Input {} is a file", value);
        img_paths.push(value);
    }

    // density char map
    let density = match matches
        .get_one::<String>("characters")
        .map(|res| res.as_str())
    {
        Some("short") | Some("s") | Some("0") => r#"Ñ@#W$9876543210?!abc;:+=-,._ "#,
        Some("flat") | Some("f") | Some("1") => r#"MWNXK0Okxdolc:;,'...   "#,
        Some("long") | Some("l") | Some("2") => {
            r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#
        }
        Some(chars) if !chars.is_empty() => {
            log::debug!("Using user provided characters");
            chars
        }
        _ => {
            // density map from jp2a
            log::debug!("Using default characters");
            r#"MWNXK0Okxdolc:;,'...   "#
        }
    };
    log::debug!("Characters used: '{density}'");
    config_builder.characters(density.to_string());

    // set the default resizing dimension to width
    config_builder.dimension(config::ResizingDimension::Width);

    let terminal_size = |height: bool| -> u32 {
        // read terminal size, error when STDOUT is not a tty
        terminal_size::terminal_size()
            .map(|size| if height { size.1 .0 } else { size.0 .0 } as u32)
            .unwrap_or_else(|| {
                fatal_error(
                    "Failed to read terminal size, STDOUT is not a tty",
                    Some(72),
                )
            })
    };
    let height = matches.get_flag("height");
    // get target size from args
    // only one arg should be present
    let target_size = if matches.get_flag("width") || height {
        if height {
            config_builder.dimension(config::ResizingDimension::Height);
        }
        terminal_size(height)
    } else {
        // use given input size
        log::trace!("Using user input size as target size");
        *matches.get_one::<u32>("size").unwrap_or_else(|| {
            fatal_error(
                "Failed to read terminal size, STDOUT is not a tty",
                Some(72),
            )
        })
    }
    .max(20); // min should be 20 to ensure a somewhat visible picture

    log::debug!("Target Size: {target_size}");
    config_builder.target_size(NonZeroU32::new(target_size).unwrap()); // safe to unwrap, since it is clamped before

    // best ratio between height and width is 0.43
    let Some(scale) = matches.get_one::<f32>("scale").map(|scale| {
        scale.clamp(
            0.1f32, // a negative or 0 scale is not allowed
            1f32,   // even a scale above 0.43 is not looking good
        )
    }) else {
        fatal_error("Could not work with ratio input value", Some(65));
    };
    log::debug!("Scale: {scale}");
    config_builder.scale(scale);

    let invert = matches.get_flag("invert-density");
    log::debug!("Invert is set to: {invert}");
    config_builder.invert(invert);

    let background_color = matches.get_flag("background-color");
    log::debug!("BackgroundColor is set to: {background_color}");
    config_builder.background_color(background_color);

    // check if no colors should be used or the if an output file will be used
    // since text documents don`t support ansi ascii colors
    let color = if matches.get_flag("no-color") {
        // print the "normal" non-colored conversion
        log::info!("Using non-colored ascii");
        false
    } else {
        if matches.get_flag("outline") {
            log::warn!("Using outline, result will only be in grayscale");
            // still set colors to true, since grayscale has different gray tones
        }

        // print colored terminal conversion, this should already respect truecolor support/use ansi colors if not supported
        log::info!("Using colored ascii");
        if !*artem::SUPPORTS_TRUECOLOR {
            if background_color {
                log::warn!("Background flag will be ignored, since truecolor is not supported.")
            }
            log::warn!("Truecolor is not supported. Using ansi color.")
        } else {
            log::info!("Using truecolor ascii")
        }
        true
    };
    config_builder.color(color);

    // get flag for border around image
    let border = matches.get_flag("border");
    config_builder.border(border);
    log::info!("Using border: {border}");

    // get flags for flipping along x axis
    let transform_x = matches.get_flag("flipX");
    config_builder.transform_x(transform_x);
    log::debug!("Flipping X-Axis: {transform_x}");

    // get flags for flipping along y axis
    let transform_y = matches.get_flag("flipY");
    config_builder.transform_y(transform_y);
    log::debug!("Flipping Y-Axis: {transform_y}");

    // get flags for centering the image
    let center_x = matches.get_flag("centerX");
    config_builder.center_x(center_x);
    log::debug!("Centering X-Axis: {center_x}");

    let center_y = matches.get_flag("centerY");
    config_builder.center_y(center_y);
    log::debug!("Center Y-Axis: {center_y}");

    // get flag for creating an outline
    let outline = matches.get_flag("outline");
    config_builder.outline(outline);
    log::debug!("Outline: {outline}");

    // if outline is set, also check for hysteresis
    if outline {
        let hysteresis = matches.get_flag("hysteresis");
        config_builder.hysteresis(hysteresis);
        log::debug!("Hysteresis: {hysteresis}");
        if hysteresis {
            log::warn!("Using hysteresis might result in an worse looking ascii image than only using --outline")
        }
    }

    // get output file extension for specific output, default to plain text
    if let Some(output_file) = matches.get_one::<PathBuf>("output-file") {
        log::debug!("Output-file: {}", output_file.to_str().unwrap());

        // check file extension
        let file_extension = output_file.extension().and_then(std::ffi::OsStr::to_str);
        log::debug!("FileExtension: {:?}", file_extension);

        config_builder.target(match file_extension {
            Some("html") | Some("htm") => {
                log::debug!("Target: Html-File");
                TargetType::HtmlFile
            }
            Some("ansi") | Some("ans") => {
                log::debug!("Target: Ansi-File");

                // by definition ansi file must have colors, only the background color is optional
                if matches.get_flag("no-color") {
                    log::warn!("The --no-color argument conflicts with the target file type. Falling back to plain text file without colors.");
                    TargetType::File
                } else {
                    if !*artem::SUPPORTS_TRUECOLOR {
                        log::warn!("truecolor is disabled, output file will not use truecolor chars")
                    }
                    TargetType::AnsiFile
                }
            }
            Some("svg") => {
                log::debug!("Target: SVG");
                TargetType::Svg
            }
            _ => {
                log::debug!("Target: File");

                if !matches.get_flag("no-color") {
                    // warn user that output is not colored
                    log::warn!("Filetype does not support using colors. For colored output file please use either .html or .ansi files");
                }
                TargetType::File
            }
        });
    } else {
        log::debug!("Target: Shell");
        config_builder.target(TargetType::Shell);
    }

    let config = config_builder.build();
    let mut output = img_paths
        .iter()
        .map(|path| load_image(path).unwrap_or_else(|err| fatal_error(&err.to_string(), Some(66))))
        .filter(|img| img.height() != 0 || img.width() != 0)
        .map(|img| artem::convert(img, &config))
        .collect::<String>();

    // remove last linebreak, we cannot use `.trim_end()` here
    // as it may end up remove whitespace that is part of the image
    if output.ends_with('\n') {
        output.remove(output.len() - 1);
    }

    // create and write to output file
    if let Some(output_file) = matches.get_one::<PathBuf>("output-file") {
        log::info!("Writing output to output file");

        let Ok(mut file) = File::create(output_file) else {
            fatal_error("Could not create output file", Some(73));
        };

        if config.target == TargetType::Svg {
            // convert terminal text to svg
            output = anstyle_svg::Term::new().render_svg(&output);
        }

        log::trace!("Created output file");
        let Ok(bytes_count) = file.write(output.as_bytes()) else {
            fatal_error("Could not write to output file", Some(74));
        };
        log::info!("Written ascii chars to output file");
        println!("Written {} bytes to {}", bytes_count, output_file.display())
    } else {
        // print the ascii img to the terminal
        log::info!("Printing output");
        println!("{}", output);
    }
}

/// Return the image from the specified path.
///
/// Loads the image from the specified path.
/// If the path is a url and the web_image feature is enabled,
/// the image will be downloaded and opened from memory.
///
/// # Examples
/// ```
/// let image = load_image("../examples/abraham_lincoln.jpg")
/// ```
fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    #[cfg(feature = "web_image")]
    if path.starts_with("http") {
        log::info!("Started to download image from: {}", path);
        let now = std::time::Instant::now();
        let Ok(mut resp) = ureq::get(path).call() else {
            fatal_error(
                &format!("Failed to load image bytes from {}", path),
                Some(66),
            );
        };

        // get bytes of the images
        let mut bytes: Vec<u8> = Vec::new();
        resp.body_mut()
            .as_reader()
            .read_to_end(&mut bytes)
            .expect("Failed to read bytes");
        log::info!("Downloading took {:3} ms", now.elapsed().as_millis());

        log::debug!("Opening downloaded image from memory");
        return image::load_from_memory(&bytes);
    }

    log::info!("Opening image");
    let mut decoder = ImageReader::open(path)?.into_decoder()?;
    let orientation = decoder.orientation()?;
    let mut img = DynamicImage::from_decoder(decoder)?;
    img.apply_orientation(orientation);

    Ok(img)
}

/// Function for fatal errors.
///
/// A fatal error is an error, from which the program can no recover, meaning the only option left is to print
/// an error message letting the user know what went wrong. For example if a non-existing file was passed in,
/// this program can not work correctly and should print an error message and exit.
///
/// This function will print the passed in error message as well as an exit message, then it will exit the program with the exit code.
/// If non is specified, it will use exit code 1 by default.
/// A list of exit code can be found here: <https://man.freebsd.org/cgi/man.cgi?query=sysexits&manpath=FreeBSD+4.3-RELEASE>
///
/// # Examples
/// ```no_run
/// use std::fs::File;
///
/// let f = File::open("hello.txt");
/// let f = match f {
///     Ok(file) => file,
///     Err(error) => fatal_error(&error.to_string(), Some(66)),
/// };
/// ```
pub fn fatal_error(message: &str, code: Option<i32>) -> ! {
    // this function never returns, since it always exit the program
    log::error!("{}", message);
    log::error!("Artem exited with code: {}", code.unwrap_or(1));
    std::process::exit(code.unwrap_or(1));
}
