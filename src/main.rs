//! # artem
//! artem is a small cli program written in rust to easily convert images to ascii art.
//! It uses the image-rs library to read images from different image formats.
//!
//! # Example usage
//! ```bash
//! artem examples/abraham_lincoln.jpg
//! ```

use std::{
    fs::File,
    io::Write,
    num::NonZeroU32,
    path::{Path, PathBuf},
};

use log::{debug, info, trace, warn, LevelFilter};

use artem::{
    options::{OptionBuilder, TargetType},
    util,
};

//import cli
mod cli;

fn main() {
    //get args from cli
    let matches = cli::build_cli().get_matches();

    //get log level from args
    let log_level = match matches.value_of("verbosity") {
        Some("trace") => LevelFilter::Trace,
        Some("debug") => LevelFilter::Debug,
        Some("info") => LevelFilter::Info,
        Some("warn") => LevelFilter::Warn,
        Some("error") => LevelFilter::Error,
        Some("off") => LevelFilter::Off, //explicit off, this will not even show errors when the file was not found
        _ => LevelFilter::Warn,          //always show warnings and errors
    };

    //enable logging
    env_logger::builder()
        .format_target(false)
        .format_timestamp(None)
        .filter_level(log_level)
        .init();
    trace!("Started logger with trace");

    //log enabled features
    trace!("Feature web_image: {}", cfg!(feature = "web_image"));

    let mut options_builder = OptionBuilder::new();

    //at least one input must exist, so its safe to unwrap
    let input = matches.values_of("INPUT").unwrap();

    let mut img_paths = Vec::with_capacity(input.len());

    info!("Checking inputs");
    for value in input {
        #[cfg(feature = "web_image")]
        {
            if value.starts_with("http") {
                debug!("Input {} is a URL", value);
                img_paths.push(value);
                continue;
            }
        }

        let path = Path::new(value);
        //check if file exist and is a file (not a directory)
        if !path.exists() {
            util::fatal_error(&format!("File {value} does not exist"), Some(66));
        } else if !Path::new(path).is_file() {
            util::fatal_error(&format!("{value} is not a file"), Some(66));
        }
        debug!("Input {} is a file", value);
        img_paths.push(value);
    }

    //density char map
    let density = match matches.value_of("characters") {
        Some("short") | Some("s") | Some("0") => r#"Ñ@#W$9876543210?!abc;:+=-,._ "#,
        Some("flat") | Some("f") | Some("1") => r#"MWNXK0Okxdolc:;,'...   "#,
        Some("long") | Some("l") | Some("2") => {
            r#"$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^`'. "#
        }
        Some(chars) if !chars.is_empty() => {
            info!("Using user provided characters");
            chars
        }
        _ => {
            //density map from jp2a
            info!("Using default characters");
            r#"MWNXK0Okxdolc:;,'...   "#
        }
    };
    debug!("Characters used: '{density}'");
    options_builder.characters(density.to_string());

    //set the default resizing dimension to width
    options_builder.dimension(util::ResizingDimension::Width);

    //get target size from args
    //only one arg should be present
    let target_size = if matches.is_present("height") {
        //use max terminal height
        trace!("Using terminal height as target size");
        //change dimension to height
        options_builder.dimension(util::ResizingDimension::Height);

        //read terminal size, error when STDOUT is not a tty
        let Some(height) = terminal_size::terminal_size().map(|size| size.1.0 as u32) else {
            util::fatal_error(
                "Failed to read terminal size, STDOUT is not a tty",
                Some(72),
            );
        };
        height
    } else if matches.is_present("width") {
        //use max terminal width
        trace!("Using terminal width as target size");

        //read terminal size, error when STDOUT is not a tty
        let Some(width) = terminal_size::terminal_size().map(|size| size.0.0 as u32) else {
            util::fatal_error(
                "Failed to read terminal size, STDOUT is not a tty",
                Some(72),
            );
        };
        width
    } else {
        //use given input size
        trace!("Using user input size as target size");
        let Some(size) = matches
            .value_of("size")
            .and_then(|size| size.parse::<u32>().ok()) else {
                util::fatal_error("Could not work with size input value", Some(65));
            };
        size
    }
    .clamp(
        20,  //min should be 20 to ensure a somewhat visible picture
        230, //img above 230 might not be displayed properly
    );

    debug!("Target Size: {target_size}");
    options_builder.target_size(NonZeroU32::new(target_size).unwrap()); //safe to unwrap, since it is clamped before

    //best ratio between height and width is 0.43
    let Some(scale) = matches
        .value_of("scale")
        .and_then(|scale| scale.parse::<f32>().ok())
        .map(|scale| {
            scale.clamp(
                0.1f32, //a negative or 0 scale is not allowed
                1f32,   //even a scale above 0.43 is not looking good
            )
        }) else {
        util::fatal_error("Could not work with ratio input value", Some(65));
    };
    debug!("Scale: {scale}");
    options_builder.scale(scale);

    let invert = matches.is_present("invert-density");
    debug!("Invert is set to: {invert}");
    options_builder.invert(invert);

    let background_color = matches.is_present("background-color");
    debug!("BackgroundColor is set to: {background_color}");

    //check if no colors should be used or the if a output file will be used
    //since text documents don`t support ansi ascii colors
    let color = if matches.is_present("no-color") {
        //print the "normal" non-colored conversion
        info!("Using non-colored ascii");
        false
    } else {
        if matches.is_present("outline") {
            warn!("Using outline, result will only be in grayscale");
            //still set colors  to true, since grayscale has different gray tones
        }

        //print colored terminal conversion, this should already respect truecolor support/use ansi colors if not supported
        info!("Using colored ascii");
        if !*util::SUPPORTS_TRUECOLOR {
            if background_color {
                warn!("Background flag will be ignored, since truecolor is not supported.")
            }
            warn!("Truecolor is not supported. Using ansi color.")
        } else {
            info!("Using truecolor ascii")
        }
        true
    };

    //get flag for border around image
    let border = matches.is_present("border");
    options_builder.border(border);
    info!("Using border: {border}");

    //get flags for flipping along x axis
    let transform_x = matches.is_present("flipX");
    options_builder.transform_x(transform_x);
    debug!("Flipping X-Axis: {transform_x}");

    //get flags for flipping along y axis
    let transform_y = matches.is_present("flipY");
    options_builder.transform_y(transform_y);
    debug!("Flipping Y-Axis: {transform_y}");

    //get flags for centering the image
    let center_x = matches.is_present("centerX");
    options_builder.center_x(center_x);
    debug!("Centering X-Axis: {center_x}");

    let center_y = matches.is_present("centerY");
    options_builder.center_y(center_y);
    debug!("Center Y-Axis: {center_y}");

    //get flag for creating an outline
    let outline = matches.is_present("outline");
    options_builder.outline(outline);
    debug!("Outline: {outline}");

    //if outline is set, also check for hysteresis
    if outline {
        let hysteresis = matches.is_present("hysteresis");
        options_builder.hysteresis(hysteresis);
        debug!("Hysteresis: {hysteresis}");
        if hysteresis {
            warn!("Using hysteresis might result in an worse looking ascii image than only using --outline")
        }
    }

    //get output file extension for specific output, default to plain text
    if let Some(output_file) = matches.value_of("output-file") {
        let file_path = PathBuf::from(output_file); //save to unwrap, checked before
        debug!("Output-file: {}", file_path.to_str().unwrap());

        //check file extension
        let file_extension = file_path.extension().and_then(std::ffi::OsStr::to_str);
        debug!("FileExtension: {:?}", file_extension);

        options_builder.target(match file_extension {
            Some("html") | Some("htm") => {
                debug!("Target: Html-File");
                TargetType::HtmlFile(color, background_color)
            }

            Some("ansi") | Some("ans") => {
                debug!("Target: Ansi-File");

                //by definition ansi file must have colors, only the background color is optional
                if matches.is_present("no-color") {
                    warn!("The --no-color argument conflicts with the target file type. Falling back to plain text file without colors.");
                    TargetType::File
                } else {
                    if !*util::SUPPORTS_TRUECOLOR {
                        warn!("truecolor is disabled, output file will not use truecolor chars")
                    }
                    TargetType::AnsiFile(background_color)
                }
            }
            _ => {
                debug!("Target: File");

                if !matches.is_present("no-color") {
                    //warn user that output is not colored
                    warn!("Filetype does not support using colors. For colored output file please use either .html or .ansi files");
                }
                TargetType::File
            }
        });
    } else {
        debug!("Target: Shell");
        options_builder.target(TargetType::Shell(color, background_color));
    }

    let mut output = img_paths
        .iter()
        .map(|path| load_image(path))
        .filter(|img| img.height() != 0 || img.width() != 0)
        .map(|img| artem::convert(img, options_builder.build()))
        .collect::<String>();

    //remove last linebreak, we cannot use `.trim_end()` here
    //as it may end up remove whitespace that is part of the image
    if output.ends_with('\n') {
        output.remove(output.len() - 1);
    }

    //create and write to output file
    if let Some(output_file) = matches.value_of("output-file") {
        info!("Writing output to output file");

        let Ok(mut file) = File::create(output_file) else {
            util::fatal_error("Could not create output file", Some(73));
        };

        trace!("Created output file");
        let Ok(bytes_count) = file.write(output.as_bytes()) else {
                util::fatal_error("Could not write to output file", Some(74));
        };
        info!("Written ascii chars to output file");
        println!("Written {} bytes to {}", bytes_count, output_file)
    } else {
        //print the ascii img to the terminal
        info!("Printing output");
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
fn load_image(path: &str) -> image::DynamicImage {
    #[cfg(feature = "web_image")]
    {
        if path.starts_with("http") {
            {
                info!("Started to download image from: {}", path);
                let now = std::time::Instant::now();
                let resp = minreq::get(path).send();

                //get bytes of the image
                let bytes = match resp {
                    Ok(value) => value.into_bytes(),
                    Err(_) => util::fatal_error(
                        &format!("Failed to parse image bytes from {path}"),
                        Some(66),
                    ),
                };
                info!("Downloading took {:3} ms", now.elapsed().as_millis());

                debug!("Opening downloaded image from memory");
                return match image::load_from_memory(&bytes) {
                    Ok(img) => img,
                    Err(err) => util::fatal_error(&err.to_string(), Some(66)),
                };
            }
        }
    }

    info!("Opening image");
    match image::open(path) {
        Ok(img) => img,
        Err(err) => util::fatal_error(&err.to_string(), Some(66)),
    }
}
