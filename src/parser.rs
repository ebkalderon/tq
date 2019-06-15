pub use self::error::{FilterError, ModuleError};

use std::str::{self, FromStr};

use pom::parser::*;

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
    let text = filter.as_ref();
    let stmts = stmt().repeat(0..);
    let expr = expr();
    (stmts + expr - end())
        .map(|(stmts, expr)| Filter::new(stmts, expr))
        .parse(text.as_bytes())
        .map_err(|e| FilterError::new(e, text))
}

impl FromStr for Module {
    type Err = ModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_module(s)
    }
}

pub fn parse_module<S: AsRef<str>>(module: S) -> Result<Module, ModuleError> {
    let text = module.as_ref();
    let meta = (tokens::space() + tokens::keyword_module()) * expr() - sym(b';');
    let stmts = stmt().repeat(0..);
    let decls = function_decl().repeat(0..);
    (meta.opt() + stmts + decls - tokens::space() - end())
        .map(|((meta, stmts), decls)| Module::new(meta, stmts, decls))
        .parse(text.as_bytes())
        .map_err(|e| ModuleError::new(e, text))
}
