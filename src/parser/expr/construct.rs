use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::{many0, separated_list};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

use super::tokens;
use super::{expr, term};
use crate::ast::{BinaryOp, Expr, TableKey, UnaryOp};

pub fn construct(input: &str) -> IResult<&str, Expr> {
    alt((array, table))(input)
}

fn array(input: &str) -> IResult<&str, Expr> {
    let expr = opt(map(expr, Box::new));
    let array = delimited(pair(char('['), tokens::space), expr, char(']'));
    map(array, Expr::Array)(input)
}

fn table(input: &str) -> IResult<&str, Expr> {
    let key = terminated(table_key, tokens::space);
    let value = terminated(table_value, tokens::space);
    let member = pair(key, preceded(pair(char('='), tokens::space), value));

    let members = separated_list(pair(char(','), tokens::space), member);
    let table = delimited(pair(char('{'), tokens::space), members, char('}'));

    map(table, Expr::Table)(input)
}

pub fn table_key(input: &str) -> IResult<&str, TableKey> {
    let variable = map(tokens::variable, TableKey::Variable);
    let identifier = map(tokens::identifier, TableKey::Field);
    let literal = map(tokens::literal, TableKey::Literal);
    let expr = map(
        delimited(pair(char('('), tokens::space), expr, char(')')),
        TableKey::Expr,
    );

    alt((variable, identifier, literal, expr))(input)
}

fn table_value(input: &str) -> IResult<&str, Expr> {
    let expr = pair(term, many0(preceded(pair(char('|'), tokens::space), term)));
    let pipe = map(expr, |(first, rest)| {
        rest.into_iter().fold(first, |lhs, rhs| {
            Expr::Binary(BinaryOp::Pipe, Box::new(lhs), Box::new(rhs))
        })
    });

    let expr = preceded(char('-'), table_value);
    let neg = map(expr, |e| Expr::Unary(UnaryOp::Neg, Box::new(e)));

    let paren = delimited(pair(char('('), tokens::space), table_value, char(')'));
    alt((paren, neg, pipe))(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;
    use crate::tq_expr_and_str;

    #[test]
    fn array() {
        let (expected, string) = tq_expr_and_str!([]);
        let (_, actual) = all_consuming(construct)(&string).unwrap();
        assert_eq!(expected, actual);

        let (expected, string) = tq_expr_and_str!([.foo.bar[]]);
        let (_, actual) = all_consuming(construct)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn table() {
        let (expected, string) = tq_expr_and_str!({});
        let (_, actual) = all_consuming(construct)(&string).unwrap();
        assert_eq!(expected, actual);

        let (expected, string) = tq_expr_and_str!({ one = 1, two = -func(12), three = foo | bar });
        let (_, actual) = all_consuming(construct)(&string).unwrap();
        assert_eq!(expected, actual);
    }
}
