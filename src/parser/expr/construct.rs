use pom::parser::*;

use super::tokens;
use super::{expr, unary};
use crate::ast::{BinaryOp, Expr, TableKey};

pub fn construct<'a>() -> Parser<'a, u8, Expr> {
    array() | table()
}

fn array<'a>() -> Parser<'a, u8, Expr> {
    (sym(b'[') * call(expr).opt() - sym(b']'))
        .map(|inner| inner.map(Box::from))
        .map(Expr::Array)
}

fn table<'a>() -> Parser<'a, u8, Expr> {
    let assign = table_key() + (sym(b'=') * table_value());
    (sym(b'{') * list(assign, sym(b',')) - sym(b'}')).map(Expr::Table)
}

pub fn table_key<'a>() -> Parser<'a, u8, TableKey> {
    let variable = tokens::variable().map(TableKey::Variable);
    let identifier = tokens::identifier().map(TableKey::Field);
    let literal = tokens::literal().map(TableKey::Literal);
    let expr = sym(b'(') * call(expr).map(TableKey::Expr) - sym(b')');
    tokens::space() * (variable | identifier | literal | expr) - tokens::space()
}

fn table_value<'a>() -> Parser<'a, u8, Expr> {
    let pipe = {
        let expr = call(unary) + (sym(b'|') * call(unary)).repeat(0..);
        expr.map(|(first, rest)| {
            rest.into_iter().fold(first, |lhs, rhs| {
                Expr::Binary(BinaryOp::Pipe, Box::from(lhs), Box::from(rhs))
            })
        })
    };

    let paren = sym(b'(') * call(table_value) - sym(b')');
    tokens::space() * (paren | pipe) - tokens::space()
}
