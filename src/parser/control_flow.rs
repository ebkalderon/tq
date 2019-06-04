use pom::parser::*;

use super::expr;
use crate::ast::{Expr, ExprIfElse, ExprTry};

pub fn control_flow<'a>() -> Parser<'a, u8, Expr> {
    if_else() | try_catch()
}

fn if_else<'a>() -> Parser<'a, u8, Expr> {
    let main_clause = seq(b"if") * call(expr) + (seq(b"then") * call(expr));
    let alt_clauses = (seq(b"elif") * call(expr) + (seq(b"then") * call(expr))).repeat(0..);
    let fallback = seq(b"else") * call(expr) - seq(b"end");

    (main_clause + alt_clauses + fallback)
        .map(|((main, alt), f)| ExprIfElse::new(main, alt, f))
        .map(Box::from)
        .map(Expr::IfElse)
}

fn try_catch<'a>() -> Parser<'a, u8, Expr> {
    let block = seq(b"try") * call(expr) + (seq(b"catch") * call(expr)).opt();
    block.map(|(expr, catch)| Expr::Try(Box::new(ExprTry::new(expr, catch))))
}
