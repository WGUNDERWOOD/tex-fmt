// wrapping

const WRAP: usize = 80;

// any line under WRAP chars must be unchanged

// while line is long
//   find the first break space (should be after non-space character)
//   if break is after WRAP, throw warning
//   if no comment, replace space with newline
//   if comment and break before comment, replace space with newline
//   if comment and break after comment
//     if spaced comment
//       replace space before comment with newline
//     if non-spaced comment
//       replace comment % with %\n%

pub fn needs_wrap(file: &str) -> bool {
    file.lines().any(|l| l.len() > WRAP)
}

pub fn wrap(file: &str) -> String {
    let mut new_file = "".to_string();
    let lines: Vec<&str> = file.lines().collect();
    for line in lines.iter() {
    }

    //dbg!(&file);
    file.to_string()
}
