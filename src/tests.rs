use crate::apply;
use crate::colors::*;
use crate::format_file;
use crate::fs;
use crate::logging::*;
use crate::rstest;
use crate::template;
use crate::Cli;

#[template]
#[rstest]
#[case::brackets("brackets", "tex")]
#[case::comments("comments", "tex")]
#[case::cv("cv", "tex")]
#[case::document("document", "tex")]
#[case::empty("empty", "tex")]
#[case::environment_lines("environment_lines", "tex")]
#[case::ignore("ignore", "tex")]
#[case::lists("lists", "tex")]
#[case::masters_dissertation("masters_dissertation", "tex")]
#[case::phd_dissertation("phd_dissertation", "tex")]
#[case::phd_dissertation_refs("phd_dissertation_refs", "bib")]
#[case::pu_thesis("pu_thesis", "cls")]
#[case::readme("readme", "tex")]
#[case::short_document("short_document", "tex")]
#[case::tikz_network("tikz_network", "sty")]
#[case::verbatim("verbatim", "tex")]
#[case::wrap("wrap", "tex")]
fn test_file(#[case] filename: &str, #[case] extension: &str) {}

#[apply(test_file)]
fn test_in_file(filename: &str, extension: &str) {
    let args = Cli::new();
    let mut logs = Vec::<Log>::new();
    let in_filename = format!("tests/{}_in.{}", filename, extension);
    let out_filename = format!("tests/{}_out.{}", filename, extension);
    let in_file = fs::read_to_string(&in_filename).unwrap();
    let out_file = fs::read_to_string(&out_filename).unwrap();
    let fmt_in_file = format_file(&in_file, &in_filename, &args, &mut logs);
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
}

#[apply(test_file)]
fn test_out_file(filename: &str, extension: &str) {
    let args = Cli::new();
    let mut logs = Vec::<Log>::new();
    let out_filename = format!("tests/{}_out.{}", filename, extension);
    let out_file = fs::read_to_string(&out_filename).unwrap();
    let fmt_out_file = format_file(&out_file, &out_filename, &args, &mut logs);
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
}
