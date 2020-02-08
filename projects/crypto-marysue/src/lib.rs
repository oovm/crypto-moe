extern crate encoding_rs;
extern crate flate2;
extern crate rand;
#[macro_use]
extern crate lazy_static;

pub mod auxiliary;

pub mod v1;
pub mod v2;

pub use v2::{decode, encode};
