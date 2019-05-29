use pom::parser::*;

use super::expr;
use crate::ast::{Expr, ExprTry};

pub fn try_catch<'a>() -> Parser<'a, u8, Box<ExprTry>> {
    let keyword = seq(b"try") * call(expr) + (seq(b"catch") * call(expr)).opt();
    keyword.map(|(expr, catch)| Box::new(ExprTry::new(expr, catch)))
}

pub fn try_operator<'a>(expr: Parser<'a, u8, Expr>) -> Parser<'a, u8, Expr> {
    (expr + sym(b'?').opt()).map(|(expr, is_try)| match is_try.is_some() {
        true => Expr::Try(Box::new(ExprTry::new(expr, None))),
        false => expr,
    })
}
