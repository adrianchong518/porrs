#![feature(const_mut_refs)]

mod error;
mod lex;
mod op;
mod parse;
mod program;
mod simulate;
mod token;

pub use error::Error;
pub use error::Result;

pub use program::Program;
pub use simulate::simulate;
