use wasm_bindgen::prelude::*;

use crate::args::*;
use crate::format::*;
use crate::logging::*;

#[wasm_bindgen]
pub fn main(text: &str) -> String {
    let mut args = Args {
        stdin: true,
        ..Default::default()
    };
    //init_logger(args.verbosity);
    let mut logs = Vec::<Log>::new();
    args.resolve(&mut logs);
    let file = "input";
    record_file_log(&mut logs, log::Level::Error, "", "");
    let new_text = format_file(text, file, &args, &mut logs);
    new_text
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}
