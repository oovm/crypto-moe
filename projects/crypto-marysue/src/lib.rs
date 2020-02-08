extern crate encoding_rs;
extern crate flate2;
extern crate rand;
#[macro_use]
extern crate lazy_static;

pub mod auxiliary;
mod decoder;
mod encoder;

pub use decoder::decode;
pub use encoder::encode;
