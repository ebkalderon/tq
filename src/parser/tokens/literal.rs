pub use self::datetime::datetime;
pub use self::number::{float, integer};
pub use self::string::string;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::map;
use nom::IResult;

use crate::ast::tokens::Literal;

mod datetime;
mod number;
mod string;

pub fn literal(input: &str) -> IResult<&str, Literal> {
    let boolean = map(boolean, Literal::Boolean);
    let datetime = map(datetime, Literal::Datetime);
    let float = map(float, Literal::Float);
    let integer = map(integer, Literal::Integer);
    let string = map(string, Literal::String);
    alt((boolean, datetime, float, integer, string))(input)
}

pub fn boolean(input: &str) -> IResult<&str, bool> {
    let true_val = map(tag("true"), |_| true);
    let false_val = map(tag("false"), |_| false);
    alt((true_val, false_val))(input)
}

fn take_n(n: usize) -> impl Fn(&str) -> IResult<&str, &str> {
    move |input| take(n)(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    #[test]
    fn boolean_literals() {
        let (_, true_literal) = all_consuming(boolean)("true").unwrap();
        assert_eq!(true_literal, true);

        let (_, false_literal) = all_consuming(boolean)("false").unwrap();
        assert_eq!(false_literal, false);
    }
}
