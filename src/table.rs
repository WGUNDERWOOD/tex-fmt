//! Formatting tables

use itertools::Itertools;
use regex::Regex;

pub fn get_positions_delims(text: &str) -> Vec<Vec<usize>> {
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

//fn get_max_positions_delims(positions_delims: &Vec<Vec<usize>>) -> Vec<usize> {
//    // get rightmost jth delimiters for each j
//    let mut max_positions_delims = vec![];
//    for line in positions_delims {
//        for (j, &pos) in line.iter().enumerate() {
//            if j >= max_positions_delims.len() {
//                max_positions_delims.push(pos);
//            } else {
//                max_positions_delims[j] = max_positions_delims[j].max(pos);
//            }
//        }
//    }
//
//    // ensure these are increasing in j
//    let mut prev_pos: usize = *max_positions_delims.get(0).unwrap_or(&0);
//    max_positions_delims
//        .into_iter()
//        .enumerate()
//        .map(|(i, pos)| {
//            if i > 0 {
//                dbg!(prev_pos);
//                prev_pos = pos.max(prev_pos + 2);
//                prev_pos
//            } else {
//                pos
//            }
//        })
//    .collect()
//    //dbg!(&max_positions_delims);
//    //max_positions_delims
//}

//pub fn get_offsets_delims(
//    positions_delims: &Vec<Vec<usize>>,
//) -> Vec<Vec<usize>> {
//    let max_positions_delims = get_max_positions_delims(positions_delims);
//    let n_lines = positions_delims.len();
//    let mut offsets_delims: Vec<Vec<usize>> =
//        (0..n_lines).map(|_| vec![]).collect();
//    for (linum, line) in positions_delims.into_iter().enumerate() {
//        let mut offset_counter = 0;
//        dbg!(linum);
//        for j in 0..line.len() {
//            dbg!(j);
//            //dbg!(max_positions_delims[j]);
//            //dbg!(positions_delims[linum][j]);
//            dbg!(&positions_delims[linum]);
//            dbg!(&max_positions_delims);
//            dbg!(offset_counter);
//            let offset = max_positions_delims[j] - positions_delims[linum][j];
//            dbg!(offset);
//            offsets_delims[linum].push(offset - offset_counter);
//            offset_counter = offset;
//        }
//    }
//    offsets_delims
//}

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

pub fn align_table(text: &str) -> String {
    //println!("{}", &text);
    let positions_delims = get_positions_delims(&text);
    let offsets = get_offsets(positions_delims);
    let mut new_text = String::new();
    for (linum, line) in text.lines().enumerate() {
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

/*
pub fn align_tables(text: &str) -> String {

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

    }

    new_text.to_string()
}
*/

// TODO Oh boy this is more complicated than I thought...
// First we should remove all double spaces from the table
// (except indents at the start of the line).
// Then we get the starting positions of all delims.
// Then we find the desired new position of the 1st delim
// (this is the position of the rightmost 1st delim).
// Next we calculate the offset (positive) that must
// be applied to each 1st delim.
// We then update all the delim positions by applying
// this offset to every entry.
// Then we find the desired new position of the 2nd delim
// (this is the position of the rightmost 2nd delim).
// Etc

fn clean_table(text: &str) -> String {
    let re = Regex::new(r"(?<=\S) {2,}").unwrap();
    re.replace_all(text, " ").to_string()
}

fn get_offsets(
    positions_delims: Vec<Vec<usize>>,
) -> Vec<Vec<usize>> {
    let n_lines = positions_delims.len();
    let n_delims = positions_delims.iter().map(|l| l.len()).max().unwrap();
    let mut new_positions_delims = positions_delims.clone();
    for j in 0..n_delims {
        let positions: Vec<Option<usize>> = new_positions_delims
            .iter()
            .map(|l| l.get(j).copied())
            .collect();
        let new_position: usize = positions.iter().max().unwrap().unwrap();
        for l in 0..n_lines {
            if new_positions_delims[l].len() > j {
                let offset: usize = new_position - new_positions_delims[l][j];
                for r in (j + 1)..n_delims {
                    new_positions_delims[l][r] += offset;
                }
            }
        }
    }
    new_positions_delims
        .iter()
        .enumerate()
        .map(|(l, line)| {
            line.iter()
                .enumerate()
                .map(|(j, _)| line[j] - positions_delims[l][j])
                .collect()
        })
        .collect()
}
