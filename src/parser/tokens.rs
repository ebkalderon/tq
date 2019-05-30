pub use self::literal::literal;

use std::iter;

use pom::char_class::{alpha, alphanum, multispace};
use pom::parser::*;

use crate::ast::tokens::{Ident, Variable};

mod literal;

pub fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

pub fn identifier<'a>() -> Parser<'a, u8, Ident> {
    (is_a(alpha) + (is_a(alphanum) | one_of(b"_-")).repeat(0..))
        .map(|(first, rest)| iter::once(first).chain(rest).collect())
        .convert(String::from_utf8)
        .map(Ident::from)
}

pub fn variable<'a>() -> Parser<'a, u8, Variable> {
    sym(b'$') * identifier().map(Variable::from)
}
