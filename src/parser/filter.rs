use pom::parser::*;

use super::index::index;
use super::tokens::identifier;
use crate::ast::{Expr, Filter};

pub fn filter<'a>() -> Parser<'a, u8, Expr> {
    let identity = sym(b'.').map(|_| Filter::Identity);
    let recurse = seq(b"..").map(|_| Filter::Recurse);

    let first = field() | (sym(b'.') * index().map(Filter::Index));
    let segments = first + (field() | index().map(Filter::Index)).repeat(0..);
    let path = segments.map(|(first, rest)| {
        rest.into_iter().fold(first, |prev, next| {
            Filter::Path(Box::new(prev), Box::new(next))
        })
    });

    (recurse | path | identity).map(|e| Expr::Filter(Box::new(e)))
}

fn field<'a>() -> Parser<'a, u8, Filter> {
    sym(b'.') * identifier().map(Filter::Field)
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
