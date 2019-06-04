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
    (.) => {
        Filter::Identity
    };

    (..) => {
        Filter::Recurse
    };

    ( .$field:ident ) => {
        Filter::Field($crate::tq_token!($field))
    };

    ( .[$($expr:tt)*] ) => {
        Filter::Index(Box::new($crate::tq_expr_index!($($expr)*)))
    };

    ( .$field:ident $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.$field), |seq, next| {
                Filter::Path(Box::new(seq), Box::new(next))
            })
    };

    ( .[$($expr:tt)*] $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.[$($expr)*]), |seq, next| {
                Filter::Path(Box::new(seq), Box::new(next))
            })
    };

    (@path .$field:ident) => {
        ::std::iter::once($crate::tq_filter!(.$field))
    };

    (@path .$field:ident $($rest:tt)+) => {
        $crate::tq_filter!(@path .$field).chain($crate::tq_filter!(@path $($rest)+))
    };

    (@path [$($expr:tt)*]) => {
        ::std::iter::once($crate::tq_filter!(.[$($expr)*]))
    };

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

#[cfg(test)]
mod tests {
    #[test]
    fn define_filter() {
        let identity = tq!(.foo.bar["hello"]);
        println!("{:?}", identity);
    }
}
