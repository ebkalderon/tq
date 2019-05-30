use pom::parser::*;

use super::{expr, sum};
use crate::ast::Expr;

pub fn construct<'a>() -> Parser<'a, u8, Expr> {
    array() | table()
}

fn array<'a>() -> Parser<'a, u8, Expr> {
    (sym(b'[') * call(expr).opt() - sym(b']'))
        .map(|inner| inner.map(Box::from))
        .map(Expr::Array)
}

fn table<'a>() -> Parser<'a, u8, Expr> {
    let assign = call(expr) + (sym(b'=') * call(sum));
    (sym(b'{') * list(assign, sym(b',')) - sym(b'}')).map(Expr::Table)
}
