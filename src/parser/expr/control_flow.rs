use pom::parser::*;

use super::{expr, pattern, tokens};
use crate::ast::{Expr, ExprForeach, ExprIfElse, ExprReduce, ExprTry};

pub fn control_flow<'a>() -> Parser<'a, u8, Expr> {
    foreach() | if_else() | reduce() | try_catch()
}

fn foreach<'a>() -> Parser<'a, u8, Expr> {
    let bind = tokens::keyword_foreach() * pattern::binding() - sym(b'(');
    let init = call(expr) - sym(b';');
    let update = call(expr) - sym(b';');
    let extract = call(expr) - sym(b')');

    let body = bind + init + update + extract;
    body.map(|(((bind, init), update), extract)| ExprForeach::new(bind, init, update, extract))
        .map(Box::from)
        .map(Expr::Foreach)
}

fn if_else<'a>() -> Parser<'a, u8, Expr> {
    use super::tokens::{keyword_elif, keyword_else, keyword_end, keyword_if, keyword_then};

    let main_clause = keyword_if() * call(expr) + (keyword_then() * call(expr));
    let alt_clauses = (keyword_elif() * call(expr) + (keyword_then() * call(expr))).repeat(0..);
    let fallback = keyword_else() * call(expr) - keyword_end();

    (main_clause + alt_clauses + fallback)
        .map(|((main, alt), f)| ExprIfElse::new(main, alt, f))
        .map(Box::from)
        .map(Expr::IfElse)
}

fn reduce<'a>() -> Parser<'a, u8, Expr> {
    let bind = tokens::keyword_reduce() * pattern::binding() - sym(b'(');
    let acc = call(expr) - sym(b';');
    let eval = call(expr) - sym(b')');

    let body = bind + acc + eval;
    body.map(|((bind, acc), eval)| ExprReduce::new(bind, acc, eval))
        .map(Box::from)
        .map(Expr::Reduce)
}

fn try_catch<'a>() -> Parser<'a, u8, Expr> {
    let block = tokens::keyword_try() * call(expr) + (tokens::keyword_catch() * call(expr)).opt();
    block.map(|(expr, catch)| Expr::Try(Box::new(ExprTry::new(expr, catch))))
}
