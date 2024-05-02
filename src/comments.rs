#[derive(Debug)]
pub struct Comment{
    // index where the comment starts
    idx: usize,
    // does the comment have a leading space
    space: bool,
}

pub fn find_comment(line: &str) -> Option<Comment> {

    // no percent means no comment
    if !line.contains('%') {
        return None
    }

    let n = line.len();

    // empty line
    if n == 0 {
        return None
    }

    // check the first character
    let mut prev_c: char = line.chars().nth(0).unwrap();
    if prev_c == '%' {
        return Some(Comment{idx: 0, space: false})
    }

    // single-character line
    if n == 1 {
        return None
    }

    // multi-character line
    for i in 1..n {
        let c = line.chars().nth(i).unwrap();
        if c == '%' {
            if prev_c == ' ' {
                return Some(Comment{idx: i, space: true})
            } else if prev_c != '\\' {
                return Some(Comment{idx: i, space: false})
            }
        }
        prev_c = c;
    }
    None
}

pub fn remove_comment<'a>(line: &'a str, comm: &Option<Comment>) -> &'a str {
    match comm {
        Some(c) => &line[0..c.idx],
        None => line
    }
}

pub fn get_comment<'a>(line: &'a str, comm: &Option<Comment>) -> &'a str {
    match comm {
        Some(c) => &line[c.idx..line.len()],
        None => ""
    }
}
