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
                .help("Change the characters that are used to display the image.
                The first character should have the highest 'density' and the last should have the least (probably a space).
                A lower detail map is recommend for smaller images. "),
        )
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
