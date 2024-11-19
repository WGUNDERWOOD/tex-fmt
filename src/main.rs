//! tex-fmt
//! An extremely fast LaTeX formatter written in Rust

#[warn(unused_imports)]

fn main() -> std::process::ExitCode {
    let mut output: Option<String> = None;
    let exit_code = tex_fmt_lib::run(None, &mut output);
    std::process::ExitCode::from(exit_code)
}