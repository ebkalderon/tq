pub use self::datetime::datetime;
pub use self::number::{float, integer};
pub use self::string::string;

use std::str::{self, FromStr};

use pom::parser::*;

use crate::ast::tokens::Literal;

mod datetime;
mod number;
mod string;

pub fn literal<'a>() -> Parser<'a, u8, Literal> {
    boolean().map(Literal::Boolean)
        | datetime().map(Literal::Datetime)
        | float().map(Literal::Float)
        | integer().map(Literal::Integer)
        | string().map(Literal::String)
}

pub fn boolean<'a>() -> Parser<'a, u8, bool> {
    let boolean = seq(b"true") | seq(b"false");
    boolean.convert(str::from_utf8).convert(bool::from_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boolean_literals() {
        let true_literal = boolean().parse(b"true").unwrap();
        assert_eq!(true_literal, true);

        let false_literal = boolean().parse(b"false").unwrap();
        assert_eq!(false_literal, false);
    }
}
