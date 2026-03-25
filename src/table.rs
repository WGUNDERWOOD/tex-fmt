//! Formatting tables

use crate::LINE_END;
use crate::format::State;
use crate::regexes::{TABLES_BEGIN, TABLES_END};
use itertools::Itertools;
use regex::Regex;

// Remove all double spaces from the table
fn remove_double_spaces(text: &str) -> String {
    let re = Regex::new(r"(\S) {2,}").unwrap();
    re.replace_all(text, "$1 ").to_string()
}

// Add line breaks after "\\"
fn add_line_breaks(text: &str) -> (String, bool) {
    let re_break = Regex::new(r"\\\\ .*\S").unwrap();
    let re_indent = Regex::new(r"^\s*\S").unwrap();
    let re_first_non_white = Regex::new(r"\S.*").unwrap();
    let re_to_break = Regex::new(r"^[^\\]*\\\\").unwrap();
    let mut new_text = String::new();
    let mut finished: bool = true;
    for line in text.lines() {
        if re_break.is_match(line) {
            finished = false;
            let indent = re_indent.find(line).map_or("", |m| {
                let s = m.as_str();
                &s[..s.len() - 1]
            });
            let next_line_long = re_break.find(line).map_or("", |m| {
                let s = m.as_str();
                &s[2..]
            });
            let next_line = re_first_non_white
                .find(next_line_long)
                .map_or("", |m| m.as_str());
            let this_line = re_to_break.find(line).map_or("", |m| m.as_str());
            new_text.push_str(this_line);
            new_text.push_str(LINE_END);
            new_text.push_str(indent);
            new_text.push_str(next_line);
            new_text.push_str(LINE_END);
        } else {
            new_text.push_str(line);
            new_text.push_str(LINE_END);
        }
    }
    (new_text, finished)
}

// Get the starting positions of all delims
fn get_positions(text: &str) -> Vec<Vec<usize>> {
    text.lines()
        .map(|l| {
            let mut prev = None;
            l.char_indices()
                .filter_map(|(i, c)| {
                    let is_match = c == '&' && prev != Some('\\');
                    prev = Some(c);
                    if is_match {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect()
        })
        .collect()
}

// Find the desired new positions of the delims
fn get_new_positions(positions: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n_delims = positions.iter().map(std::vec::Vec::len).max().unwrap();
    let mut new_positions = positions.to_owned();
    for j in 0..n_delims {
        let positions: Vec<Option<usize>> =
            new_positions.iter().map(|l| l.get(j).copied()).collect();
        let new_position: usize = positions.iter().max().unwrap().unwrap();
        for line in &mut new_positions {
            let n_delims_line = line.len();
            if n_delims_line > j {
                let offset = new_position - line[j];
                for pos in line.iter_mut().skip(j) {
                    *pos += offset;
                }
            }
        }
    }
    new_positions
}

// Calculate the cumulative offsets needed
fn get_offsets(
    positions: &[Vec<usize>],
    new_positions: &[Vec<usize>],
) -> Vec<Vec<usize>> {
    let mut offsets = vec![];
    for l in 0..positions.len() {
        let mut prev_offset = 0;
        let mut offset = vec![];
        for j in 0..positions[l].len() {
            offset.push(new_positions[l][j] - positions[l][j] - prev_offset);
            prev_offset = new_positions[l][j] - positions[l][j];
        }
        offsets.push(offset);
    }
    offsets
}

// Use the offsets to format one line of the table
fn format_table_line(line: &str, offsets_delims_row: &[usize]) -> String {
    let mut new_line = String::new();
    let mut j = 0;
    for c in line.chars() {
        if c == '&' {
            let offset = offsets_delims_row.get(j).copied().unwrap_or(0);
            new_line.extend(std::iter::repeat_n(' ', offset));
            j += 1;
        }
        new_line.push(c);
    }
    new_line
}

// Format a single table
fn format_table(text: &str) -> String {
    let mut clean_text = remove_double_spaces(text);

    // add line breaks
    let mut finished: bool;
    (clean_text, finished) = add_line_breaks(&clean_text);
    let max_line_break_attempts = 10;
    for _attempt in 0..max_line_break_attempts {
        if finished {
            break;
        }
        (clean_text, finished) = add_line_breaks(&clean_text);
    }

    let positions = get_positions(&clean_text);
    let new_positions = get_new_positions(&positions);
    let offsets = get_offsets(&positions, &new_positions);
    let mut new_text = String::new();
    for (linum, line) in clean_text.lines().enumerate() {
        let new_line = format_table_line(line, &offsets[linum]);
        new_text.push_str(&new_line);
        new_text.push_str(LINE_END);
    }
    new_text
}

fn contains_table_begin(line: &str) -> bool {
    TABLES_BEGIN.iter().any(|r| line.contains(r))
}

fn contains_table_end(line: &str) -> bool {
    TABLES_END.iter().any(|r| line.contains(r))
}

// Locate all the tables in the text
fn find_table_positions(text: &str) -> Vec<(usize, usize)> {
    let mut table_positions = vec![];
    let mut begin: usize = 0;
    let mut end: usize;
    for (linum, line) in text.lines().enumerate() {
        if contains_table_begin(line) {
            begin = linum;
        } else if contains_table_end(line) {
            end = linum;
            table_positions.push((begin, end));
        }
    }
    table_positions
}

// Format all the tables in the text
#[must_use]
pub fn format_tables(text: &str) -> String {
    let table_positions = find_table_positions(text);
    if table_positions.is_empty() {
        return text.to_string();
    }

    let first_table_begin = table_positions[0].0;
    let mut new_text: String = text.lines().take(first_table_begin).join(LINE_END);
    new_text.push_str(LINE_END);

    for (t, table_position) in table_positions.iter().enumerate() {
        // format each table
        let begin = table_position.0;
        let end = table_position.1;
        let table_text: String =
            text.lines().skip(begin).take(end - begin + 1).join(LINE_END);
        let new_table_text = format_table(&table_text);
        new_text.push_str(&new_table_text);

        // format text following each table
        let next_table_begin = if t + 1 < table_positions.len() {
            table_positions[t + 1].0
        } else {
            text.lines().count()
        };
        let next_text: String = text
            .lines()
            .skip(end + 1)
            .take(next_table_begin - end - 1)
            .join(LINE_END);
        new_text.push_str(&next_text);
        new_text.push_str(LINE_END);
    }

    new_text
}

/// Information on the table state of a line
#[derive(Clone, Debug)]
pub struct Table {
    /// Whether the line is in a table
    pub actual: bool,
    /// Whether the line appears to be in a table
    pub visual: bool,
}

impl Table {
    /// Construct a new table state
    #[must_use]
    pub const fn new() -> Self {
        Self {
            actual: false,
            visual: false,
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

// Check if a line is inside a table
#[must_use]
pub fn is_inside_table(line: &str, state: &State) -> Table {
    let begin = contains_table_begin(line);
    let end = contains_table_end(line);
    let actual: bool;
    let visual: bool;
    if begin {
        actual = true;
        visual = true;
    } else if end {
        actual = false;
        visual = true;
    } else {
        actual = state.table.actual;
        visual = state.table.actual;
    }

    Table { actual, visual }
}
