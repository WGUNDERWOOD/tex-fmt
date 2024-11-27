use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct TexFmtResult {
    code: u8,
    output: Option<String>, // Private field
}

#[wasm_bindgen]
impl TexFmtResult {
    #[wasm_bindgen(constructor)]
    pub fn new(code: u8, output: Option<String>) -> TexFmtResult {
        TexFmtResult { code, output }
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> u8 {
        self.code
    }

    #[wasm_bindgen(getter)]
    pub fn output(&self) -> String {
        match self.output {
            // Clone the String to return a copy
            Some(ref value) => value.clone(),
            None => String::new(),
        }
    }

    #[wasm_bindgen(setter)]
    pub fn set_output(&mut self, output: String) {
        self.output = Some(output);
    }
}
#[wasm_bindgen]
pub fn run_tex_fmt(input: &str) -> TexFmtResult {
    let mut output: Option<String> = Some(String::new());
    let code = tex_fmt_lib::run(Some(input), &mut output);
    TexFmtResult {
        code: code,
        output: output,
    }
}
