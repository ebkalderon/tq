use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use super::{expr, pattern, tokens};
use crate::ast::{Expr, ExprForeach, ExprIfElse, ExprReduce, ExprTry};

pub fn control_flow(input: &str) -> IResult<&str, Expr> {
    alt((foreach, if_else, reduce, try_catch))(input)
}

fn foreach(input: &str) -> IResult<&str, Expr> {
    let key_foreach = pair(tokens::keyword_foreach, tokens::space);
    let bind = delimited(key_foreach, pattern::binding, tokens::space);
    let init = delimited(pair(char('('), tokens::space), expr, char(';'));
    let update = delimited(tokens::space, expr, pair(char(';'), tokens::space));
    let extract = terminated(expr, char(')'));

    let body = tuple((bind, init, update, extract));
    let expr = map(body, |(b, i, u, e)| ExprForeach::new(b, i, u, e));
    map(expr, |expr| Expr::Foreach(Box::new(expr)))(input)
}

fn if_else(input: &str) -> IResult<&str, Expr> {
    let key_if = pair(tokens::keyword_if, tokens::space);
    let key_then = pair(tokens::keyword_then, tokens::space);
    let main_clause = preceded(key_if, pair(expr, preceded(key_then, expr)));

    let key_elif = pair(tokens::keyword_elif, tokens::space);
    let key_then = pair(tokens::keyword_then, tokens::space);
    let alt_clauses = many0(preceded(key_elif, pair(expr, preceded(key_then, expr))));

    let key_else = pair(tokens::keyword_else, tokens::space);
    let key_end = pair(tokens::keyword_end, tokens::space);
    let fallback = preceded(key_else, terminated(expr, key_end));

    let block = tuple((main_clause, alt_clauses, fallback));
    let expr = map(block, |(main, alt, f)| ExprIfElse::new(main, alt, f));
    map(expr, |expr| Expr::IfElse(Box::new(expr)))(input)
}

fn reduce(input: &str) -> IResult<&str, Expr> {
    let key_reduce = pair(tokens::keyword_reduce, tokens::space);
    let bind = delimited(key_reduce, pattern::binding, tokens::space);
    let acc = delimited(pair(char('('), tokens::space), expr, char(';'));
    let eval = delimited(tokens::space, expr, char(')'));

    let body = tuple((bind, acc, eval));
    let expr = map(body, |(bind, acc, eval)| ExprReduce::new(bind, acc, eval));
    map(expr, |expr| Expr::Reduce(Box::new(expr)))(input)
}

fn try_catch(input: &str) -> IResult<&str, Expr> {
    let key_try = pair(tokens::keyword_try, tokens::space);
    let key_catch = pair(tokens::keyword_catch, tokens::space);
    let block = preceded(key_try, pair(expr, opt(preceded(key_catch, expr))));
    let expr = map(block, |(expr, catch)| ExprTry::new(expr, catch));
    map(expr, |expr| Expr::Try(Box::new(expr)))(input)
}
