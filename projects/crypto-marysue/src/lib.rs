#![deny(missing_docs)]

//! doc me

#[allow(non_upper_case_globals)]
pub mod utils;

pub mod v1;
pub mod v2;

pub use v2::{decode, encode};
