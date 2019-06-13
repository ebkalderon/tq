pub use self::error::FilterError;

use std::str::{self, FromStr};

use pom::parser::*;
use pom::Error as ParseError;

use self::expr::{expr, function_decl};
use self::stmt::stmt;
use crate::ast::{Filter, Module};

mod error;
mod expr;
mod stmt;
mod tokens;

impl FromStr for Filter {
    type Err = FilterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_filter(s)
    }
}

pub fn parse_filter<S: AsRef<str>>(filter: S) -> Result<Filter, FilterError> {
    let s = filter.as_ref();
    let stmts = stmt().repeat(0..);
    let expr = expr();
    (stmts + expr - end())
        .map(|(stmts, expr)| Filter::new(stmts, expr))
        .parse(s.as_bytes())
        .map_err(|e| FilterError::new(e, s))
}

impl FromStr for Module {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_module(s)
    }
}

pub fn parse_module<S: AsRef<str>>(module: S) -> Result<Module, ParseError> {
    let metadata = (seq(b"module") + tokens::space()) * expr() - tokens::space() - sym(b';');
    let stmts = (stmt() - tokens::space()).repeat(0..);
    let decls = (function_decl() - tokens::space()).repeat(1..);
    (metadata.opt() + stmts + decls - end())
        .map(|((meta, stmts), decls)| Module::new(meta, stmts, decls))
        .parse(module.as_ref().as_bytes())
}
