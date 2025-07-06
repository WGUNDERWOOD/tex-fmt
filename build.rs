use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use std::env::var_os;
use std::fs::create_dir;
use std::io::Error;
use std::path::Path;

include!("src/command.rs");

fn main() -> Result<(), Error> {
    println!("cargo::rerun-if-changed=src/");
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=Cargo.toml");
    if std::env::var("CARGO_FEATURE_SHELLINSTALL").is_ok() {
        println!("cargo::warning=shellinstall");
        build_completion()?;
        build_man()?;
    }
    Ok(())
}

fn build_completion() -> Result<(), Error> {
    let outdir = match var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => Path::new(&outdir).join("completion/"),
    };

    if !outdir.exists() {
        create_dir(&outdir).unwrap();
    }

    let mut command = get_cli_command();
    for &shell in Shell::value_variants() {
        generate_to(shell, &mut command, "tex-fmt", &outdir)?;
    }
    Ok(())
}

fn build_man() -> Result<(), Error> {
    let outdir = match var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => Path::new(&outdir).join("man/"),
    };

    if !outdir.exists() {
        create_dir(&outdir).unwrap();
    }

    let command = get_cli_command();
    let man = clap_mangen::Man::new(command);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(outdir.join("tex-fmt.1"), buffer)?;
    Ok(())
}
