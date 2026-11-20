//! Conservative, linear-time sentence layout for prose. Only used when enabled.

use crate::args::Args;
use crate::format::{Pattern, State};
use crate::ignore::get_ignore;
use crate::verbatim::get_verbatim;
use std::collections::{HashMap, HashSet};
use std::path::Path;

// Built-in LaTeX syntax and English abbreviation roles.
#[derive(Clone, Copy)]
enum Role {
    Prefix,
    Number,
    Reference,
    Contextual,
}
const ABBREVIATIONS: &[(&str, Role)] = &[
    ("no", Role::Number),
    ("dr", Role::Prefix),
    ("mr", Role::Prefix),
    ("mrs", Role::Prefix),
    ("ms", Role::Prefix),
    ("prof", Role::Prefix),
    ("sr", Role::Prefix),
    ("jr", Role::Prefix),
    ("st", Role::Prefix),
    ("cf", Role::Prefix),
    ("vs", Role::Prefix),
    ("fig", Role::Reference),
    ("eq", Role::Reference),
    ("sec", Role::Reference),
    ("ch", Role::Reference),
    ("pp", Role::Reference),
    ("vol", Role::Reference),
    ("thm", Role::Reference),
    ("lem", Role::Reference),
    ("prop", Role::Reference),
    ("def", Role::Reference),
    ("cor", Role::Reference),
    ("tab", Role::Reference),
    ("ref", Role::Reference),
    ("etc", Role::Contextual),
    ("al", Role::Contextual),
];
const INLINE_VERBATIMS: &[&str] = &["verb", "lstinline", "mintinline"];
const THEOREM_DECLARATIONS: &[&str] = &["newtheorem", "declaretheorem"];
const MACRO_DECLARATIONS: &[&str] = &[
    "newcommand",
    "renewcommand",
    "providecommand",
    "DeclareRobustCommand",
    "newenvironment",
    "renewenvironment",
];
const DIRECTIVES: &[&str] = &[
    "par",
    "newline",
    "linebreak",
    "section",
    "subsection",
    "subsubsection",
    "chapter",
    "part",
    "paragraph",
    "subparagraph",
    // These can occur without # or @ between unfinished prose lines.
    "input",
    "include",
    "catcode",
    "def",
    "gdef",
    "edef",
    "xdef",
];
// All three classes end prose runs. Declarations also have syntax that
// the scanner needs to recognize.
const BOUNDARY_COMMANDS: &[&[&str]] =
    &[DIRECTIVES, MACRO_DECLARATIONS, THEOREM_DECLARATIONS];
const PROSE_ENVIRONMENTS: &[&str] = &[
    "document",
    "abstract",
    "quote",
    "quotation",
    "center",
    "flushleft",
    "flushright",
    "proof",
    "theorem",
    "lemma",
    "proposition",
    "corollary",
    "definition",
    "remark",
    "example",
];

pub(crate) struct LayoutLine {
    pub number: usize,
    pub text: String,
    pub rewritten: bool,
    protected: bool,
}

/// Apply the ordinary structural split points to untouched lines. Reflow the
/// exposed fragments together so the first pass already sees their boundaries.
pub(crate) fn reflow(text: &str, args: &Args) -> Vec<LayoutLine> {
    let lines = layout(text, args);
    let mut divisions = vec![];
    for (i, line) in lines.iter().enumerate() {
        if line.rewritten || line.protected || !line.text.contains('\\') {
            continue;
        }
        let content = &line.text;
        let limit = if content.contains('%') {
            crate::comments::find_comment_index(content, &Pattern::new(content))
                .unwrap_or(content.len())
        } else {
            content.len()
        };
        let mut start = 0;
        for m in crate::regexes::SPLIT_COMMAND.find_iter(&content[..limit]) {
            let prefix = &content[start..m.start()];
            if prefix.ends_with(char::is_whitespace)
                && !prefix.trim().is_empty()
            {
                divisions.push((i, m.start()));
                start = m.start();
            }
        }
    }
    if divisions.is_empty() {
        return lines;
    }
    let mut divisions = divisions.into_iter().peekable();
    let mut normalized = String::with_capacity(text.len());
    let mut origins = vec![];
    for (i, line) in lines.iter().enumerate() {
        let mut start = 0;
        while let Some((_, at)) = divisions.next_if(|&(index, _)| index == i) {
            normalized.push_str(line.text[start..at].trim_end());
            normalized.push('\n');
            origins.push(line.number);
            start = at;
        }
        normalized.push_str(&line.text[start..]);
        normalized.push('\n');
        origins.push(line.number);
    }
    let mut result = layout(&normalized, args);
    for line in &mut result {
        line.number = origins[line.number - 1];
    }
    result
}

/// Reflow safe prose, retaining the original starting line for diagnostics.
/// Protected or uncertain lines remain separate and are passed through intact.
fn layout(text: &str, args: &Args) -> Vec<LayoutLine> {
    let abbreviations = args
        .sentence_abbreviations
        .iter()
        .map(|s| s.trim().trim_end_matches('.').to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();
    let lines = source_lines(text, args);
    let syntax = DocumentSyntax::discover(text, &lines);
    let mut scanner = Scanner::default();
    let mut breaks = vec![];
    let scans: Vec<_> = lines
        .iter()
        .map(|line| scanner.scan(line, args, &syntax, &mut breaks))
        .collect();
    let mut output = vec![];
    let mut pending = Prose::default();
    for (i, (line, scan)) in lines.iter().zip(&scans).enumerate() {
        let trimmed = line.text.trim();
        if command_name(trimmed) == Some("item") {
            pending.flush(&abbreviations, &mut output);
        }
        let continues = !scan.has_prose
            && !pending.text.is_empty()
            && !ends_in_terminal(&pending.text)
            && scans
                .get(i + 1)
                .is_some_and(|next| next.safe && next.has_prose)
            && command_name(lines[i + 1].text.trim_start()) != Some("item");
        if !scan.safe || trimmed.is_empty() || (!scan.has_prose && !continues) {
            pending.flush(&abbreviations, &mut output);
            output.push((line.number, line.text.to_string()));
        } else {
            let argument = starts_argument(
                trimmed,
                line.offset + line.text.len() - line.text.trim_start().len(),
                &syntax,
            );
            pending.push(
                line.text,
                line.number,
                &breaks[scan.start..scan.stop],
                argument,
            );
        }
    }
    pending.flush(&abbreviations, &mut output);
    output
        .into_iter()
        .map(|(number, text)| {
            let source = &lines[number - 1];
            let rewritten = source.text.trim() != text.trim();
            LayoutLine {
                number,
                text,
                rewritten,
                protected: !source.active || scans[number - 1].literal,
            }
        })
        .collect()
}

struct SourceLine<'a> {
    text: &'a str,
    number: usize,
    offset: usize,
    active: bool,
}

fn source_lines<'a>(text: &'a str, args: &Args) -> Vec<SourceLine<'a>> {
    let mut state = State::new();
    let mut logs = vec![];
    let begins: Vec<_> = args
        .verbatims
        .iter()
        .map(|s| format!("\\begin{{{s}}}"))
        .collect();
    let ends: Vec<_> = args
        .verbatims
        .iter()
        .map(|s| format!("\\end{{{s}}}"))
        .collect();
    let mut offset = 0;
    text.split_inclusive('\n')
        .enumerate()
        .map(|(index, raw)| {
            let line = raw.strip_suffix('\n').unwrap_or(raw);
            let line = line.strip_suffix('\r').unwrap_or(line);
            state.ignore =
                get_ignore(line, &state, &mut logs, Path::new(""), false);
            if line.contains(r"\begin") || line.contains(r"\end") {
                state.verbatim = get_verbatim(
                    line,
                    &state,
                    &mut logs,
                    Path::new(""),
                    false,
                    &Pattern::new(line),
                    &begins,
                    &ends,
                );
            } else {
                state.verbatim.visual = state.verbatim.actual > 0;
            }
            let result = SourceLine {
                text: line,
                number: index + 1,
                offset,
                active: !state.ignore.visual && !state.verbatim.visual,
            };
            offset += raw.len();
            result
        })
        .collect()
}

#[derive(Default)]
struct ScannedLine {
    safe: bool,
    start: usize,
    stop: usize,
    has_prose: bool,
    literal: bool,
}

/// Source-derived syntax facts. Each bracket is queued and resolved at most
/// once, including when many ordinary brackets precede the same closing one.
#[derive(Default)]
struct DocumentSyntax {
    optional_ends: HashMap<usize, usize>,
    theorems: HashSet<String>,
}

impl DocumentSyntax {
    fn discover(text: &str, lines: &[SourceLine<'_>]) -> Self {
        let mut result = Self::default();
        if !text.contains('[')
            && !THEOREM_DECLARATIONS.iter().any(|name| text.contains(name))
        {
            return result;
        }
        let mut pending = Vec::<(usize, usize)>::new();
        let mut depth = 0usize;
        let mut declarations = vec![];
        for source in lines {
            if !source.active || source.text.trim().is_empty() {
                pending.clear();
                continue;
            }
            let line = source.text;
            let mut i = 0;
            // All syntax bytes here are ASCII. Non-ASCII bytes can be
            // skipped individually since only ASCII delimiters start slices.
            while let Some(&c) = line.as_bytes().get(i) {
                if c == b'%' {
                    break;
                }
                if c == b'\\' {
                    let rest = &line[i..];
                    let name = command_name(rest).unwrap_or("");
                    if INLINE_VERBATIMS.contains(&name) {
                        i += inline_code_end(rest, name).unwrap_or(rest.len());
                        continue;
                    }
                    if depth == 0 && THEOREM_DECLARATIONS.contains(&name) {
                        declarations.push(source.offset + i + 1 + name.len());
                    }
                    i += 1 + if name.is_empty() {
                        rest.chars().nth(1).map_or(0, char::len_utf8)
                    } else {
                        name.len()
                    };
                    continue;
                }
                match c {
                    b'{' => depth += 1,
                    b'}' => {
                        while pending.last().is_some_and(|&(d, _)| d == depth) {
                            pending.pop();
                        }
                        depth = depth.saturating_sub(1);
                    }
                    b'[' => pending.push((depth, source.offset + i)),
                    b']' => {
                        while let Some(&(d, open)) = pending.last() {
                            if d != depth {
                                break;
                            }
                            pending.pop();
                            result
                                .optional_ends
                                .insert(open, source.offset + i);
                        }
                    }
                    _ => {}
                }
                i += 1;
            }
        }
        let mut used_options = HashSet::new();
        for mut i in declarations {
            if text[i..].starts_with('*') {
                i += 1;
            }
            i = skip_space(text, i);
            if text[i..].starts_with('[') {
                let Some(&end) = result.optional_ends.get(&i) else {
                    continue;
                };
                // Malformed declarations can share a closing bracket.
                // Inspect the following name only once.
                if !used_options.insert(end) {
                    continue;
                }
                i = skip_space(text, end + 1);
            }
            if let Some((name, _)) = environment_name(&text[i..]) {
                result.theorems.insert(name.to_owned());
            }
        }
        result
    }
}

/// Stop at the first invalid character even in a malformed declaration.
fn environment_name(text: &str) -> Option<(&str, &str)> {
    let body = text.strip_prefix('{')?;
    let end = body.find(|c: char| {
        c.is_whitespace() || matches!(c, '{' | '}' | '\\' | '%')
    })?;
    let rest = body[end..].strip_prefix('}')?;
    (end > 0).then_some((&body[..end], rest))
}

fn skip_space(text: &str, start: usize) -> usize {
    start + text[start..].len() - text[start..].trim_start().len()
}

fn starts_argument(text: &str, offset: usize, syntax: &DocumentSyntax) -> bool {
    text.starts_with('{')
        || (text.starts_with('[') && syntax.optional_ends.contains_key(&offset))
}

/// Inspect only the final token, not an ever-growing paragraph prefix.
fn ends_in_control_word(text: &str) -> bool {
    let mut chars = text.chars().rev();
    let mut letters = false;
    for c in chars.by_ref() {
        if c.is_ascii_alphabetic() {
            letters = true;
        } else if c == '\\' && letters {
            return (1 + chars.take_while(|&c| c == '\\').count()) % 2 == 1;
        } else {
            return false;
        }
    }
    false
}

fn ends_in_terminal(text: &str) -> bool {
    text.trim_end()
        .trim_end_matches(['\'', '"', '’', '”', ')', ']'])
        .ends_with(['.', '!', '?'])
}

/// Buffer one safe prose run so decisions can see the following token even
/// when it was authored on another line. Source offsets retain diagnostics.
#[derive(Default)]
struct Prose {
    text: String,
    ends: Vec<usize>,
    origins: Vec<(usize, usize)>,
}

impl Prose {
    fn push(
        &mut self,
        line: &str,
        number: usize,
        ends: &[usize],
        argument: bool,
    ) {
        let trimmed = line.trim();
        let leading = line.len() - line.trim_start().len();
        if !(self.text.is_empty()
            || argument && ends_in_control_word(&self.text))
        {
            self.text.push(' ');
        }
        let offset = self.text.len();
        self.origins.push((offset, number));
        self.text.push_str(trimmed);
        self.ends
            .extend(ends.iter().map(|end| offset + end - leading));
    }

    fn flush(
        &mut self,
        abbreviations: &HashSet<String>,
        output: &mut Vec<(usize, String)>,
    ) {
        let mut start = 0;
        let mut origin = 0;
        for end in self
            .ends
            .iter()
            .copied()
            .chain(std::iter::once(self.text.len()))
        {
            if end != self.text.len()
                && !sentence_end(&self.text, end, abbreviations)
            {
                continue;
            }
            let fragment = &self.text[start..end];
            let first = start + fragment.len() - fragment.trim_start().len();
            let trimmed = fragment.trim();
            if !trimmed.is_empty() {
                while origin + 1 < self.origins.len()
                    && self.origins[origin + 1].0 <= first
                {
                    origin += 1;
                }
                output.push((self.origins[origin].1, trimmed.to_string()));
            }
            start = end;
        }
        self.text.clear();
        self.ends.clear();
        self.origins.clear();
    }
}

fn command_name(text: &str) -> Option<&str> {
    let rest = text.strip_prefix('\\')?;
    let end = rest
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(rest.len());
    Some(&rest[..end])
}

#[derive(Default)]
struct Scanner {
    groups: Vec<char>,
    // Only a bracket following a command (or another optional argument)
    // opens an optional argument. Ordinary prose such as [0,1) is not a group.
    optional_argument: bool,
    // Declarations such as \newcommand allow optional arguments after the
    // first braced argument (the command name), unlike ordinary prose macros.
    declaration_name_depth: Option<usize>,
    math: Option<&'static str>,
    environments: Vec<(String, bool)>,
    protected_environments: usize,
    expl3: bool,
}

impl Scanner {
    /// Scan even lines which cannot be reflowed, so multiline contexts persist.
    #[allow(clippy::too_many_lines)]
    fn scan(
        &mut self,
        source: &SourceLine,
        args: &Args,
        syntax: &DocumentSyntax,
        breaks: &mut Vec<usize>,
    ) -> ScannedLine {
        if !source.active {
            return ScannedLine::default();
        }
        let line = source.text;
        let first_token = line.len() - line.trim_start().len();
        if first_token == line.len() {
            // A paragraph break ends adjacency to a command, but must not
            // reset a group or math region which is still open.
            self.optional_argument = false;
        }
        let mut safe = self.groups.is_empty()
            && self.math.is_none()
            && self.protected_environments == 0
            && !self.expl3;
        let start = breaks.len();
        let mut has_prose = false;
        let mut literal = false;
        let mut i = 0;
        while i < line.len() {
            if line.as_bytes()[i].is_ascii_alphanumeric() {
                has_prose |= self.groups.is_empty() && self.math.is_none();
                self.optional_argument = false;
                i += 1;
                while i < line.len()
                    && line.as_bytes()[i].is_ascii_alphanumeric()
                {
                    i += 1;
                }
                continue;
            }
            let c = line[i..].chars().next().unwrap();
            if c == '%' {
                safe = false;
                break;
            }
            if c == '\\' {
                let rest = &line[i..];
                let name = command_name(rest).unwrap();
                if INLINE_VERBATIMS.contains(&name) {
                    self.optional_argument = false;
                    literal = true;
                    // Leave the entire line alone, but skip literal delimiters
                    // so braces and dollars in inline code do not leak state.
                    safe = false;
                    if let Some(end) = inline_code_end(rest, name) {
                        i += end;
                        continue;
                    }
                    break;
                }
                if matches!(name, "begin" | "end") {
                    self.optional_argument = false;
                    safe = false;
                    let argument = rest[1 + name.len()..].trim_start();
                    if let Some(env) = environment_name(argument) {
                        self.environment(name == "begin", env.0, args, syntax);
                        i = line.len() - env.1.len();
                        continue;
                    }
                }
                if name == "ExplSyntaxOn" || name == "ExplSyntaxOff" {
                    self.expl3 = name == "ExplSyntaxOn";
                    safe = false;
                }
                if self.protected_environments > 0 || self.expl3 {
                    self.optional_argument = false;
                    i += 1 + if name.is_empty() {
                        rest.chars().nth(1).map_or(0, char::len_utf8)
                    } else {
                        name.len()
                    };
                    continue;
                }
                if name.is_empty() {
                    self.optional_argument = false;
                    if let Some(symbol) = rest.chars().nth(1) {
                        match symbol {
                            '(' => {
                                has_prose |= self.groups.is_empty();
                                self.math = Some("\\)");
                            }
                            '[' => {
                                self.math = Some("\\]");
                                safe = false;
                            }
                            ')' | ']' if self.math == Some(&rest[..2]) => {
                                self.math = None;
                            }
                            '\\' => safe = false,
                            _ => {}
                        }
                        i += 1 + symbol.len_utf8();
                        continue;
                    }
                } else {
                    self.optional_argument = true;
                    if name == "item" && i != first_token {
                        safe = false;
                    }
                    if MACRO_DECLARATIONS.contains(&name) {
                        self.declaration_name_depth = Some(self.groups.len());
                    } else if self.declaration_name_depth
                        == Some(self.groups.len())
                    {
                        // A bare control word can supply the declaration's
                        // name instead of a braced argument.
                        self.declaration_name_depth = None;
                    }
                    if BOUNDARY_COMMANDS
                        .iter()
                        .any(|class| class.contains(&name))
                        || args
                            .sentence_skip_commands
                            .iter()
                            .any(|s| s.trim_start_matches('\\') == name)
                    {
                        safe = false;
                    }
                    i += 1 + name.len();
                    continue;
                }
            }
            if self.protected_environments > 0 || self.expl3 {
                i += c.len_utf8();
                continue;
            }
            let optional_argument = self.optional_argument;
            // Whitespace and a command's star can precede an optional
            // argument, including when its opening bracket is on a new line.
            if !(c.is_whitespace() || c == '*' && optional_argument) {
                self.optional_argument = false;
            }
            if c == '$' {
                let delimiter = if line[i..].starts_with("$$") {
                    "$$"
                } else {
                    "$"
                };
                if self.math == Some(delimiter) {
                    self.math = None;
                } else if self.math.is_none() {
                    has_prose |= self.groups.is_empty();
                    self.math = Some(delimiter);
                }
                if delimiter == "$$" {
                    safe = false;
                }
                i += delimiter.len();
                continue;
            }
            if self.math.is_none() {
                has_prose |= self.groups.is_empty() && c.is_alphanumeric();
                match c {
                    '{' => self.groups.push(c),
                    '[' if optional_argument
                        && self.groups.last() != Some(&'[')
                        && syntax
                            .optional_ends
                            .contains_key(&(source.offset + i)) =>
                    {
                        self.groups.push(c);
                        self.declaration_name_depth = None;
                    }
                    ']' if self.groups.last() == Some(&'[') => {
                        self.groups.pop();
                        self.optional_argument = true;
                    }
                    '}' => {
                        if self.groups.last() == Some(&'{') {
                            self.groups.pop();
                            if self.declaration_name_depth
                                == Some(self.groups.len())
                            {
                                self.optional_argument = true;
                                self.declaration_name_depth = None;
                            }
                        } else {
                            safe = false;
                        }
                    }
                    '.' | '!' | '?' if self.groups.is_empty() => {
                        if let Some(end) = candidate_end(line, i) {
                            breaks.push(end);
                        }
                    }
                    '&' | '#' | '@' => safe = false,
                    _ => {}
                }
            }
            i += c.len_utf8();
        }
        safe &= self.groups.is_empty() && self.math.is_none();
        ScannedLine {
            safe,
            start,
            stop: breaks.len(),
            has_prose,
            literal,
        }
    }

    fn environment(
        &mut self,
        begin: bool,
        name: &str,
        args: &Args,
        syntax: &DocumentSyntax,
    ) {
        if begin {
            let protected = !PROSE_ENVIRONMENTS.contains(&name)
                && !syntax.theorems.contains(name)
                && !args.lists.iter().any(|s| s == name)
                && !args.sentence_environments.iter().any(|s| s == name);
            self.protected_environments += usize::from(protected);
            self.environments.push((name.to_string(), protected));
        } else if self.environments.last().is_some_and(|(s, _)| s == name) {
            let (_, protected) = self.environments.pop().unwrap();
            self.protected_environments -= usize::from(protected);
        }
    }
}

/// Skip code with an arbitrary delimiter, or a balanced braced body.
fn inline_code_end(text: &str, name: &str) -> Option<usize> {
    let mut i = 1 + name.len();
    if text[i..].starts_with('*') {
        i += 1;
    }
    if text[i..].starts_with('[') {
        i += text[i..].find(']')? + 1;
    }
    if name == "mintinline" {
        i += text[i..].find('{')?;
        i += text[i..].find('}')? + 1;
    }
    let delimiter = text[i..].chars().next()?;
    i += delimiter.len_utf8();
    if delimiter == '{' {
        let mut depth = 1;
        for (offset, c) in text[i..].char_indices() {
            match c {
                '{' => depth += 1,
                '}' => depth -= 1,
                _ => {}
            }
            if depth == 0 {
                return Some(i + offset + 1);
            }
        }
        None
    } else {
        Some(i + text[i..].find(delimiter)? + delimiter.len_utf8())
    }
}

/// Find punctuation candidates without making a language decision yet.
fn candidate_end(text: &str, i: usize) -> Option<usize> {
    let mut end = i + 1;
    for ch in text[end..].chars() {
        if matches!(ch, '\'' | '"' | '’' | '”' | ')' | ']') {
            end += ch.len_utf8();
        } else {
            break;
        }
    }
    (text[end..].is_empty() || text[end..].starts_with(char::is_whitespace))
        .then_some(end)
}

/// Apply roles and neighbouring-token context to an unprotected candidate.
fn sentence_end(
    text: &str,
    end: usize,
    abbreviations: &HashSet<String>,
) -> bool {
    let before = text[..end].trim_end_matches(['\'', '"', '’', '”', ')', ']']);
    if !before.ends_with('.') {
        return true;
    }
    let i = before.len() - 1;
    if text[..i].ends_with(['.', '…']) {
        return false;
    }
    let start = text[..i]
        .rfind(|c: char| !c.is_alphabetic() && c != '.')
        .map_or(0, |n| n + text[n..].chars().next().unwrap().len_utf8());
    let word = &text[start..i];
    if !abbreviations.is_empty() {
        let lower = word.to_lowercase();
        if abbreviations.contains(&lower) {
            return false;
        }
        if word.bytes().all(|c| c.is_ascii_alphabetic())
            && !word.is_empty()
            && text[..start]
                .bytes()
                .rev()
                .take_while(|&b| b == b'\\')
                .count()
                % 2
                == 1
            && abbreviations.contains(&format!("\\{lower}"))
        {
            return false;
        }
    }
    let lookup = |word: &str| {
        ABBREVIATIONS.iter().find_map(|(key, role)| {
            key.eq_ignore_ascii_case(word).then_some(*role)
        })
    };
    let role = lookup(word).or_else(|| {
        word.strip_suffix(['s', 'S'])
            .and_then(lookup)
            .filter(|role| {
                matches!(role, Role::Prefix | Role::Reference | Role::Number)
            })
    });
    let mut letters = word.chars();
    let initial = letters.next().is_some_and(char::is_uppercase)
        && letters.next().is_none();
    // The word scan above admits only letters and dots. Degrees such as
    // Ph.D. have a two-letter segment alongside a single-letter initial.
    let mut single_letter = false;
    let acronym = word.contains('.')
        && word.split('.').all(|part| {
            let length = part.chars().count();
            single_letter |= length == 1;
            (1..=2).contains(&length)
        })
        && single_letter;
    // Ordinary terminal words do not need next-word or previous-word context.
    if initial || acronym {
        return false;
    }
    if role.is_none() {
        return true;
    }
    let after = text[end..].trim_start_matches(|c: char| {
        c.is_whitespace() || matches!(c, '\'' | '"' | '‘' | '“' | '(' | '[')
    });
    let next = after.split_whitespace().next().unwrap_or("");
    let next_word = next.trim_end_matches(|c: char| !c.is_alphabetic());
    let upper = next.chars().next().is_some_and(char::is_uppercase);
    let numeric = next.chars().next().is_some_and(|c| c.is_ascii_digit());
    match role {
        Some(Role::Prefix) => false,
        Some(Role::Reference) => {
            !(numeric
                || next.starts_with(['$', '\\'])
                || (upper && next_word.chars().count() == 1))
        }
        Some(Role::Number) => !numeric,
        _ => upper || next.is_empty(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{args::Args, format::format_file, LINE_END};
    use std::path::Path;

    fn format(input: &str) -> String {
        let args = Args {
            sentence_lines: true,
            ..Args::default()
        };
        format_file(input, Path::new("generated.tex"), &args, &mut vec![])
    }

    fn check_sentence_wrappings(parts: &[&str], expected: &str) {
        let expected = expected.replace('\n', LINE_END);
        for mask in 0..1 << (parts.len() - 1) {
            let mut input = parts[0].to_string();
            for (i, part) in parts[1..].iter().enumerate() {
                input.push(if mask & (1 << i) == 0 { ' ' } else { '\n' });
                input.push_str(part);
            }
            assert_eq!(format(&input), expected, "Wrapping of {input:?}");
            assert!(input.split_whitespace().eq(expected.split_whitespace()));
        }
        assert_eq!(format(&expected), expected);
    }

    // Each template is one complete sentence, followed by a separate sentence.
    fn check_then_next(sentence: &[&str], next: &str) {
        let expected = format!("{}\n{next}\n", sentence.join(" "));
        let mut parts = sentence.to_vec();
        parts.push(next);
        check_sentence_wrappings(&parts, &expected);
    }

    #[test]
    fn sentence_wrapping_invariants() {
        for atom in ["$x$", r"\(x\)", "$x. y$"] {
            check_then_next(&["The value of", atom, "is large."], "Next one.");
        }
        for atom in [
            r"\cite{key}",
            r"\citep{key}",
            r"\parencite{key}",
            r"\footnote{First. Second.}",
            r"\url{https://example.org/a.b}",
            r"\href{https://example.org}{the result}",
            r"\cite[see p. 2]{key}",
            r"\mycmd[opt]{arg}",
        ] {
            check_then_next(&["As shown by", atom, "this works."], "Next one.");
        }
        for name in [
            &[
                "The term spin glass is attributed to B.",
                "R.",
                "Coles around 1970.",
            ][..],
            &["B.", "R.", "Coles described the behaviour."],
            &[
                "The terminology appeared in the work of Philip W.",
                "Anderson.",
            ],
            &["Tony", "G.", "J.", "Jones published this."],
        ] {
            check_then_next(name, "Next sentence.");
        }
        check_then_next(&["We meet at 9 a.m.", "tomorrow."], "Next sentence.");
        for (end, next, separator) in [
            ("At 9 a.m.", "tomorrow.", " "),
            ("We left at 5 p.m.", "Tomorrow we return.", " "),
            ("We used tools, materials, etc.", "We then stopped.", "\n"),
            ("He lives in the U.S.", "However, he travels.", " "),
            ("Tools, etc.", "“Zebras arrived.”", "\n"),
            ("Smith et al.", "reported this.", " "),
            ("Smith et al.", "(Zebras arrived.)", "\n"),
        ] {
            check_sentence_wrappings(
                &[end, next],
                &format!("{end}{separator}{next}\n"),
            );
        }
        for cmd in [
            r"\vspace{1em}",
            r"\bigskip",
            r"\newpage",
            r"\bibliography{x}",
        ] {
            for end in ["End.", "“End!”", "(End?)", "End of paragraph."] {
                let standalone = format!("{end}\n{cmd}\nNext sentence.");
                check_sentence_wrappings(
                    &[&standalone],
                    &format!("{standalone}\n"),
                );
            }
            let boundary = format!("ends.\n{cmd}\nNext");
            check_sentence_wrappings(
                &["It", &boundary, "sentence."],
                &format!("It {boundary} sentence.\n"),
            );
        }
        let long = "A long sentence with many words ".repeat(100) + ".";
        check_sentence_wrappings(&[&long], &format!("{long}\n"));
    }

    // The only allowed token join is a control word followed by an argument.
    fn tokens_preserved(input: &str, output: &str) -> bool {
        let mut words = input.split_whitespace();
        for mut token in output.split_whitespace() {
            while !token.is_empty() {
                let Some(word) = words.next() else {
                    return false;
                };
                let Some(rest) = token.strip_prefix(word) else {
                    return false;
                };
                if !rest.is_empty() {
                    let prefix = word
                        .trim_end_matches(|c: char| c.is_ascii_alphabetic());
                    let slashes = prefix
                        .bytes()
                        .rev()
                        .take_while(|&b| b == b'\\')
                        .count();
                    if prefix == word
                        || slashes % 2 == 0
                        || !rest.starts_with(['{', '['])
                    {
                        return false;
                    }
                }
                token = rest;
            }
        }
        words.next().is_none()
    }

    fn check_invariants(input: &str) {
        let output = format(input);
        assert!(
            tokens_preserved(input, &output),
            "{input:?} became {output:?}"
        );
        assert_eq!(format(&output), output, "Second pass for {input:?}");
    }

    #[test]
    fn sentence_malformed_input() {
        for bracket in ['[', ']'] {
            check_invariants(&format!(
                "A literal {bracket} is ordinary prose. Next sentence."
            ));
        }
        check_invariants(&format!(
            "{}\nUnicode é. Next.\n{}",
            "{".repeat(256),
            "}".repeat(256)
        ));
        check_invariants(&format!(
            "{}{}",
            "\\begin{verbatim}\n".repeat(140),
            "\\end{verbatim}\n".repeat(140)
        ));
        #[rustfmt::skip]
        let fragments = [
            "word.", " Word ", "$", r"\(", r"\)", "[", "]", "{", "}", "%",
            r"\\", r"\verb|ä|", r"\begin{x}", r"\end{x}",
            "é", "東京", "\n", "\n\n", " ", "?!", r"\cmd",
            r"\section{x}", r"\item", r"\begin{itemize}", r"\end{itemize}",
            r"\label{x}", r"\[", r"\]", "$$", r"\newcommand{\x}{y}", r"\caption{c}",
        ];
        let mut seed = 0x162_u32;
        for _ in 0..1024 {
            let mut input = String::new();
            for _ in 0..24 {
                seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                input.push_str(
                    fragments[(seed >> 8) as usize % fragments.len()],
                );
            }
            check_invariants(&input);
        }
    }

    #[test]
    fn stdin_overrides_the_file_extension() {
        let input =
            "First sentence. Another sentence with several words. ".repeat(6);
        let args = Args {
            sentence_lines: true,
            ..Args::default()
        };
        for extension in ["bib", "sty", "cls"] {
            let path = std::path::PathBuf::from(format!("sample.{extension}"));
            let expected =
                format_file(&input, &path, &Args::default(), &mut vec![]);
            assert_ne!(expected, format(&input));
            assert_eq!(
                format_file(&input, &path, &args, &mut vec![]),
                expected
            );
        }
        let stdin = Args {
            stdin: true,
            ..args
        };
        assert_eq!(
            format_file(&input, Path::new("sample.bib"), &stdin, &mut vec![]),
            format(&input)
        );
    }

    #[test]
    fn untouched_structure_matches_default_layout() {
        let input =
            include_str!("../tests/sentence_lines/source/structure.tex");
        let default = format_file(
            input,
            Path::new("structure.tex"),
            &Args::default(),
            &mut vec![],
        );
        assert_eq!(format(input), default);
    }
}
