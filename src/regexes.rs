use lazy_static::lazy_static;
use regex::Regex;

const LISTS: [&str; 4] = ["itemize", "enumerate", "description", "inlineroman"];

lazy_static! {
    pub static ref RE_NEWLINES: Regex = Regex::new(r"\n\n\n+").unwrap();
    pub static ref RE_TABS: Regex = Regex::new(r"\t").unwrap();
    pub static ref RE_TRAIL: Regex = Regex::new(r" +\n").unwrap();
    pub static ref RE_ITEM: Regex = Regex::new(r"\\item").unwrap();
    pub static ref RE_DOCUMENT_BEGIN: Regex =
        Regex::new(r"\\begin\{document\}").unwrap();
    pub static ref RE_DOCUMENT_END: Regex =
        Regex::new(r"\\end\{document\}").unwrap();
    pub static ref RE_VERBATIM_BEGIN: Regex =
        Regex::new(r"\\begin\{verbatim\}").unwrap();
    pub static ref RE_VERBATIM_END: Regex =
        Regex::new(r"\\end\{verbatim\}").unwrap();
    pub static ref RE_ENV_BEGIN: Regex =
        Regex::new(r"\\begin\{").unwrap();
    pub static ref RE_ENV_END: Regex =
        Regex::new(r"\\end\{").unwrap();
    pub static ref RE_LISTS_BEGIN: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r"\\begin\{{{}}}", l)).unwrap())
        .collect();
    pub static ref RE_LISTS_END: Vec<Regex> = LISTS
        .iter()
        .map(|l| Regex::new(&format!(r"\\end\{{{}}}", l)).unwrap())
        .collect();
    pub static ref RE_ENV_BEGIN_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\begin\{)").unwrap();
    pub static ref RE_ENV_END_SHARED_LINE: Regex =
        Regex::new(r"(?P<prev>\S.*?)(?P<env>\\end\{)").unwrap();
}
