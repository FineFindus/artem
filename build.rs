use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::io::Error;

include!("src/cli.rs");
//from https://docs.rs/clap_complete/3.0.6/clap_complete/generator/fn.generate_to.html
fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut app = build_cli();
    let path = generate_to(
        Bash, &mut app, // We need to specify what generator to use
        "ica",    // We need to specify the bin name manually
        outdir,   // We need to specify where to write to
    )?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}
