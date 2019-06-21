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

    macro_rules! tq_literal_and_str {
        ($literal:expr) => {{
            let (expr, string) = $crate::tq_expr_and_str!($literal);
            match expr {
                $crate::ast::Expr::Literal(lit) => (lit, string),
                e => panic!(format!(
                    "tq_expr_and_str!() did not produce a `Literal`: {:?}",
                    e
                )),
            }
        }};
    }

    #[test]
    fn boolean() {
        let (expected, string) = tq_literal_and_str!(false);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);

        let (expected, string) = tq_literal_and_str!(true);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn float() {
        let (expected, string) = tq_literal_and_str!(12.5);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);

        let (expected, string) = tq_literal_and_str!(12E6);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);

        let (expected, string) = tq_literal_and_str!(12.5E6);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn integer() {
        let (expected, string) = tq_literal_and_str!(1234);
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn string() {
        let (expected, string) = tq_literal_and_str!("hello world\n");
        let (_, actual) = all_consuming(literal)(&string).unwrap();
        assert_eq!(expected, actual);
    }
}
