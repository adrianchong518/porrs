#![feature(variant_count)]

mod error;
mod lex;
mod parse;
mod program;
mod simulate;

pub use error::Error;
pub use program::Program;
pub use simulate::simulate;
