//! Regexes and matching utilities

use crate::LINE_END;
use regex::Regex;
use std::sync::LazyLock;

/// Match a LaTeX \item
pub const ITEM: &str = "\\item";

/// Match a LaTeX \begin{...}
pub const ENV_BEGIN: &str = "\\begin{";

/// Match a LaTeX \end{...}
pub const ENV_END: &str = "\\end{";

/// Acceptable LaTeX file extensions
pub const EXTENSIONS: [&str; 4] = [".tex", ".bib", ".sty", ".cls"];
/// Match a LaTeX \verb|...|
pub const VERB: &str = "\\verb|";

/// Regex matches for sectioning commands
const SPLITTING: [&str; 6] = [
    r"\\begin\{",
    r"\\end\{",
    r"\\item(?:$|[^a-zA-Z])",
    r"\\(?:sub){0,2}section\*?\{",
    r"\\chapter\*?\{",
    r"\\part\*?\{",
];

// A static `String` which is a regex to match any of [`SPLITTING_COMMANDS`].
static SPLITTING_STRING: LazyLock<String> =
    LazyLock::new(|| ["(", SPLITTING.join("|").as_str(), ")"].concat());

// Regex to match newlines
pub static RE_NEWLINES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(&format!(r"{LINE_END}{LINE_END}({LINE_END})+")).unwrap()
});

// Regex to match double spaces
pub static RE_DOUBLE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"  +").unwrap());

// Regex to match trailing new ines
pub static RE_TRAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(&format!(r" +{LINE_END}")).unwrap());

// Regex that matches splitting commands
pub static RE_SPLITTING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(SPLITTING_STRING.as_str()).unwrap());

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
