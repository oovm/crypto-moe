use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn marysue_encode(text: &str) -> String {
    crypto_marysue::v2::encode(text)
}

#[wasm_bindgen]
pub fn marysue_decode(text: &str) -> String {
    crypto_marysue::v2::decode(text)
}

#[wasm_bindgen]
pub fn aaencode_encode(text: &str) -> String {
    crypto_aaencode::encode(text)
}
