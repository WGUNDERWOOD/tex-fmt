use clap::Parser;

const EXTENSIONS: [&str; 4] = [".tex", ".bib", ".sty", ".cls"];

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    #[arg(long, short, help = "Check formatting, do not modify files")]
    pub check: bool,
    #[arg(long, short, help = "Print to STDOUT, do not modify files")]
    pub print: bool,
    #[arg(long, short, help = "Show info log messages")]
    pub verbose: bool,
    #[arg(long, short, help = "Show trace log messages")]
    pub trace: bool,
    #[arg(required = true)]
    pub files: Vec<String>,
}

impl Cli {
    pub fn resolve(&mut self) {
        if self.trace {
            self.verbose = true;
        }
    }

    #[cfg(test)]
    pub fn new() -> Self {
        Cli {
            check: false,
            print: false,
            verbose: false,
            trace: false,
            files: Vec::<String>::new(),
        }
    }
}

pub fn check_extension_valid(file: &str) -> bool {
    EXTENSIONS.iter().any(|e| file.ends_with(e))
}
