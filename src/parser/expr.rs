//! TODO: Need to improve Filter parsing and AST design.

pub use self::function::function_decl;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use self::construct::construct;
use self::control_flow::control_flow;
use self::filter::filter;
use self::function::function_call;
use self::label::{label_break, label_decl};
use super::tokens;
use crate::ast::*;

mod construct;
mod control_flow;
mod filter;
mod function;
mod index;
mod label;
mod pattern;

pub fn expr(input: &str) -> IResult<&str, Expr> {
    terminated(pipe, tokens::space)(input)
}

fn pipe(input: &str) -> IResult<&str, Expr> {
    let pipe = pair(char('|'), tokens::space);
    let expr = pair(chain, many0(preceded(pipe, chain)));
    map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, rhs| {
            Expr::Binary(BinaryOp::Pipe, Box::new(lhs), Box::new(rhs))
        })
    })(input)
}

fn chain(input: &str) -> IResult<&str, Expr> {
    let comma = pair(char(','), tokens::space);
    let expr = pair(fn_decl, many0(preceded(comma, fn_decl)));
    map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, rhs| {
            Expr::Binary(BinaryOp::Comma, Box::new(lhs), Box::new(rhs))
        })
    })(input)
}

fn fn_decl(input: &str) -> IResult<&str, Expr> {
    let expr = pair(many0(terminated(function_decl, tokens::space)), binding);
    map(expr, |(decls, expr)| {
        decls.into_iter().rev().fold(expr, |expr, decl| {
            Expr::FnDecl(Box::new(decl), Box::new(expr))
        })
    })(input)
}

fn binding(input: &str) -> IResult<&str, Expr> {
    let pipe = tuple((tokens::space, char('|'), tokens::space));
    let expr = pair(many0(terminated(pattern::binding, pipe)), label);
    map(expr, |(bindings, expr)| {
        bindings.into_iter().rev().fold(expr, |expr, binding| {
            Expr::Binding(Box::new(binding), Box::new(expr))
        })
    })(input)
}

fn label(input: &str) -> IResult<&str, Expr> {
    let label = map(label_decl, Expr::Label);
    let label = terminated(label, pair(char('|'), tokens::space));
    let expr = pair(many0(label), assign);
    map(expr, |(decls, expr)| {
        decls.into_iter().rev().fold(expr, |expr, label| {
            Expr::Binary(BinaryOp::Pipe, Box::new(label), Box::new(expr))
        })
    })(input)
}

fn assign(input: &str) -> IResult<&str, Expr> {
    let pipe = map(char('|'), |_| BinaryOp::Pipe);
    let add = map(char('+'), |_| BinaryOp::Add);
    let sub = map(char('-'), |_| BinaryOp::Sub);
    let mul = map(char('*'), |_| BinaryOp::Mul);
    let alt_ = map(tag("//"), |_| BinaryOp::Alt);
    let div = map(char('/'), |_| BinaryOp::Div);
    let rem = map(char('%'), |_| BinaryOp::Mod);

    let op = terminated(opt(alt((pipe, add, sub, mul, alt_, div, rem))), char('='));
    let expr = pair(logical, opt(pair(terminated(op, tokens::space), logical)));
    map(expr, |(expr, assign)| match assign {
        Some((Some(op), value)) => Expr::AssignOp(op, Box::new(expr), Box::new(value)),
        Some((None, value)) => Expr::Assign(Box::new(expr), Box::new(value)),
        None => expr,
    })(input)
}

fn logical(input: &str) -> IResult<&str, Expr> {
    let and = map(tag("and "), |_| BinaryOp::And);
    let or = map(tag("or "), |_| BinaryOp::Or);
    let op = terminated(alt((and, or)), tokens::space);
    let expr = pair(compare, many0(pair(op, compare)));
    map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::new(lhs), Box::new(rhs))
        })
    })(input)
}

fn compare(input: &str) -> IResult<&str, Expr> {
    let eq = map(tag("=="), |_| BinaryOp::Eq);
    let neq = map(tag("!="), |_| BinaryOp::NotEq);
    let equality = alt((eq, neq));

    let lte = map(tag("<="), |_| BinaryOp::LessThanEq);
    let lt = map(char('<'), |_| BinaryOp::LessThan);
    let gte = map(tag(">="), |_| BinaryOp::LessThanEq);
    let gt = map(char('>'), |_| BinaryOp::LessThan);
    let comparison = alt((lte, lt, gte, gt));

    let op = terminated(alt((equality, comparison)), tokens::space);
    let expr = pair(sum, opt(pair(op, sum)));
    map(expr, |(expr, cmp)| match cmp {
        Some((op, rhs)) => Expr::Binary(op, Box::new(expr), Box::new(rhs)),
        None => expr,
    })(input)
}

fn sum(input: &str) -> IResult<&str, Expr> {
    let add = map(char('+'), |_| BinaryOp::Add);
    let sub = map(char('-'), |_| BinaryOp::Sub);
    let op = terminated(alt((add, sub)), tokens::space);
    let expr = pair(product, many0(pair(op, product)));
    map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::new(lhs), Box::new(rhs))
        })
    })(input)
}

fn product(input: &str) -> IResult<&str, Expr> {
    let alt_ = map(tag("//"), |_| BinaryOp::Alt);
    let mul = map(char('*'), |_| BinaryOp::Mul);
    let div = map(char('/'), |_| BinaryOp::Div);
    let rem = map(char('%'), |_| BinaryOp::Mod);
    let op = terminated(alt((alt_, mul, div, rem)), tokens::space);
    let expr = pair(try_postfix, many0(pair(op, try_postfix)));
    map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, (op, rhs)| {
            Expr::Binary(op, Box::new(lhs), Box::new(rhs))
        })
    })(input)
}

fn try_postfix(input: &str) -> IResult<&str, Expr> {
    let expr = terminated(pair(unary, many0(char('?'))), tokens::space);
    map(expr, |(expr, tries)| match !tries.is_empty() {
        true => Expr::Try(Box::new(ExprTry::new(expr, None))),
        false => expr,
    })(input)
}

fn unary(input: &str) -> IResult<&str, Expr> {
    let neg = map(char('-'), |_| UnaryOp::Neg);
    let not = map(char('!'), |_| UnaryOp::Not);
    let expr = pair(opt(alt((neg, not))), block);
    map(expr, |(unary, expr)| match unary {
        Some(op) => Expr::Unary(op, Box::new(expr)),
        None => expr,
    })(input)
}

fn block(input: &str) -> IResult<&str, Expr> {
    terminated(alt((control_flow, index)), tokens::space)(input)
}

fn index(input: &str) -> IResult<&str, Expr> {
    let index = pair(index::index, map(opt(char('?')), |x| x.is_some()));
    let expr = pair(term, many0(index));
    map(expr, |(expr, index)| {
        index.into_iter().fold(expr, |expr, (index, with_try)| {
            let index = Expr::Index(Box::new(expr), Box::new(index));
            if with_try {
                Expr::Try(Box::new(ExprTry::new(index, None)))
            } else {
                index
            }
        })
    })(input)
}

fn term(input: &str) -> IResult<&str, Expr> {
    let paren = delimited(pair(char('('), tokens::space), expr, char(')'));
    let paren = map(paren, |e| Expr::Paren(Box::new(e)));
    let literal = map(tokens::literal, Expr::Literal);
    let brk = map(label_break, Expr::Break);
    let empty = map(tag("empty"), |_| Expr::Empty);
    let fn_call = map(function_call, Expr::FnCall);
    let variable = map(tokens::variable, Expr::Variable);
    alt((
        paren, literal, brk, empty, filter, construct, variable, fn_call,
    ))(input)
}
