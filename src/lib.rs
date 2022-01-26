use wasm_bindgen::prelude::*;

#[wasm_bindgen(raw_module = "../main")]
extern {
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    log(name);
}
