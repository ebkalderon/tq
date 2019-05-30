use pom::parser::*;

use super::expr;
use crate::ast::{Expr, ExprIndex, ExprSlice};

pub fn index<'a>() -> Parser<'a, u8, Box<ExprIndex>> {
    let exact = (sym(b'[') * call(expr).opt() - sym(b']')).map(ExprIndex::Exact);
    let slice = index_slice().map(ExprIndex::Slice);
    (exact | slice).map(Box::from)
}

pub fn index_slice<'a>() -> Parser<'a, u8, ExprSlice> {
    let lower = (seq(b"[:") * call(expr) - sym(b']')).map(ExprSlice::Lower);
    let range_or_upper = ((sym(b'[') * call(expr)) + (sym(b':') * call(expr).opt() - sym(b']')))
        .map(|bounds| match bounds {
            (upper, None) => ExprSlice::Upper(upper),
            (upper, Some(lower)) => ExprSlice::Range(upper, lower),
        });
    lower | range_or_upper
}
