extern crate crypto_marysue;

use wasm_bindgen::prelude::*;
use crypto_marysue::v2::{encode, decode};


#[wasm_bindgen]
pub fn marysue_encode(text: &str) -> String {
    encode(text)
}

#[wasm_bindgen]
pub fn marysue_decode(text: &str) -> String {
    decode(text)
}
