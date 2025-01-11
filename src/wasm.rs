use js_sys::{Object, Reflect};
use merge::Merge;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

use crate::args::*;
use crate::config::*;
use crate::format::*;
use crate::logging::*;

#[wasm_bindgen]
pub fn main(text: &str, config: &str) -> JsValue {

    // Get args
    let config = Some((PathBuf::new(), "".to_string(), config.to_string()));
    let mut args: OptionArgs = get_config_args(config).unwrap();
    args.merge(OptionArgs::default());
    let mut args = Args::from(args);
    args.stdin = true;

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
