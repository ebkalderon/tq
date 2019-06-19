pub use self::keywords::*;
pub use self::literal::{literal, string};

use std::iter;

use nom::branch::alt;
use nom::bytes::complete::{is_a, tag};
use nom::character::complete::{alpha1, alphanumeric1, char, multispace1, not_line_ending};
use nom::combinator::{map, not, peek, recognize};
use nom::multi::{count, many0, many1};
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

use crate::ast::tokens::{FnParam, Ident, IdentPath, Variable};

mod keywords;
mod literal;

pub fn space(input: &str) -> IResult<&str, ()> {
    let comment = preceded(char('#'), not_line_ending);
    let whitespace = multispace1;
    map(many0(alt((comment, whitespace))), |_| ())(input)
}

pub fn identifier(input: &str) -> IResult<&str, Ident> {
    let first = count(alt((alpha1, is_a("_-"))), 1);
    let rest = many0(alt((alphanumeric1, is_a("_-"))));
    map(recognize(pair(first, rest)), Ident::from)(input)
}

pub fn ident_path(input: &str) -> IResult<&str, IdentPath> {
    let is_single_ident = terminated(identifier, peek(not(tag("::"))));
    let ident = preceded(peek(not(keyword)), is_single_ident);
    let single = map(ident, |x| vec![x]);

    let seq = pair(identifier, many1(preceded(tag("::"), identifier)));
    let multiple = map(seq, |(first, rest)| iter::once(first).chain(rest).collect());

    map(alt((single, multiple)), IdentPath::from)(input)
}

pub fn variable(input: &str) -> IResult<&str, Variable> {
    map(preceded(char('$'), identifier), Variable::from)(input)
}

pub fn fn_param(input: &str) -> IResult<&str, FnParam> {
    let function = map(ident_path, FnParam::Function);
    let variable = map(variable, FnParam::Variable);
    alt((function, variable))(input)
}
