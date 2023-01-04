#![forbid(unsafe_code)]

pub mod bytecode;
mod parser;

pub use parser::parse;
