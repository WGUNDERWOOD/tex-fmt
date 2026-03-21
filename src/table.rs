//! Formatting tables

use itertools::Itertools;

// New ideas
// Write a function to format a table only
//    Takes text for a table environment in and formats it to a string
// Then write separate logic for finding tables in the file
// Finally write logic for formatting the full file

pub fn get_max_delims_per_line(text: &str) -> usize {
    text.lines().map(|l| l.matches('&').count()).max().unwrap()
}

pub fn get_position_of_jth_delim(line: &str, j: usize) -> Option<usize> {
    line.chars()
        .enumerate()
        .filter(|&(_, c)| c == '&')
        .nth(j)
        .map(|(i, _)| i)
}

pub fn get_max_position_of_jth_delim(text: &str, j: usize) -> Option<usize> {
    text.lines().map(|l| get_position_of_jth_delim(l, j)).flatten().max()
}

pub fn get_offsets_for_jth_delim(text: &str, j: usize) -> Vec<Option<usize>> {
    let max_position = get_max_position_of_jth_delim(text, j);
    //dbg!(j);
    //dbg!(max_position);
    text
        .lines()
        .map(|l| get_position_of_jth_delim(l, j))
        .map(|pos| pos.map(|p| max_position.map(|m| m - p)).flatten())
        .collect()
}

pub fn align_table(text: &str) -> String {
    // Get max number of delimiters on any line
    let max_delims_per_line = get_max_delims_per_line(text);

    // Calculate the necessary offset of each delim for each line
    // TODO Just do this one line at a time, then build the text for that line
    for j in 0..max_delims_per_line {
        let offsets_for_jth_delim =
            get_offsets_for_jth_delim(&text, j);
        dbg!(j);
        println!("{:?}", &offsets_for_jth_delim);
    }

    text.to_string()
}

pub fn align_tables(text: &str) -> String {
    align_table(text);
    text.to_string()
}


/*
pub fn align_tables(text: &str) -> String {

    let table_begins = ["\\begin{tabular}"];
    let table_ends = ["\\end{tabular}"];
    let mut begin: usize = 0;
    let mut end: usize;

    let mut table_positions: Vec<(usize, usize)> = vec![];

    // find tables in text
    for (linum, line) in text.lines().enumerate() {
        if table_begins.iter().any(|r| line.contains(r)) {
            begin = linum;
        } else if table_ends.iter().any(|r| line.contains(r)) {
            end = linum;
            table_positions.push((begin, end))
        }
    }

    let first_table_begin = table_positions[0].0;
    let mut new_text: String = text.lines().take(first_table_begin).join("\n");
    new_text.push('\n');

    // for each table
    for (t, table_position) in table_positions.iter().enumerate() {

        // get the desired delimiter positions
        let begin = table_position.0;
        let end = table_position.1;
        let mut max_delim_positions: Vec<usize> = vec![];
        for line in text.lines().skip(begin).take(end - begin + 1) {
            let mut delim_positions: Vec<usize> = vec![];
            for (i, c) in line.chars().enumerate() {
                if c == '&' {
                    delim_positions.push(i);
                }
            }
            for (i, p) in delim_positions.iter().enumerate() {
                if max_delim_positions.len() > i {
                    if *p > max_delim_positions[i] {
                        max_delim_positions[i] = *p;
                    }
                } else {
                    max_delim_positions.push(*p);
                }
            }
        }

        // get gaps between delimiter positions
        let diff_delim_positions: Vec<_> =
            max_delim_positions.first()
            .into_iter()
            .copied()
            .chain(max_delim_positions.windows(2).map(|w| w[1] - w[0]))
            .collect();

        // insert characters as appropriate
        for line in text.lines().skip(begin).take(end - begin + 1) {
            let mut new_line = String::new();
            let mut delim_counter = 0;
            let mut prev_delim_position = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '&' {
                    let space_needed = diff_delim_positions[delim_counter]
                        + prev_delim_position - i;
                    if space_needed > 0 {
                        new_line.push_str(&" ".repeat(space_needed));
                    }
                    delim_counter += 1;
                    prev_delim_position = i;
                }
                new_line.push(c)
            }
            new_text.push_str(&new_line);
            new_text.push('\n');
        }

        let next_table_begin = if t + 1 < table_positions.len() {
                table_positions[t+1].0
        } else {
            text.lines().count()
        };
        let next_text: String =
            text.lines().skip(end+1).take(next_table_begin - end - 1).join("\n");
        new_text.push_str(&next_text);
        new_text.push('\n');
    }

    new_text.to_string()
}
*/
