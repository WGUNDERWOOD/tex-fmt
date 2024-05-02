mod tests {

    use crate::format_file;
    use std::fs;

    const YELLOW: &str = "\x1b[33m\x1b[1m";
    const GREEN: &str = "\x1b[32m\x1b[1m";
    const RED: &str = "\x1b[31m\x1b[1m";
    const WHITE: &str = "\x1b[37m\x1b[1m";
    const RESET: &str = "\x1b[00m\x1b[0m";

    fn test_file(filename: &str) {
        let in_filename = format!("tests/{}_in.tex", filename);
        let out_filename = format!("tests/{}_out.tex", filename);
        let in_file = fs::read_to_string(&in_filename).expect("");
        let out_file = fs::read_to_string(&out_filename).expect("");
        let fmt_in_file = format_file(&in_file, false);
        let fmt_out_file = format_file(&out_file, false);
        assert!(fmt_in_file == out_file,
            "\n{}Test failed: {}{}{} -> {}{}{}\n\n{}Output:\n{}{}{}\nDesired:\n{}{}",
            &RED,
            &YELLOW,
            &in_filename,
            &WHITE,
            &YELLOW,
            &out_filename,
            &RESET,
            &YELLOW,
            &RESET,
            &fmt_in_file,
            &YELLOW,
            &RESET,
            &out_file);
        assert!(fmt_out_file == out_file,
            "\n{}Test failed: {}{}{} -> {}{}{}\n\n{}Output:\n{}{}{}\nDesired:\n{}{}",
            &RED,
            &YELLOW,
            &out_filename,
            &WHITE,
            &YELLOW,
            &out_filename,
            &RESET,
            &YELLOW,
            &RESET,
            &fmt_out_file,
            &YELLOW,
            &RESET,
            &out_file);
        println!("{}Pass: {}{}", &GREEN, &RESET, &in_filename);
    }

    #[test]
    fn test_files() {
        let filenames: Vec<String> = fs::read_dir("tests/")
            .unwrap()
            .map(|f| f.unwrap().file_name().into_string().unwrap())
            .filter(|f| f.ends_with("_in.tex"))
            .map(|f| f.strip_suffix("_in.tex").unwrap().to_string())
            .collect();
        for filename in filenames {
            test_file(&filename);
        }
    }
}
