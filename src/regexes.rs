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

/// Regex matches for non-sectioning commands that should be on a new line.
//const REQUIRE_NEW_LINE: [&str; 3] = [
    //r"\\begin\{",
    //r"\\end\{",
    //r"\\item ", // The trailing space should remain here.
//];

/// Regex matches for sectioning commands
//const SECTIONING_COMMANDS: [&str; 10] = [
    //r"\\part\{",
    //r"\\part\*\{",
    //r"\\chapter\{",
    //r"\\chapter\*\{",
    //r"\\section\{",
    //r"\\section\*\{",
    //r"\\subsection\{",
    //r"\\subsection\*\{",
    //r"\\subsubsection\{",
    //r"\\subsubsection\*\{",
//];

/// Regex matches for sectioning commands
const SPLITTING: [&str; 8] = [
    r"\\begin\{",
    r"\\end\{",
    r"\\item ", // The trailing space should remain here.
    r"\\part\*?\{",
    r"\\chapter\*?\{",
    r"\\section\*?\{",
    r"\\subsection\*?\{",
    r"\\subsubsection\*?\{",
];

// Regexes
lazy_static! {
    // A static `String` which is a valid regex to match any one of the
    // [`SECTIONING_COMMANDS`].
    //pub static ref SECTIONING_OR_GROUP: String = [
        //"(",
        //SECTIONING_COMMANDS.join("|").as_str(),
        //")"
    //].concat();
    // A Vec of string slices that combines sectioning commands with other
    // commands that need a new line.
    //pub static ref SPLITTING_COMMANDS: Vec<&'static str> = {
        //let mut v = Vec::with_capacity(
            //REQUIRE_NEW_LINE.len() + SECTIONING_COMMANDS.len(),
        //);
        //for str in REQUIRE_NEW_LINE {
            //v.push(str);
        //}
        //for str in SECTIONING_COMMANDS {
            //v.push(str);
        //}
        //v
    //};
    // A static `String` which is a valid regex to match any one of the
    // [`SPLITTING_COMMANDS`].
    pub static ref SPLITTING_OR_GROUP: String = [
        "(",
        SPLITTING.join("|").as_str(),
        ")"
    ].concat();
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
    // Regex that matches sectioning commands
    pub static ref RE_SECTIONING: Regex = Regex::new(
        SPLITTING_OR_GROUP.as_str()
    )
    .unwrap();
    // Regex that matches sectioning commands with non-whitespace characters
    // before it.
    pub static ref RE_SECTION_SHARED_LINE: Regex = Regex::new(
        [r"(\S.*?)", "(", SPLITTING_OR_GROUP.as_str(), ".*)"]
        .concat().as_str()
    )
    .unwrap();
    // Regex that matches any splitting command with non-whitespace
    // characters before it, catches the previous text in a group called
    // "prev" and captures the command itself and the remaining text
    // in a group called "env".
    pub static ref RE_ENV_ITEM_SEC_SHARED_LINE: Regex = Regex::new(
        [r"(?P<prev>\S.*?)", "(?P<env>", SPLITTING_OR_GROUP.as_str(), ".*)"]
        .concat().as_str()
    )
    .unwrap();
}
