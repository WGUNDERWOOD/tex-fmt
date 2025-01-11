use js_sys::{Object, Reflect};
use log::LevelFilter;
use wasm_bindgen::prelude::*;

use crate::args::*;
use crate::format::*;
use crate::logging::*;

#[wasm_bindgen]
pub fn main(text: &str) -> JsValue {
    // Set up arguments
    let mut args = Args {
        stdin: true,
        verbosity: LevelFilter::Warn,
        ..Default::default()
    };

    // Run tex-fmt
    let mut logs = Vec::<Log>::new();
    args.resolve(&mut logs);
    let file = "input";
    let new_text = format_file(text, file, &args, &mut logs);
    let logs = format_logs(&mut logs, &args);

    // Wrap into JS object
    let js_object = Object::new();
    Reflect::set(&js_object, &"output".into(), &new_text.into()).unwrap();
    Reflect::set(&js_object, &"logs".into(), &logs.into()).unwrap();
    js_object.into()
}
