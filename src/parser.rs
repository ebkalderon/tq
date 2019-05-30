use std::str;

use pom::parser::*;
use pom::Error as ParseError;

use self::construct::construct;
use self::control_flow::control_flow;
use self::filter::filter;
use self::index::index_slice;
use crate::ast::*;

mod construct;
mod control_flow;
mod filter;
mod index;
mod tokens;

pub fn parse_filter(filter: &str) -> Result<Expr, ParseError> {
    (tokens::space() * expr() - end()).parse(filter.as_bytes())
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
    let expr = call(logical) + (comma + call(logical)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn logical<'a>() -> Parser<'a, u8, Expr> {
    let and = seq(b"and").map(|_| BinaryOp::And);
    let or = seq(b"or").map(|_| BinaryOp::Or);
    let expr = call(compare) + ((and | or) + call(compare)).repeat(0..);
    expr.map(|(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::from(lhs), Box::from(rhs))
        })
    })
}

fn compare<'a>() -> Parser<'a, u8, Expr> {
    let eq = seq(b"==").map(|_| BinaryOp::Eq);
    let neq = seq(b"!=").map(|_| BinaryOp::NotEq);
    let equality = eq | neq;

    let lte = seq(b"<=").map(|_| BinaryOp::LessThanEq);
    let lt = sym(b'<').map(|_| BinaryOp::LessThan);
    let gte = seq(b">=").map(|_| BinaryOp::LessThanEq);
    let gt = sym(b'>').map(|_| BinaryOp::LessThan);
    let comparison = lte | lt | gte | gt;

    let expr = call(sum) + ((equality | comparison) + call(sum)).repeat(0..);
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
    let neg = sym(b'-').map(|_| UnaryOp::Neg);
    let not = sym(b'!').map(|_| UnaryOp::Not);
    let expr = (neg | not).opt() + call(index);
    let expr = tokens::space() * expr - tokens::space();
    expr.map(|(unary, term)| match unary {
        Some(op) => Expr::Unary(op, Box::from(term)),
        None => term,
    })
}

fn index<'a>() -> Parser<'a, u8, Expr> {
    let iter = seq(b"[]").map(|_| ExprIndex::Iter);
    let exact = (sym(b'[') * call(expr) - sym(b']')).map(ExprIndex::Exact);
    let slice = index_slice().map(ExprIndex::Slice);
    let expr = call(try_postfix) + (iter | exact | slice).opt();
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
    let literal = tokens::literal().map(Expr::Literal);
    let variable = tokens::variable().map(Expr::Variable);
    let field = tokens::identifier().map(Expr::Field);
    paren | control_flow | filter | construct | literal | variable | field
}
