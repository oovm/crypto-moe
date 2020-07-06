/*!
This crate provides a library for encoding and decoding morse code
This crate's documentation provides some simple examples on how to use it.
*/

mod decoder;
mod encoder;
pub mod utils;

pub use decoder::decode;
pub use encoder::{encode, encode_raw};
