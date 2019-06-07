use pom::parser::*;

use super::index::index;
use super::tokens::identifier;
use crate::ast::{Expr, ExprFilter};

pub fn filter<'a>() -> Parser<'a, u8, Expr> {
    let identity = sym(b'.').map(|_| ExprFilter::Identity);
    let recurse = seq(b"..").map(|_| ExprFilter::Recurse);

    let first = field() | (sym(b'.') * index().map(ExprFilter::Index));
    let segments = first + (field() | index().map(ExprFilter::Index)).repeat(0..);
    let path = segments.map(|(first, rest)| {
        rest.into_iter().fold(first, |prev, next| {
            ExprFilter::Path(Box::new(prev), Box::new(next))
        })
    });

    (recurse | path | identity).map(|e| Expr::Filter(Box::new(e)))
}

fn field<'a>() -> Parser<'a, u8, ExprFilter> {
    sym(b'.') * identifier().map(ExprFilter::Field)
}

#[cfg(test)]
mod tests {
    use crate::tq_expr_and_str;

    use super::*;

    #[test]
    fn identity() {
        let (expected, path) = tq_expr_and_str!(.);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn recurse() {
        let (expected, path) = tq_expr_and_str!(..);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn iterate() {
        let (expected, path) = tq_expr_and_str!(.[]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.foo[]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"][]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn simple_path() {
        let (expected, path) = tq_expr_and_str!(.foo);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn nested_path() {
        let (expected, path) = tq_expr_and_str!(.foo.bar.baz);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"]["bar"]["baz"]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, path) = tq_expr_and_str!(.["foo"].bar["baz"]);
        let actual = filter().parse(path.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}
