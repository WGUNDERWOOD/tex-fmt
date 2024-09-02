use crate::LINE_END;
use lazy_static::lazy_static;
use regex::Regex;

pub const ITEM: &str = "\\item";
pub const DOC_BEGIN: &str = "\\begin{document}";
pub const DOC_END: &str = "\\end{document}";
pub const ENV_BEGIN: &str = "\\begin{";
pub const ENV_END: &str = "\\end{";

const LISTS: [&str; 5] = [
    "itemize",
    "enumerate",
    "description",
    "inlineroman",
    "inventory",
];

const VERBATIMS: [&str; 4] = ["verbatim", "Verbatim", "lstlisting", "minted"];

lazy_static! {
    pub static ref RE_NEWLINES: Regex =
        Regex::new(&format!(r"{LINE_END}{LINE_END}({LINE_END})+")).unwrap();
    pub static ref RE_TRAIL: Regex =
        Regex::new(&format!(r" +{LINE_END}")).unwrap();
    pub static ref RE_VERBATIMS_BEGIN: Vec<Regex> = VERBATIMS
        .iter()
        .map(|l| Regex::new(&format!(r"\\begin\{{{l}}}")).unwrap())
        .collect();
    pub static ref RE_VERBATIMS_END: Vec<Regex> = VERBATIMS
        .iter()
        .map(|l| Regex::new(&format!(r"\\end\{{{l}}}")).unwrap())
        .collect();
    pub static ref RE_LISTS_BEGIN: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r"\\begin\{{{l}}}")).unwrap())
        .collect();
    pub static ref RE_LISTS_END: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r"\\end\{{{l}}}")).unwrap())
        .collect();
    pub static ref RE_ENV_BEGIN_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\begin\{)").unwrap();
    pub static ref RE_ENV_END_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\end\{)").unwrap();
    pub static ref RE_ITEM_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\item)").unwrap();
}
