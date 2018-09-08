#![forbid(unsafe_code)]

extern crate lalrpop_util;
extern crate toml;

/// NOTE: For interactive testing in `main`.
pub use grammar::FilterParser;

mod ast;
mod grammar;
