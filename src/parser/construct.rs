use pom::parser::*;

use super::expr;
use crate::ast::Expr;

pub fn comma<'a>() -> Parser<'a, u8, Expr> {
    list(call(expr), sym(b',')).map(Expr::Comma)
}

pub fn construct<'a>() -> Parser<'a, u8, Expr> {
    array() | table()
}

fn array<'a>() -> Parser<'a, u8, Expr> {
    (sym(b'[') * list(call(expr), sym(b',')) - sym(b']'))
        .map(|mut exprs| match exprs.len() {
            1 => exprs.remove(0),
            _ => Expr::Comma(exprs),
        })
        .map(Box::from)
        .map(Expr::Array)
}

fn table<'a>() -> Parser<'a, u8, Expr> {
    let assign = call(expr) + (sym(b'=') * call(expr));
    (sym(b'{') * list(assign, sym(b',')) - sym(b'}')).map(Expr::Table)
}
