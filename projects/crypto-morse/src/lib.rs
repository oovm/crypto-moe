/*!
This crate provides a library for encoding and decoding morse code
This crate's documentation provides some simple examples on how to use it.
*/

mod decoder;
mod encoder;

pub use decoder::decode;
pub use encoder::{encode, encode_raw};

///
pub const LETTER: [char; 41] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
    'y', 'z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', ',', '!', '?', ' ',
];

///
pub const DOT_LINE: [&str; 41] = [
    "._", "_...", "_._.", "_..", ".", ".._.", "__.", "....", "..", ".___", "_._", "._..", "__", "_.", "___", ".__.", "__._",
    "._.", "...", "_", ".._", "..._", ".__", "_.._", "_.__", "__..", ".____", "..___", "...__", "...._", ".....", "_....",
    "__...", "___..", "____.", "_____", "._._._", "__..__", "_._.__", "..__..", "/",
];
