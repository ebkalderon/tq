pub use self::keywords::*;
pub use self::literal::{literal, string};

use std::iter;

use pom::char_class::{alpha, alphanum, multispace};
use pom::parser::*;

use crate::ast::tokens::{FnParam, Ident, IdentPath, Variable};

mod keywords;
mod literal;

pub fn space<'a>() -> Parser<'a, u8, ()> {
    let comment = sym(b'#') - none_of(b"\n\r").repeat(0..);
    let whitespace = is_a(multispace);
    (comment | whitespace).repeat(0..).discard()
}

pub fn identifier<'a>() -> Parser<'a, u8, Ident> {
    let first = is_a(|c| alpha(c) || c == b'_' || c == b'-');
    let rest = is_a(|c| alphanum(c) || c == b'_' || c == b'-');
    (first + rest.repeat(0..))
        .map(|(first, rest)| iter::once(first).chain(rest).collect())
        .convert(String::from_utf8)
        .map(Ident::from)
}

pub fn ident_path<'a>() -> Parser<'a, u8, IdentPath> {
    let single = !keyword() * (identifier() - !seq(b"::")).repeat(1);
    let multiple = (identifier() + (seq(b"::") * identifier()).repeat(1..))
        .map(|(first, rest)| iter::once(first).chain(rest).collect());

    (single | multiple).map(IdentPath::from)
}

pub fn variable<'a>() -> Parser<'a, u8, Variable> {
    sym(b'$') * identifier().map(Variable::from)
}

pub fn fn_param<'a>() -> Parser<'a, u8, FnParam> {
    let function = ident_path().map(FnParam::Function);
    let variable = variable().map(FnParam::Variable);
    function | variable
}
