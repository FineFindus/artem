use clap_complete::{
    generate_to,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
    Generator,
};
use std::ffi::OsString;
use std::{env, path};

use std::io::Error;

include!("src/cli.rs");
//from https://docs.rs/clap_complete/3.0.6/clap_complete/generator/fn.generate_to.html
fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(dir) => dir,
    };

    let mut cmd = build_cli();
    //this is only generated when the git ref changes???
    generate_shell_completion(&mut cmd, &out_dir, Bash).unwrap();
    generate_shell_completion(&mut cmd, &out_dir, PowerShell).unwrap();
    generate_shell_completion(&mut cmd, &out_dir, Zsh).unwrap();
    generate_shell_completion(&mut cmd, &out_dir, Fish).unwrap();
    generate_shell_completion(&mut cmd, &out_dir, Elvish).unwrap();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_page_path = path::PathBuf::from(out_dir).join("artem.1");

    std::fs::write(&man_page_path, buffer)?;

    println!("cargo:warning=man page is generated: {:?}", man_page_path);

    Ok(())
}

fn generate_shell_completion<T>(
    cmd: &mut Command,
    out_dir: &OsString,
    shell: T,
) -> Result<PathBuf, Error>
where
    T: Generator,
{
    //generate shell completions
    let path = generate_to(
        shell, cmd,     // We need to specify what generator to use
        "artem", // We need to specify the bin name manually
        out_dir, // We need to specify where to write to
    )?;
    println!("cargo:warning=completion file is generated: {:?}", &path);
    Ok(path)
}
