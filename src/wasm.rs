use wasm_bindgen::prelude::*;

use crate::args::*;
use crate::logging::*;
use crate::format::*;

#[wasm_bindgen]
pub fn main(text: &str) {
    let mut args = Args::default();
    args.stdin = true;
    init_logger(args.verbosity);
    let mut logs = Vec::<Log>::new();
    args.resolve(&mut logs);
    let file = "input";

    alert(&format!("Before"));

    // TODO Need web time here?
    record_file_log(&mut logs, log::Level::Error, "", "");

    alert(&format!("After"));
    //let new_text = format_file(text, &file, &args, &mut logs);
    //alert(&format!("{}", new_text));
    //new_text
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
