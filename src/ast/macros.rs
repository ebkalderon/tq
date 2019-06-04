//! Utility macros for constructing ASTs from literals.

/// Construct a `tq::ast::Expr` from a `tq` filter literal.
///
/// # Example
///
/// ```rust,edition2018
/// # use tq::tq;
/// #
/// let expr = tq!(.foo.bar["baz"]);
/// ```
#[macro_export]
macro_rules! tq {
    ( $($args:tt)+ ) => {
        {
            #[allow(unused_imports)]
            use $crate::ast::*;
            #[allow(unused_imports)]
            use $crate::ast::tokens::*;
            $crate::tq_expr_pipe!($($args)+)
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_expr_pipe {
    ( $($exprs:tt)|+ ) => {
        {
            vec![$($crate::tq_expr!($exprs)),+]
                .into_iter()
                .fold(None, |seq, next| {
                    if let Some(first) = seq {
                        Some(Expr::Binary(BinaryOp::Pipe, Box::new(first), Box::new(next)))
                    } else {
                        Some(next)
                    }
                })
                .unwrap()
        }
    };

    ( $($exprs:tt)* ) => {
        $crate::tq_expr!($($exprs)*);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_expr {
    // Parenthesized expressions.
    ( ($($expr:tt)+) ) => {
        $crate::tq_expr!($($expr)+)
    };

    // Unary operations.
    ( -$($expr:tt)+ ) => {
        Expr::Unary(UnaryOp::Neg, Box::new($crate::tq_expr!($($expr)+)))
    };

    ( !$($expr:tt)+ ) => {
        Expr::Unary(UnaryOp::Not, Box::new($crate::tq_expr!($($expr)+)))
    };

    // Filters.
    ( .. ) => {
        Expr::Filter(Box::new($crate::tq_filter!(..)))
    };

    ( .$($path:tt)* ) => {
        Expr::Filter(Box::new($crate::tq_filter!(.$($path)*)))
    };

    // Literal values.
    ( $literal:expr ) => {
        Expr::Literal($crate::tq_token!($literal))
    };

    // Field identifier.
    ( $i:ident ) => {
        Expr::Field($crate::tq_token!($i))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_expr_index {
    () => {
        ExprIndex::Iter
    };

    ( $upper:tt:$($lower:tt)+ ) => {
        ExprIndex::Slice(ExprSlice::Range($crate::tq_expr_pipe!($($upper)+), $crate::tq_expr_pipe!($($lower)+)))
    };

    ( $expr:tt: ) => {
        ExprIndex::Slice(ExprSlice::Lower($crate::tq_expr_pipe!($expr)))
    };

    ( :$($expr:tt)+ ) => {
        ExprIndex::Slice(ExprSlice::Upper($crate::tq_expr_pipe!($($expr)+)))
    };

    ( $($expr:tt)+ ) => {
        ExprIndex::Exact($crate::tq_expr_pipe!($($expr)+))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_filter {
    // Identity filter literal.
    (.) => {
        Filter::Identity
    };

    // Recurse filter literal.
    (..) => {
        Filter::Recurse
    };

    // Single field path.
    ( .$field:ident ) => {
        Filter::Field($crate::tq_token!($field))
    };

    // Single slice path.
    ( .[$($expr:tt)*] ) => {
        Filter::Index(Box::new($crate::tq_expr_index!($($expr)*)))
    };

    // Nested path beginning with an identifier-style field access.
    ( .$field:ident $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.$field), |seq, next| {
                Filter::Path(Box::new(seq), Box::new(next))
            })
    };

    // Nested path beginning with a slice-style field access.
    ( .[$($expr:tt)*] $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.[$($expr)*]), |seq, next| {
                Filter::Path(Box::new(seq), Box::new(next))
            })
    };

    // Final path segment is an identifier-style field.
    (@path .$field:ident) => {
        ::std::iter::once($crate::tq_filter!(.$field))
    };

    // Next path segment is an identifier-style field.
    (@path .$field:ident $($rest:tt)+) => {
        $crate::tq_filter!(@path .$field).chain($crate::tq_filter!(@path $($rest)+))
    };

    // Final path segment is a slice-style field.
    (@path [$($expr:tt)*]) => {
        ::std::iter::once($crate::tq_filter!(.[$($expr)*]))
    };

    // Next path segment is a slice-style field.
    (@path [$($expr:tt)*] $($rest:tt)+) => {
        $crate::tq_filter!(@path [$($expr)*]).chain($crate::tq_filter!(@path $($rest)+))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_token {
    ( false ) => {
        Literal::Boolean(false)
    };

    ( true ) => {
        Literal::Boolean(true)
    };

    ( $ident:ident ) => {
        Ident::from(stringify!($ident))
    };

    ( $item:expr ) => {
        Literal::from($item)
    };
}

/// Returns an AST constructed by `tq!()` and also the filter expression as a static string.
///
/// This is useful for testing whether the parser and the `tq!()` macro both produce the same AST.
///
/// # Example
///
/// ```rust,edition2018
/// # use tq::tq_expr_and_str;
/// # use tq::ast::{Expr, Filter};
/// #
/// let (expr, s) = tq_expr_and_str!(.);
/// assert_eq!(expr, Expr::Filter(Filter::Identity));
/// assert_eq!(s, ".");
/// ```
#[cfg(test)]
#[macro_export]
macro_rules! tq_expr_and_str {
    ($($expr:tt)+) => {
        (
            $crate::tq!($($expr)+),
            concat!($(stringify!($expr)),+)
        )
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn define_filter() {
        let identity = tq!(.foo.bar["hello"]);
        println!("{:?}", identity);
    }
}
