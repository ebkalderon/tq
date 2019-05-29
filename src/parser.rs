use std::{iter, str};

use pom::char_class::{alpha, alphanum, multispace};
use pom::parser::*;
use pom::Error as ParseError;

use self::construct::{comma, construct};
use self::filter::filter;
use self::index::index_expr;
use self::literal::literal;
use self::try_catch::{try_catch, try_operator};
use crate::ast::{
    tokens::{Ident, Variable},
    Expr,
};

mod construct;
mod filter;
mod index;
mod literal;
mod try_catch;

pub fn parse_filter(filter: &str) -> Result<Expr, ParseError> {
    (expr() - end()).parse(filter.as_bytes())
}

fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

fn identifier<'a>() -> Parser<'a, u8, Ident> {
    (is_a(alpha) + (is_a(alphanum) | one_of(b"_-")).repeat(0..))
        .map(|(first, rest)| iter::once(first).chain(rest).collect())
        .convert(String::from_utf8)
        .map(Ident::from)
}

fn variable<'a>() -> Parser<'a, u8, Variable> {
    sym(b'$') * identifier().map(Variable::from)
}

fn expr<'a>() -> Parser<'a, u8, Expr> {
    let paren = sym(b'(') * call(expr) - sym(b')');
    let try_catch = try_catch().map(Expr::Try);
    let literal = literal().map(Expr::Literal);
    let field = identifier().map(Expr::Field);
    let variable = variable().map(Expr::Variable);
    let filter = filter();
    let construct = construct();

    let expr = paren | try_catch | literal | field | variable | filter | construct;
    let wrapped_expr = try_operator(index_expr(expr));

    space() * wrapped_expr - space()
}
