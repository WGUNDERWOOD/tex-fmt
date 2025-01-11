use wasm_bindgen::prelude::*;

use crate::args::*;
use crate::format::*;
use crate::logging::*;

#[wasm_bindgen]
pub fn main(text: &str) -> String {
    // TODO Display logs on web page
    // TODO Read user-supplied config file
    let mut args = Args {
        stdin: true,
        ..Default::default()
    };
    let mut logs = Vec::<Log>::new();
    args.resolve(&mut logs);
    let file = "input";
    format_file(text, file, &args, &mut logs)
}
