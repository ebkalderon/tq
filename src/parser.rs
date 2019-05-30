use std::{iter, str};

use pom::char_class::{alpha, alphanum, multispace};
use pom::parser::*;
use pom::Error as ParseError;

use self::construct::construct;
use self::control_flow::control_flow;
use self::filter::filter;
use self::index::index_slice;
use self::literal::literal;
use crate::ast::{
    tokens::{Ident, Variable},
    *,
};

mod construct;
mod control_flow;
mod filter;
mod index;
mod literal;

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
    pipe()
}

fn pipe<'a>() -> Parser<'a, u8, Expr> {
    let pipe = sym(b'|').map(|_| BinaryOp::Pipe);
    let expr = call(chain) + (pipe + call(chain)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn chain<'a>() -> Parser<'a, u8, Expr> {
    let comma = sym(b',').map(|_| BinaryOp::Comma);
    let expr = call(sum) + (comma + call(sum)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn sum<'a>() -> Parser<'a, u8, Expr> {
    let add = sym(b'+').map(|_| BinaryOp::Add);
    let sub = sym(b'-').map(|_| BinaryOp::Sub);
    let expr = call(product) + ((add | sub) + call(product)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn product<'a>() -> Parser<'a, u8, Expr> {
    let alt = seq(b"//").map(|_| BinaryOp::Alt);
    let mul = sym(b'*').map(|_| BinaryOp::Mul);
    let div = sym(b'/').map(|_| BinaryOp::Div);
    let rem = sym(b'%').map(|_| BinaryOp::Mod);
    let expr = call(unary) + ((alt | mul | div | rem) + call(unary)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn unary<'a>() -> Parser<'a, u8, Expr> {
    let pos = sym(b'+').map(|_| UnaryOp::Pos);
    let neg = sym(b'-').map(|_| UnaryOp::Neg);
    let expr = (pos | neg).opt() + call(index);
    let expr = space() * expr - space();
    expr.map(|(unary, term)| match unary {
        Some(op) => Expr::Unary(op, Box::from(term)),
        None => term,
    })
}

fn index<'a>() -> Parser<'a, u8, Expr> {
    let exact = (sym(b'[') * call(expr).opt() - sym(b']')).map(ExprIndex::Exact);
    let slice = index_slice().map(ExprIndex::Slice);
    let expr = call(try_postfix) + (exact | slice).opt();
    expr.map(|(term, index)| match index {
        Some(index) => Expr::Index(Box::from(term), Box::from(index)),
        None => term,
    })
}

fn try_postfix<'a>() -> Parser<'a, u8, Expr> {
    let expr = call(terms) + sym(b'?').opt();
    expr.map(|(term, is_try)| match is_try {
        Some(_) => Expr::Try(Box::new(ExprTry::new(term, None))),
        None => term,
    })
}

fn terms<'a>() -> Parser<'a, u8, Expr> {
    let paren = sym(b'(') * call(expr) - sym(b')');
    let control_flow = control_flow();
    let filter = filter();
    let construct = construct();
    let literal = literal().map(Expr::Literal);
    let field = identifier().map(Expr::Field);
    let variable = variable().map(Expr::Variable);
    paren | control_flow | filter | construct | literal | field | variable
}
