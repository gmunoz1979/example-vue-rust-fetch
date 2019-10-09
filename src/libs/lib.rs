#[macro_use]
extern crate lazy_static;

use wasm_bindgen::prelude::*;
use std::sync::Mutex;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn test(str: &str) {
    log(&str.to_string());
}