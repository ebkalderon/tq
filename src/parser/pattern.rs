use pom::parser::*;

use super::construct;
use super::{pipe, terms, tokens};
use crate::ast::{ExprBinding, ExprPattern};

pub fn pattern<'a>() -> Parser<'a, u8, ExprPattern> {
    let variable = tokens::variable().map(ExprPattern::Variable);
    let array = array();
    let table = table();
    variable | array | table
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
    let as_keyword = tokens::space() + seq(b"as") + tokens::space();
    (call(pipe) + (as_keyword * call(pattern)))
        .map(|(expr, pat)| ExprBinding::new(expr, pat))
        .map(Box::from)
}
