use std::str;

use pom::parser::*;
use pom::Error as ParseError;

use self::expr::{expr, function_decl};
use self::stmt::stmt;
use crate::ast::{Filter, Module};

mod expr;
mod stmt;
mod tokens;

pub fn parse_filter(filter: &str) -> Result<Filter, ParseError> {
    let stmts = stmt().repeat(0..);
    let expr = expr();
    (stmts + expr - end())
        .map(|(stmts, expr)| Filter::new(stmts, expr))
        .parse(filter.as_bytes())
}

pub fn parse_module(module: &str) -> Result<Module, ParseError> {
    let metadata = (seq(b"module") + tokens::space()) * expr() - tokens::space() - sym(b';');
    let stmts = (stmt() - tokens::space()).repeat(0..);
    let decls = (function_decl() - tokens::space()).repeat(1..);
    (metadata.opt() + stmts + decls - end())
        .map(|((meta, stmts), decls)| Module::new(meta, stmts, decls))
        .parse(module.as_bytes())
}
