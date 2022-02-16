use clap::{App, Arg, ValueHint};

///Build the clap command-line-interface  
pub fn build_cli() -> App<'static> {
    App::new("ica")
        .version("0.2")
        .about("Solves and displays Sudoku")
        .arg(
            Arg::new("INPUT")
                .help("Path to the target image. Does NOT alter the original")
                .required(true)
                .value_hint(ValueHint::FilePath)
                .index(1),
        )
        .arg(
            Arg::new("density")
                .short('c')
                .long("characters")
                .takes_value(true)
                .value_hint(ValueHint::Other)
                .help("Change the characters that are used to display the image.
                The first character should have the highest 'density' and the last should have the least (probably a space).
                A lower detail map is recommend for smaller images."),
        )
        .arg(
            Arg::new("size")
                .short('s')
                .long("size")
                .takes_value(true)
                .default_value("80")
                .value_hint(ValueHint::Other)
                .help("Change the size of the output image. 
                The minimum size is 20, the maximum 230. Values outside of the range will be 
                ignored and changed to the nearest usable value"),
        )
        .arg(
            Arg::new("scale")
                .long("ratio")
                .takes_value(true)
                .default_value("0.43") 
                .value_hint(ValueHint::Other)
                .help("Change the ratio between height and width, since Ascii chars are a bit higher than long.
                The default value is 0.43, min is 0 and max 2. It is not recommend to change this setting."),
        )
        // .arg(
        //     Arg::new("tiles")
        //         .short('t')
        //         .long("tiles")
        //         .takes_value(true)
        //         // .value_hint(ValueHint::)
        //         .help("Change the scale of the output"),
        // )
        .arg(
            Arg::new("output-file")
                .short('o')
                .long("output")
                .takes_value(true)
                .value_hint(ValueHint::FilePath)
                .help("Output file for non-colored ascii"),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .help("Do not use color when printing the image to the terminal"),
        )
}
