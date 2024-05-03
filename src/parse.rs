use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(long, short, help = "Print to stdout, do not modify files")]
    pub print: bool,
    #[arg(
        long,
        short,
        help = "Debug mode, disable checks and do not modify files"
    )]
    pub debug: bool,
    #[arg(required = true)]
    pub filenames: Vec<String>,
}

