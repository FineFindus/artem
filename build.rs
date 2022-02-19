use clap_complete::{generate_to, shells::Bash};
use std::env;

use std::io::Error;

include!("src/cli.rs");
//from https://docs.rs/clap_complete/3.0.6/clap_complete/generator/fn.generate_to.html
fn main() -> Result<(), Error> {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = build_cli();
    let path = generate_to(
        Bash, &mut cmd, // We need to specify what generator to use
        "artem",  // We need to specify the bin name manually
        &out_dir, // We need to specify where to write to
    )?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_page_path = std::path::PathBuf::from(out_dir).join("artem.1");

    std::fs::write(&man_page_path, buffer)?;

    println!(
        "cargo:warning=completion file is generated: {:?}",
        man_page_path
    );

    Ok(())
}
