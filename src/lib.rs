#![forbid(unsafe_code)]

extern crate toml;
extern crate lalrpop_util;

/// NOTE: For interactive testing in `main`.
pub use grammar::FilterParser;

mod ast;
mod grammar;
