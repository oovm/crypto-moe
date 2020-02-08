extern crate convert_base;
extern crate rand;

pub mod aligned;
pub mod dense;

pub use aligned::SecretAligned;
pub use dense::SecretDense;
