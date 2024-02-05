mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &str) {
    alert(&format!("Hello, from {msg}!"));
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    greet("inside Rust");

    Ok(())
}