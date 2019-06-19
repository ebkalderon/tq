use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::many0;
use nom::sequence::{pair, preceded};
use nom::IResult;

use super::index::index;
use super::tokens::{identifier, variable};
use crate::ast::{Expr, ExprFilter};

pub fn filter(input: &str) -> IResult<&str, Expr> {
    let identity = map(char('.'), |_| ExprFilter::Identity);
    let recurse = map(tag(".."), |_| ExprFilter::Recurse);

    let index = map(index, |e| ExprFilter::Index(Box::new(e)));
    let segments = pair(first_segment, many0(alt((field, index))));
    let path = map(segments, |(first, rest)| {
        rest.into_iter().fold(first, |prev, next| {
            ExprFilter::Path(Box::new(prev), Box::new(next))
        })
    });

    let filter = alt((recurse, path, identity));
    map(filter, |expr| Expr::Filter(Box::new(expr)))(input)
}

fn first_segment(input: &str) -> IResult<&str, ExprFilter> {
    let identity_field = field;
    let identity_index = map(preceded(char('.'), index), |e| {
        ExprFilter::Index(Box::new(e))
    });
    let identity = alt((identity_field, identity_index));

    let var_field = pair(map(variable, ExprFilter::Variable), field);
    let var_index = pair(
        map(variable, ExprFilter::Variable),
        map(index, |e| ExprFilter::Index(Box::new(e))),
    );
    let variable = alt((var_field, var_index));
    let path = map(variable, |(var, seg)| {
        ExprFilter::Path(Box::new(var), Box::new(seg))
    });

    alt((identity, path))(input)
}

fn field(input: &str) -> IResult<&str, ExprFilter> {
    map(preceded(char('.'), identifier), ExprFilter::Field)(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;
    use crate::tq_expr_and_str;

    #[test]
    fn identity() {
        let (expected, path) = tq_expr_and_str!(.);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn recurse() {
        let (expected, path) = tq_expr_and_str!(..);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn iterate() {
        let (expected, path) = tq_expr_and_str!(.[]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.foo[]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"][]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_path() {
        let (expected, path) = tq_expr_and_str!(.foo);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn nested_path() {
        let (expected, path) = tq_expr_and_str!(.foo.bar.baz);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"]["bar"]["baz"]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"].bar["baz"]);
        let (_, actual) = all_consuming(filter)(&path).unwrap();
        assert_eq!(expected, actual);
    }
}
