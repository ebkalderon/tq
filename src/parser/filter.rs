use std::iter;

use pom::parser::*;

use super::identifier;
use super::index::index;
use crate::ast::{Expr, Filter};

pub fn filter<'a>() -> Parser<'a, u8, Expr> {
    let identity = sym(b'.').map(|_| Filter::Identity);
    let recurse = seq(b"..").map(|_| Filter::Recurse);

    let first = field() | (sym(b'.') * index().map(Filter::Index));
    let segments = first + (field() | index().map(Filter::Index)).repeat(0..);
    let filter = segments.map(concat_filter_path);

    (filter | recurse | identity).map(|e| Expr::Filter(Box::new(e)))
}

fn field<'a>() -> Parser<'a, u8, Filter> {
    sym(b'.') * identifier().map(Filter::Field)
}

fn concat_filter_path(filter_segments: (Filter, Vec<Filter>)) -> Filter {
    let (first, remaining) = filter_segments;
    if remaining.is_empty() {
        first
    } else {
        Filter::Path(iter::once(first).chain(remaining).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::ast::tokens::Ident;

    #[test]
    fn identity() {
        let expr = filter().parse(".".as_bytes()).unwrap();
        assert_eq!(expr, Expr::Filter(Box::new(Filter::Identity)));
    }

    #[test]
    fn recurse() {
        let expr = filter().parse("..".as_bytes()).unwrap();
        assert_eq!(expr, Expr::Filter(Box::new(Filter::Recurse)));
    }

    #[test]
    fn simple_path() {
        let simple_path = Expr::Filter(Box::new(Filter::Field(Ident::from("foo"))));
        let expr = filter().parse(".foo".as_bytes()).unwrap();
        assert_eq!(expr, simple_path);
    }

    #[test]
    fn nested_path() {
        let complex_path = Expr::Filter(Box::new(Filter::Path(vec![
            Filter::Field(Ident::from("foo")),
            Filter::Field(Ident::from("bar")),
            Filter::Field(Ident::from("baz")),
        ])));
        let expr = filter().parse(".foo.bar.baz".as_bytes()).unwrap();
        assert_eq!(expr, complex_path);
    }
}
