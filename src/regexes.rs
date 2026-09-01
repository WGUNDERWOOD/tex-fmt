//! Regexes and matching utilities

use crate::LINE_END;
use regex::{Regex, RegexSet};
use std::sync::LazyLock;

/// Match a LaTeX \item
pub const ITEM: &str = "\\item";

/// Match a LaTeX \begin{...}
pub const ENV_BEGIN: &str = "\\begin{";

/// Match a LaTeX \end{...}
pub const ENV_END: &str = "\\end{";

/// Acceptable LaTeX file extensions
pub const EXTENSIONS: [&str; 4] = ["tex", "bib", "sty", "cls"];

/// Match a LaTeX \verb|...|
pub const VERBS: [&str; 3] = ["\\verb|", "\\verb+", "\\mintinline"];

/// Regex matches for sectioning commands
const SPLITTING: [&str; 6] = [
    r"\\begin\{",
    r"\\end\{",
    r"\\item(?:$|[^a-zA-Z])",
    r"\\(?:sub){0,2}section\*?\{",
    r"\\chapter\*?\{",
    r"\\part\*?\{",
];

/// Match table commands
const TABLES: [&str; 4] = ["tabular", "tabularx", "longtable", "xltabular"];
pub static TABLES_BEGIN: LazyLock<[String; 4]> = LazyLock::new(|| {
    std::array::from_fn(|s| format!("\\begin{{{}}}", TABLES[s]))
});
pub static TABLES_END: LazyLock<[String; 4]> = LazyLock::new(|| {
    std::array::from_fn(|s| format!("\\end{{{}}}", TABLES[s]))
});

// A static `String` which is a regex to match any of [`SPLITTING_COMMANDS`].
static SPLITTING_STRING: LazyLock<String> =
    LazyLock::new(|| ["(", SPLITTING.join("|").as_str(), ")"].concat());

// Regex to match newlines
pub static RE_NEWLINES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"{LINE_END}{LINE_END}({LINE_END})+")).unwrap()
});

// Regex to match trailing new ines
pub static RE_TRAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r" +{LINE_END}")).unwrap());

// Regex that matches splitting commands
pub static RE_SPLITTING: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new(SPLITTING).unwrap());

// Matches splitting commands with non-whitespace characters before it.
pub static RE_SPLITTING_SHARED_LINE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        [r"(:?\S.*?)", "(:?", SPLITTING_STRING.as_str(), ".*)"]
            .concat()
            .as_str(),
    )
    .unwrap()
});

// Matches any splitting command with non-whitespace
// characters before it, catches the previous text in a group called
// "prev" and captures the command itself and the remaining text
// in a group called "env".
pub static RE_SPLITTING_SHARED_LINE_CAPTURE: LazyLock<Regex> =
    LazyLock::new(|| {
        Regex::new(
            [
                r"(?P<prev>\S.*?)",
                "(?P<env>",
                SPLITTING_STRING.as_str(),
                ".*)",
            ]
            .concat()
            .as_str(),
        )
        .unwrap()
    });

// Regex to match a non-space followed by 2+ spaces
pub static RE_TABLE_DOUBLE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(\S) {2,}").unwrap());

// Regex to match "\\" followed by a space and more non-whitespace text on
// the same line (i.e. a line break with content after it worth moving down)
pub static RE_TABLE_LINE_BREAK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\\\\ .*\S").unwrap());

// Regex to match a line's leading whitespace plus its first non-whitespace
// character, so the indent alone can be sliced off by the caller
pub static RE_TABLE_INDENT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*\S").unwrap());

// Regex to match from the first non-whitespace character onward
pub static RE_TABLE_FIRST_NON_WHITE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\S.*").unwrap());

// Regex to match up to and including the first "\\" on a line
pub static RE_TABLE_TO_BREAK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[^\\]*\\\\").unwrap());
