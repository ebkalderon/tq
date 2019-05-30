use std::iter;

use pom::parser::*;

use super::expr;
use crate::ast::{Expr, ExprIfElse, ExprTry};

pub fn control_flow<'a>() -> Parser<'a, u8, Expr> {
    if_else() | try_catch()
}

fn if_else<'a>() -> Parser<'a, u8, Expr> {
    let first_clause = seq(b"if") * call(expr) + (seq(b"then") * call(expr));
    let other_clauses = seq(b"elif") * call(expr) + (seq(b"then") * call(expr));
    let clauses = (first_clause + other_clauses.repeat(0..))
        .map(|(first, others)| iter::once(first).chain(others).collect());
    let fallback = seq(b"else") * call(expr) - seq(b"end");

    let block = (clauses + fallback).map(|(c, f)| ExprIfElse::new(c, f));
    block.map(Box::from).map(Expr::IfElse)
}

fn try_catch<'a>() -> Parser<'a, u8, Expr> {
    let block = seq(b"try") * call(expr) + (seq(b"catch") * call(expr)).opt();
    block.map(|(expr, catch)| Expr::Try(Box::new(ExprTry::new(expr, catch))))
}
