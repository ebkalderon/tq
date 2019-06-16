pub use self::error::{FilterError, ModuleError};

use std::str::{self, FromStr};

use pom::parser::*;

use self::expr::{expr, function_decl};
use self::stmt::stmts;
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
    (tokens::space() * stmts() + expr() - end())
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
    let decls = function_decl().repeat(0..);
    (tokens::space() * stmts() + decls - tokens::space() - end())
        .map(|(stmts, decls)| Module::new(stmts, decls))
        .parse(text.as_bytes())
        .map_err(|e| ModuleError::new(e, text))
}
