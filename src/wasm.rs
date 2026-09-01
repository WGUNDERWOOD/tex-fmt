//! Web assembly implementation

use clap::crate_version;
use js_sys::{Object, Reflect};
use merge::Merge;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

use crate::args::{Args, OptionArgs};
use crate::config::get_config_args;
use crate::format::format_file;
use crate::logging::{format_logs, Log};

/// Build the JS return object holding just an error message
fn error_object(message: &str) -> JsValue {
    let js_object = Object::new();
    Reflect::set(&js_object, &"output".into(), &JsValue::from_str("")).unwrap();
    Reflect::set(&js_object, &"logs".into(), &JsValue::from_str(message))
        .unwrap();
    js_object.into()
}

/// Main function for WASM interface with JS
///
/// # Panics
///
/// This function panics if the JS return object cannot be constructed
#[wasm_bindgen]
#[must_use]
pub fn main(text: &str, config: &str) -> JsValue {
    // Get args
    let config = Some((PathBuf::new(), String::new(), config.to_string()));
    let mut args: OptionArgs = match get_config_args(config) {
        Ok(args) => args.unwrap_or_else(OptionArgs::new),
        Err(message) => return error_object(&message),
    };
    args.merge(OptionArgs::default());
    let mut args = Args::from(args);
    args.stdin = true;

    // Run tex-fmt
    let mut logs = Vec::<Log>::new();
    args.resolve(&mut logs);
    let file = PathBuf::from("input");
    let new_text = format_file(text, &file, &args, &mut logs);
    let logs = format_logs(&mut logs, &args);

    // Wrap into JS object
    let js_object = Object::new();
    Reflect::set(&js_object, &"output".into(), &new_text.into()).unwrap();
    Reflect::set(&js_object, &"logs".into(), &logs.into()).unwrap();
    js_object.into()
}

#[wasm_bindgen]
#[must_use]
pub fn version() -> JsValue {
    let version = crate_version!();
    let formatted_version = format!("v{}", version);
    JsValue::from_str(&formatted_version)
}
