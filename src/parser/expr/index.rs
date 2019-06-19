use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt, recognize};
use nom::sequence::{delimited, pair, preceded, tuple};
use nom::IResult;

use super::{expr, tokens};
use crate::ast::{ExprIndex, ExprSlice};

pub fn index(input: &str) -> IResult<&str, ExprIndex> {
    let iter = map(pair(left_brace, char(']')), |_| ExprIndex::Iter);
    let exact = map(delimited(left_brace, expr, char(']')), ExprIndex::Exact);
    let slice = map(index_slice, ExprIndex::Slice);
    alt((iter, exact, slice))(input)
}

pub fn index_slice(input: &str) -> IResult<&str, ExprSlice> {
    let empty_upper = tuple((left_brace, char(':'), tokens::space));
    let lower = map(delimited(empty_upper, expr, char(']')), ExprSlice::Lower);

    let opt_lower = delimited(tokens::space, opt(expr), char(']'));
    let range = pair(preceded(left_brace, expr), preceded(char(':'), opt_lower));
    let range_or_upper = map(range, |(upper, lower)| match lower {
        Some(lower) => ExprSlice::Range(upper, lower),
        None => ExprSlice::Upper(upper),
    });

    alt((lower, range_or_upper))(input)
}

fn left_brace(input: &str) -> IResult<&str, &str> {
    recognize(pair(char('['), tokens::space))(input)
}
