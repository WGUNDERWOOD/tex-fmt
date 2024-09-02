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
}
