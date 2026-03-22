//! Formatting tables

use itertools::Itertools;
use regex::Regex;

// First we remove all double spaces from the table
fn clean_table(text: &str) -> String {
    let re = Regex::new(r"(\S) {2,}").unwrap();
    re.replace_all(text, "$1 ").to_string()
}

// Then we get the starting positions of all delims.
pub fn get_positions(text: &str) -> Vec<Vec<usize>> {
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

// Then we find the desired new positions of the delims
fn get_new_positions(positions: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n_lines = positions.len();
    let n_delims = positions.iter().map(|l| l.len()).max().unwrap();
    let mut new_positions = positions.clone();
    for j in 0..n_delims {
        let positions: Vec<Option<usize>> =
            new_positions.iter().map(|l| l.get(j).copied()).collect();
        let new_position: usize = positions.iter().max().unwrap().unwrap();
        for l in 0..n_lines {
            let n_delims_line = new_positions[l].len();
            if n_delims_line > j {
                let offset = new_position - new_positions[l][j];
                for r in j..n_delims_line {
                    new_positions[l][r] += offset;
                }
            }
        }
    }
    new_positions
}

// Then we calculate the cumulative offsets needed
fn get_offsets(
    positions: &Vec<Vec<usize>>,
    new_positions: &Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let mut offsets = vec![];
    for l in 0..positions.len() {
        let mut prev_offset = 0;
        let mut offset = vec![];
        for j in 0..positions[l].len() {
            offset.push(new_positions[l][j] - positions[l][j] - prev_offset);
            prev_offset = new_positions[l][j] - positions[l][j];
        }
        offsets.push(offset)
    }
    offsets
}

// Use the offsets to align one table line
fn align_table_line(line: &str, offsets_delims_row: &Vec<usize>) -> String {
    let mut new_line = String::new();
    let mut j = 0;
    for c in line.chars() {
        if c == '&' {
            let offset = offsets_delims_row.get(j).copied().unwrap_or(0);
            new_line.extend(std::iter::repeat(' ').take(offset));
            j += 1;
        }
        new_line.push(c);
    }
    new_line
}

// Use the offsets to align the table text
pub fn align_table(text: &str) -> String {
    let clean_text = clean_table(&text);
    let positions = get_positions(&clean_text);
    let new_positions = get_new_positions(&positions);
    let offsets = get_offsets(&positions, &new_positions);
    let mut new_text = String::new();
    for (linum, line) in clean_text.lines().enumerate() {
        let new_line = align_table_line(line, &offsets[linum]);
        new_text.push_str(&new_line);
        new_text.push('\n');
    }
    new_text
}

pub fn find_table_positions(text: &str) -> Vec<(usize, usize)> {
    let table_begins = ["\\begin{tabular}"];
    let table_ends = ["\\end{tabular}"];
    let mut table_positions = vec![];
    let mut begin: usize = 0;
    let mut end: usize;
    for (linum, line) in text.lines().enumerate() {
        if table_begins.iter().any(|r| line.contains(r)) {
            begin = linum;
        } else if table_ends.iter().any(|r| line.contains(r)) {
            end = linum;
            table_positions.push((begin, end))
        }
    }
    table_positions
}

pub fn align_tables(text: &str) -> String {
    let table_positions = find_table_positions(text);
    if table_positions.len() == 0 {
        return text.to_string();
    }

    let first_table_begin = table_positions[0].0;
    let mut new_text: String = text.lines().take(first_table_begin).join("\n");
    new_text.push('\n');

    for (t, table_position) in table_positions.iter().enumerate() {
        // format each table
        let begin = table_position.0;
        let end = table_position.1;
        let table_text: String =
            text.lines().skip(begin).take(end - begin + 1).join("\n");
        let new_table_text = align_table(&table_text);
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
            .join("\n");
        new_text.push_str(&next_text);
        new_text.push('\n');
    }

    new_text
}
