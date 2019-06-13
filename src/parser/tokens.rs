pub use self::literal::{literal, string};

use std::iter;

use pom::char_class::{alpha, alphanum, multispace};
use pom::parser::*;

use crate::ast::tokens::{FnParam, Ident, IdentPath, Variable};

mod literal;

pub fn space<'a>() -> Parser<'a, u8, ()> {
    is_a(multispace).repeat(0..).discard()
}

pub fn identifier<'a>() -> Parser<'a, u8, Ident> {
    ((is_a(alpha) | one_of(b"_-")) + (is_a(alphanum) | one_of(b"_-")).repeat(0..))
        .map(|(first, rest)| iter::once(first).chain(rest).collect())
        .convert(String::from_utf8)
        .map(Ident::from)
}

pub fn ident_path<'a>() -> Parser<'a, u8, IdentPath> {
    (identifier() + (seq(b"::") * identifier()).repeat(0..))
        .map(|(first, rest)| iter::once(first).chain(rest))
        .map(IdentPath::from)
}

pub fn variable<'a>() -> Parser<'a, u8, Variable> {
    sym(b'$') * identifier().map(Variable::from)
}

pub fn fn_param<'a>() -> Parser<'a, u8, FnParam> {
    let function = ident_path().map(FnParam::Function);
    let variable = variable().map(FnParam::Variable);
    function | variable
}
