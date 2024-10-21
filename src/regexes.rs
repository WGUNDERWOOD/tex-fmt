//! Regexes and matching utilities

use crate::LINE_END;
use lazy_static::lazy_static;
use regex::Regex;

/// Match a LaTeX \item
pub const ITEM: &str = "\\item";
/// Match a LaTeX \begin{document}
pub const DOC_BEGIN: &str = "\\begin{document}";
/// Match a LaTeX \end{document}
pub const DOC_END: &str = "\\end{document}";
/// Match a LaTeX \begin{...}
pub const ENV_BEGIN: &str = "\\begin{";
/// Match a LaTeX \end{...}
pub const ENV_END: &str = "\\end{";
/// Acceptable LaTeX file extensions
pub const EXTENSIONS: [&str; 4] = [".tex", ".bib", ".sty", ".cls"];

/// Names of LaTeX list environments
const LISTS: [&str; 5] = [
    "itemize",
    "enumerate",
    "description",
    "inlineroman",
    "inventory",
];

/// Names of LaTeX verbatim environments
const VERBATIMS: [&str; 5] =
    ["verbatim", "Verbatim", "lstlisting", "minted", "comment"];

// Regexes
lazy_static! {
    pub static ref RE_NEWLINES: Regex =
        Regex::new(&format!(r"{LINE_END}{LINE_END}({LINE_END})+")).unwrap();
    pub static ref RE_TRAIL: Regex =
        Regex::new(&format!(r" +{LINE_END}")).unwrap();
    pub static ref VERBATIMS_BEGIN: Vec<String> = VERBATIMS
        .iter()
        .map(|l| format!("\\begin{{{l}}}"))
        .collect();
    pub static ref VERBATIMS_END: Vec<String> =
        VERBATIMS.iter().map(|l| format!("\\end{{{l}}}")).collect();
    pub static ref LISTS_BEGIN: Vec<String> =
        LISTS.iter().map(|l| format!("\\begin{{{l}}}")).collect();
    pub static ref LISTS_END: Vec<String> =
        LISTS.iter().map(|l| format!("\\end{{{l}}}")).collect();
    pub static ref RE_ENV_BEGIN_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\begin\{)").unwrap();
    pub static ref RE_ENV_END_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\end\{)").unwrap();
    pub static ref RE_ITEM_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\item)").unwrap();
    // Regex that matches any splitting command with non-whitespace
    // characters before it and catches the previous text in a group called
    // "prev" and captures the command itself and the remaining text
    // in a group called "env".
    pub static ref RE_ENV_ITEM_SHARED_LINE: Regex = Regex::new(
        r"(?x)          # Enable extended mode
        (?P<prev>\S.*?) # <prev>: captures any number of characters starting
                        # with a non-whitespace character until the start
                        # of the next group;
        (?P<env>(       # <env>: captures any LaTeX command before which the
                        # line should be split
            \\begin\{   # start of environments
            |\\end\{    # end of environments
            |\\item )   # list items (note the space before the closing bracket)
        .*)             # and any characters that follow the command
        "
    )
    .unwrap();
}
