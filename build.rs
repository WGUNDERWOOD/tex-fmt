use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use std::env::var_os;
use std::fs::create_dir;
use std::io::Error;
use std::path::Path;

include!("src/command.rs");

fn main() -> Result<(), Error> {
    let outdir = match var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => Path::new(&outdir).join("completions/"),
    };

    if !outdir.exists() {
        //println!("cargo:warning=creating completions directory");
        create_dir(&outdir).unwrap();
    }

    //println!("cargo:warning=generating completion scripts in {outdir:?}");
    let mut cmd = get_cli_command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cmd, "tex-fmt", &outdir)?;
        //println!("cargo:warning=generated completion script for {shell}");
    }

    Ok(())
}
