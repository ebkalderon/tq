use std::str::{self, FromStr};

use nom::combinator::all_consuming;
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated};

use self::expr::{expr, function_decl};
use self::stmt::stmts;
use crate::ast::{Filter, Module};

mod expr;
mod stmt;
mod tokens;

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_filter(s)
    }
}

pub fn parse_filter<S: AsRef<str>>(filter: S) -> Result<Filter, String> {
    let text = filter.as_ref();
    let stmts = terminated(stmts, tokens::space);
    all_consuming(preceded(tokens::space, pair(stmts, expr)))(text)
        .map(|(_, (stmts, expr))| Filter::new(stmts, expr))
        .map_err(|e| format!("{:?}", e))
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_module(s)
    }
}

pub fn parse_module<S: AsRef<str>>(module: S) -> Result<Module, String> {
    let text = module.as_ref();
    let stmts = terminated(stmts, tokens::space);
    let decls = many0(terminated(function_decl, tokens::space));
    all_consuming(delimited(tokens::space, pair(stmts, decls), tokens::space))(text)
        .map(|(_, (stmts, decls))| Module::new(stmts, decls))
        .map_err(|e| format!("{:?}", e))
}
