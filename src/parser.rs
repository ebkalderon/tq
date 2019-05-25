use std::str::{self, FromStr};

use pom::parser::*;
use pom::Error as ParseError;

use self::number::{float, integer};
use self::string::string;
use crate::ast::tokens::Literal;
use crate::ast::Filter;

mod number;
mod string;

pub fn parse_filter(filter: &str) -> Result<Literal, ParseError> {
    literal().parse(filter.as_bytes())
}

fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b"\t\r\n").repeat(0..).discard()
}

fn boolean<'a>() -> Parser<'a, u8, bool> {
    let boolean = seq(b"true") | seq(b"false");
    boolean.convert(str::from_utf8).convert(bool::from_str)
}

fn literal<'a>() -> Parser<'a, u8, Literal> {
    boolean().map(Literal::Boolean)
        | float().map(Literal::Float)
        | integer().map(Literal::Integer)
        | string().map(Literal::String)
}
