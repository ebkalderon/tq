use std::str;

use pom::parser::*;
use pom::Error as ParseError;

use self::expr::expr;
use crate::ast::{Expr, Filter};

mod expr;
mod stmt;
mod tokens;

pub fn parse_filter(filter: &str) -> Result<Expr, ParseError> {
    (expr() - end()).parse(filter.as_bytes())
}
