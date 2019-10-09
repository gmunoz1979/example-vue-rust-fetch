// #[macro_use]
// extern crate lazy_static;

// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use std::collections::HashMap;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub async fn run() -> Result<JsValue, JsValue> {
    let mut headers = HashMap::new();
    headers.insert("authorization", "Bearer ALsAavDy055dcNaLrCZNyA");

    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.headers(&JsValue::from_serde(&headers).unwrap());

    let request = Request::new_with_str_and_init(
        "https://tasks.test.halftau.com/rest/ten1/projects",
        &opts
    )?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    // log(&json.unwrap());

    Ok(json)
}