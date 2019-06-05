use pom::parser::*;

use super::construct;
use super::{tokens, unary};
use crate::ast::{ExprBinding, ExprPattern};

pub fn pattern<'a>() -> Parser<'a, u8, ExprPattern> {
    let variable = tokens::variable().map(ExprPattern::Variable);
    let array = array();
    let table = table();
    tokens::space() * (variable | array | table) - tokens::space()
}

fn array<'a>() -> Parser<'a, u8, ExprPattern> {
    let patterns = list(call(pattern), sym(b','));
    (sym(b'[') * patterns - sym(b']')).map(ExprPattern::Array)
}

fn table<'a>() -> Parser<'a, u8, ExprPattern> {
    let assign = construct::table_key() + (sym(b'=') * call(pattern));
    (sym(b'{') * list(assign, sym(b',')) - sym(b'}')).map(ExprPattern::Table)
}

pub fn binding<'a>() -> Parser<'a, u8, Box<ExprBinding>> {
    (call(unary) + (seq(b"as") * call(pattern)))
        .map(|(expr, pat)| ExprBinding::new(expr, pat))
        .map(Box::from)
}
