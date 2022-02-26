use clap_complete::{
    generate_to,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
    Generator,
};
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs, path};

use std::io::Error;

include!("src/cli.rs");
//from https://docs.rs/clap_complete/3.0.6/clap_complete/generator/fn.generate_to.html
fn main() -> Result<(), Error> {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(dir) => dir,
    };

    let mut cmd = build_cli();
    //this is only generated when the git ref changes???
    let mut shells_paths = Vec::with_capacity(5);

    shells_paths.push(generate_shell_completion(&mut cmd, &out_dir, Bash));
    shells_paths.push(generate_shell_completion(&mut cmd, &out_dir, PowerShell));
    shells_paths.push(generate_shell_completion(&mut cmd, &out_dir, Zsh));
    shells_paths.push(generate_shell_completion(&mut cmd, &out_dir, Fish));
    shells_paths.push(generate_shell_completion(&mut cmd, &out_dir, Elvish));

    //get output file location
    let project_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(dir) => path::PathBuf::from(dir),
    };

    //create a deployment/deb directory
    println!("cargo:warning=creating deployment directory");
    fs::create_dir_all(project_dir.join("deployment/deb")).expect("Failed to create project dir");

    for path in shells_paths {
        if path.is_err() {
            continue;
        }

        let unwrapped = &path.unwrap();

        let extension = match &unwrapped.extension() {
            Some(v) => v.to_str().unwrap(),
            None => "",
        };

        let completion_path = project_dir.join(format!("/deployment/deb/artem.{}", extension));
        println!(
            "cargo:warning=copying completion file to: {}",
            completion_path.to_str().unwrap()
        );

        //copy generated completion script to deployment dir
        fs::copy(&unwrapped, completion_path).expect_err("failed to copy completion script");
    }

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let man_page_path = path::PathBuf::from(out_dir).join("artem.1");

    std::fs::write(&man_page_path, buffer)?;

    println!("cargo:warning=man page is generated: {:?}", man_page_path);

    //copy man page to deployment dir
    println!(
        "cargo:warning=copying man page to: {:?}",
        project_dir.join("deployment/deb/artem.1")
    );
    fs::copy(&man_page_path, &project_dir.join("deployment/deb/artem.1"))
        .expect("failed to copy man page");

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
        shell, cmd,      // We need to specify what generator to use
        "artem",  // We need to specify the bin name manually
        &out_dir, // We need to specify where to write to
    )?;
    println!("cargo:warning=completion file is generated: {:?}", &path);
    Ok(path)
}
