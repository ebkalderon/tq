//! Utility macros for constructing ASTs from literals.

/// Construct an AST from a `tq` filter literal.
///
/// If any statements are present, this will return a `tq::ast::Filter`. Otherwise, this will
/// return a `tq::ast::Expr`.
///
/// # Limitations
///
/// Due to limitations of `macro_rules` in Rust, certain complex expressions might need extra
/// parentheses to reduce ambiguity and aid parsing.
///
/// Also, if the compiler complains about hitting a certain recursion limit, try adding the
/// following module attribute to the root file of your crate:
///
/// ```rust,ignore
/// #![recursion_limit = "128"]
/// ```
///
/// # Example
///
/// ```rust,edition2018
/// # #![recursion_limit = "128"]
/// # use tq::tq;
/// let expr = tq!(include "foo"; . as $var | .first["second"][$var]?);
/// ```
#[macro_export]
macro_rules! tq {
    // Detect if the sequence begins with a `module`, `import`, or `include` statement and start
    // recursing into `@stmt`, which will eventually return a `Filter`. Otherwise, fall back to the
    // `$($expr:tt)+` case and return an `Expr` instead.

    (module $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt (module $meta;) $($rest)+)
    };

    (import $file:tt as $($path:ident)::+ ; $($rest:tt)+) => {
        $crate::tq!(@stmt (import $file as $($path)::+;) $($rest)+)
    };

    (import $file:tt as $($path:ident)::+ $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt (import $file as $($path)::+ $meta;) $($rest)+)
    };

    (import $file:tt as $dollar:tt $var:ident ; $($rest:tt)+) => {
        $crate::tq!(@stmt (import $file as $dollar$var;) $($rest)+)
    };

    (import $file:tt as $dollar:tt $var:ident $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt (import $file as $dollar$var $meta;) $($rest)+)
    };

    (include $file:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt (include $file;) $($rest)+)
    };

    (include $file:tt $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt (include $file $meta;) $($rest)+)
    };

    // Beginning of the recursive `@stmt` routine. This will fold over all subsequent statements
    // and accumulate them in between `()` in the beginning, of each recursive `tq!()` invocation.
    //
    // Once all the statements are neatly nested in between parentheses in the bottom-most `@stmt`
    // case, we can do the following:
    //
    // 1. Glob match everything between the parentheses and pass it straight to `tq_stmts!()`.
    // 2. Everything after the parentheses is passed to `tq_expr!()`.
    //
    // The results of both macro invocations are used to create a `Filter` struct which we return
    // to the user.

    (@stmt ($($stmts:tt)+) import $file:tt as $($path:ident)::+ ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ import $file as $($path)::+;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) import $file:tt as $($path:ident)::+ $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ import $file as $($path)::+ $meta;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) import $file:tt as $dollar:tt $var:ident ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ import $file as $dollar$var;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) import $file:tt as $dollar:tt $var:ident $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ import $file as $dollar$var $meta;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) include $file:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ include $file;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) include $file:tt $meta:tt ; $($rest:tt)+) => {
        $crate::tq!(@stmt ($($stmts)+ include $file $meta;) $($rest)+)
    };

    (@stmt ($($stmts:tt)+) $($expr:tt)+) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        Filter::new($crate::tq_stmts!($($stmts)+), $crate::tq_expr!($($expr)+))
    }};

    // This is the happy path where the user has supplied an expression without any `module` or
    // `import` statements. This case returns a simple `Expr`.

    ($($expr:tt)+) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_expr!($($expr)+)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_stmts {
    (@stmt) => {
        ::std::iter::empty()
    };

    (@stmt import $file:tt as $($path:ident)::+ ; $($rest:tt)*) => {{
        let stmt = StmtImportMod::new($file.into(), $crate::tq_token!(::$($path)::+), None);
        let first = ::std::iter::once(Stmt::ImportMod(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (@stmt import $file:tt as $($path:ident)::+ $meta:tt ; $($rest:tt)*) => {{
        let module = Some($crate::tq_expr!($meta));
        let stmt = StmtImportMod::new($file.into(), $crate::tq_token!(::$($path)::+), module);
        let first = ::std::iter::once(Stmt::ImportMod(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (@stmt import $file:tt as $dollar:tt $var:ident ; $($rest:tt)*) => {{
        let var = $crate::tq_token!($dollar$var);
        let stmt = StmtImportToml::new($file.into(), var, None);

        let first = ::std::iter::once(Stmt::ImportToml(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (@stmt import $file:tt as $dollar:tt $var:ident $meta:tt ; $($rest:tt)*) => {{
        let module = Some($crate::tq_expr!($meta));
        let var = $crate::tq_token!($dollar$var);
        let stmt = StmtImportToml::new($file.into(), var, module);

        let first = ::std::iter::once(Stmt::ImportToml(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (@stmt include $file:tt ; $($rest:tt)*) => {{
        let stmt = StmtInclude::new($file.into(), None);
        let first = ::std::iter::once(Stmt::Include(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (@stmt include $file:tt $meta:tt ; $($rest:tt)*) => {{
        let module = Some($crate::tq_expr!($meta));
        let stmt = StmtInclude::new($file.into(), module);

        let first = ::std::iter::once(Stmt::Include(stmt));
        let rest = $crate::tq_stmts!(@stmt $($rest)*);
        first.chain(rest)
    }};

    (module $meta:tt ; $($imports:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;

        let module = Some($crate::tq_expr!($meta));
        let stmts = $crate::tq_stmts!(@stmt $($imports)*).collect();
        Stmts::new(module, stmts)
    }};

    ($($imports:tt)*) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;

        let stmts = $crate::tq_stmts!(@stmt $($imports)*).collect();
        Stmts::new(None, stmts)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_table_key {
    ( ($($expr:tt)+) ) => {
        $crate::ast::TableKey::Expr($crate::tq_expr!($($expr)+))
    };

    ( $ident:ident ) => {
        $crate::ast::TableKey::Field($crate::tq_token!($ident))
    };

    ( $literal:expr ) => {
        $crate::ast::TableKey::Literal($crate::tq_token!($literal))
    };

    ( $dollar:tt $var:ident ) => {
        $crate::ast::TableKey::Variable($crate::tq_token!($dollar$var))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_table_value {
    ( ($($expr:tt)+) ) => {
        $crate::tq_expr!($($expr)+)
    };

    ( -$($expr:tt)+ ) => {
        Expr::Unary(UnaryOp::Neg, Box::new($crate::tq_table_value!($($expr)+)))
    };

    (@pipe ($($term:tt)+) | $($rest:tt)+) => {{
        let lhs = $crate::term!($($term)+);
        let rhs = $crate::tq_table_value!($($rest)+);
        Expr::Binary(BinaryOp::Pipe, Box::new(lhs), Box::new(rhs))
    }};

    (@pipe ($($term:tt)+)) => {
        $crate::term!($($term)+)
    };

    (@pipe ($($term:tt)+) $next:tt $($rest:tt)*) => {
        $crate::tq_table_value!(@pipe ($($term)+ $next) $($rest)*)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_table_value!(@pipe ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_construct {
    ( [ ] ) => {
        Expr::Array(None)
    };

    ( [ $($expr:tt)+ ] ) => {
        Expr::Array(Some(Box::new($crate::tq_expr!($($expr)+))))
    };

    ( { } ) => {
        Expr::Table(Vec::new())
    };

    ( { $($members:tt)+ } ) => {
        Expr::Table($crate::tq_construct!(@table $($members)+).collect())
    };

    (@member ($dollar:tt $var:ident = $($value:tt)+) , $($rest:tt)+) => {{
        let member = ($crate::tq_table_key!($dollar$var), $crate::tq_table_value!($($value)+));
        ::std::iter::once(member).chain($crate::tq_construct!(@table $($rest)+))
    }};

    (@member ($key:tt = $($value:tt)+) , $($rest:tt)+) => {{
        let member = ($crate::tq_table_key!($key), $crate::tq_table_value!($($value)+));
        ::std::iter::once(member).chain($crate::tq_construct!(@table $($rest)+))
    }};

    (@member ($dollar:tt $var:ident = $($value:tt)+)) => {
        ::std::iter::once((
            $crate::tq_table_key!($dollar$var),
            $crate::tq_table_value!($($value)+),
        ))
    };

    (@member ($key:tt = $($value:tt)+)) => {
        ::std::iter::once((
            $crate::tq_table_key!($key),
            $crate::tq_table_value!($($value)+),
        ))
    };

    (@member ($($members:tt)+) $next:tt $($rest:tt)*) => {
        $crate::tq_construct!(@member ($($members)+ $next) $($rest)*)
    };

    (@table $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_construct!(@member ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_pattern {
    ( ($($pat:tt)+) ) => {
        $crate::tq_pattern!($($pat)+)
    };

    ( [$($dollar:tt $var:ident),+] ) => {
        ExprPattern::Array(vec![
            $($crate::tq_pattern!($dollar$var)),+
        ])
    };

    ( { $($members:tt)+ } ) => {
        ExprPattern::Table($crate::tq_pattern!(@table $($members)+).collect())
    };

    ( $dollar:tt $var:ident ) => {
        ExprPattern::Variable($crate::tq_token!($dollar$var))
    };

    (@member ($dollar:tt $var:ident = $($value:tt)+) , $($rest:tt)+) => {{
        let member = ($crate::tq_table_key!($dollar$var), $crate::tq_pattern!($($value)+));
        ::std::iter::once(member).chain($crate::tq_pattern!(@table $($rest)+))
    }};

    (@member ($key:tt = $($value:tt)+) , $($rest:tt)+) => {{
        let member = ($crate::tq_table_key!($key), $crate::tq_pattern!($($value)+));
        ::std::iter::once(member).chain($crate::tq_pattern!(@table $($rest)+))
    }};

    (@member ($dollar:tt $var:ident = $($value:tt)+)) => {
        ::std::iter::once((
            $crate::tq_table_key!($dollar$var),
            $crate::tq_pattern!($($value)+),
        ))
    };

    (@member ($key:tt = $($value:tt)+)) => {
        ::std::iter::once((
            $crate::tq_table_key!($key),
            $crate::tq_pattern!($($value)+),
        ))
    };

    (@member ($($members:tt)+) $next:tt $($rest:tt)*) => {
        $crate::tq_pattern!(@member ($($members)+ $next) $($rest)*)
    };

    (@table $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_pattern!(@member ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_index {
    () => {
        ExprIndex::Iter
    };

    ( $upper:tt:$($lower:tt)+ ) => {
        ExprIndex::Slice(ExprSlice::Range($crate::tq_expr!($($upper)+), $crate::tq_expr!($($lower)+)))
    };

    ( $expr:tt: ) => {
        ExprIndex::Slice(ExprSlice::Lower($crate::tq_expr!($expr)))
    };

    ( :$($expr:tt)+ ) => {
        ExprIndex::Slice(ExprSlice::Upper($crate::tq_expr!($($expr)+)))
    };

    ( $($expr:tt)+ ) => {
        ExprIndex::Exact($crate::tq_expr!($($expr)+))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_filter {
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

    // Identity filter literal.
    (.) => {
        ExprFilter::Identity
    };

    // Recurse filter literal.
    (..) => {
        ExprFilter::Recurse
    };

    // Single field path.
    ( .$field:ident ) => {
        ExprFilter::Field($crate::tq_token!($field))
    };

    // Single slice path.
    ( .[$($expr:tt)*] ) => {
        ExprFilter::Index(Box::new($crate::tq_index!($($expr)*)))
    };

    // Nested path beginning with an identifier-style field access.
    ( .$field:ident $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.$field), |seq, next| {
                ExprFilter::Path(Box::new(seq), Box::new(next))
            })
    };

    // Nested path beginning with a slice-style field access.
    ( .[$($expr:tt)*] $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold($crate::tq_filter!(.[$($expr)*]), |seq, next| {
                ExprFilter::Path(Box::new(seq), Box::new(next))
            })
    };

    // Nested path indexing into a variable.
    ( $dollar:tt $var:ident $($rest:tt)+ ) => {
        $crate::tq_filter!(@path $($rest)+)
            .fold(ExprFilter::Variable($crate::tq_token!($dollar$var)), |seq, next| {
                ExprFilter::Path(Box::new(seq), Box::new(next))
            })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_fn_args {
    (@args ($($expr:tt)+) ; $($rest:tt)+) => {
        ::std::iter::once($crate::tq_expr!($($expr)+)).chain($crate::tq_fn_args!($($rest)+))
    };

    (@args ($($expr:tt)+)) => {
        ::std::iter::once($crate::tq_expr!($($expr)+))
    };

    (@args ($($prev:tt)*) $next:tt $($rest:tt)* ) => {
        $crate::tq_fn_args!(@args ($($prev)* $next) $($rest)*)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_fn_args!(@args ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_fn_params {
    (@params ($($arg:tt)+) ; $($rest:tt)+) => {
        ::std::iter::once($crate::tq_fn_params!($($arg)+)).flatten().chain($crate::tq_fn_params!($($rest)+))
    };

    (@params ($($path:ident)::+)) => {
        ::std::iter::once(FnParam::Function($crate::tq_token!(::$($path)::+)))
    };

    (@params ($dollar:tt $var:ident)) => {
        ::std::iter::once(FnParam::Variable($crate::tq_token!($dollar$var)))
    };

    (@params ($($prev:tt)*) $next:tt $($rest:tt)* ) => {
        $crate::tq_fn_params!(@params ($($prev)* $next) $($rest)*)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::tq_fn_params!(@params ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_token {
    ( false ) => {
        $crate::ast::tokens::Literal::Boolean(false)
    };

    ( true ) => {
        $crate::ast::tokens::Literal::Boolean(true)
    };

    ( nan ) => {
        $crate::ast::tokens::Literal::from(::std::f64::NAN)
    };

    ( inf ) => {
        $crate::ast::tokens::Literal::from(::std::f64::INFINITY)
    };

    ( $ident:ident ) => {
        $crate::ast::tokens::Ident::from(stringify!($ident))
    };

    ( $($path:ident)::+ ) => {
        $crate::ast::tokens::IdentPath::from(vec![$(stringify!($path)),*])
    };

    ( ::$($path:ident)::+ ) => {
        $crate::ast::tokens::IdentPath::from(vec![$(stringify!($path)),*])
    };

    ( $dollar:tt $var:ident ) => {
        $crate::ast::tokens::Variable::from(concat!("$", stringify!($var)))
    };

    ( $literal:tt ) => {
        $crate::ast::tokens::Literal::from($literal)
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
/// assert_eq!(expr, Expr::Filter(ExprFilter::Identity));
/// assert_eq!(s, ".");
/// ```
#[cfg(test)]
#[macro_export]
macro_rules! tq_expr_and_str {
    ($($expr:tt)+) => {
        (
            $crate::tq!($($expr)+),
            stringify!($($expr)+)
                .replace(" . ", ".")
                .replace(". ", ".")
                .replace(".as", ". as")
                .replace("- ", "-")
                .replace("! ", "!")
                .replace("/ /", "//")
                .replace(" [", "[")
                .replace(" ]", "]")
                .replace("$ ", "$")
                .replace(" :: ", "::")
                .replace(" ?", "?")
        )
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! tq_expr {
    ($($expr:tt)+) => {
        $crate::pipe!($($expr)+)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! pipe {
    (@rule ($($expr:tt)+) as [ $($pat:tt)+ ] | $($rest:tt)+) => {{
        let pat = $crate::tq_pattern!([ $($pat)+ ]);
        let bind = ExprBinding::new($crate::term!($($expr)+), pat);
        Expr::Binding(Box::new(bind), Box::new($crate::pipe!($($rest)+)))
    }};

    (@rule ($($expr:tt)+) as { $($pat:tt)+ } | $($rest:tt)+) => {{
        let pat = $crate::tq_pattern!({ $($pat)+ });
        let bind = ExprBinding::new($crate::term!($($expr)+), pat);
        Expr::Binding(Box::new(bind), Box::new($crate::pipe!($($rest)+)))
    }};

    (@rule ($($expr:tt)+) as $dollar:tt $var:ident | $($rest:tt)+) => {{
        let pat = $crate::tq_pattern!($dollar$var);
        let bind = ExprBinding::new($crate::term!($($expr)+), pat);
        Expr::Binding(Box::new(bind), Box::new($crate::pipe!($($rest)+)))
    }};

    (@rule (label $dollar:tt $var:ident) | $($rest:tt)+) => {{
        let label = Expr::Label(Label::from(concat!("$", stringify!($var))));
        let expr = $crate::pipe!($($rest)+);
        Expr::Binary(BinaryOp::Pipe, Box::new(label), Box::new(expr))
    }};

    (@rule ($($lhs:tt)+) | $($rhs:tt)+) => {{
        let lhs = $crate::chain!($($lhs)+);
        let rhs = $crate::pipe!($($rhs)+);
        Expr::Binary(BinaryOp::Pipe, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::pipe!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::chain!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::pipe!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! chain {
    (@rule ($($lhs:tt)+) , $($rhs:tt)+) => {{
        let lhs = $crate::fn_decl!($($lhs)+);
        let rhs = $crate::chain!($($rhs)+);
        Expr::Binary(BinaryOp::Comma, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::chain!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::fn_decl!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::chain!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! fn_decl {
    (@rule (def $($name:ident)::+) : $($rest:tt)+) => {{
        let name = $crate::tq_token!(::$($name)::+);
        let (body, expr) = $crate::fn_decl!($($rest)+);
        Expr::FnDecl(Box::new(ExprFnDecl::new(name, Vec::new(), body)), Box::new(expr))
    }};

    (@rule (def $($name:ident)::+) ($($args:tt)+) : $($rest:tt)+) => {{
        let name = $crate::tq_token!(::$($name)::+);
        let args = $crate::tq_fn_params!($($args)+).collect();
        let (body, expr) = $crate::fn_decl!($($rest)+);
        Expr::FnDecl(Box::new(ExprFnDecl::new(name, args, body)), Box::new(expr))
    }};

    (@rule ($($prev:tt)+) ; $($rest:tt)+) => {
        ($crate::tq_expr!($($prev)+), $crate::fn_decl!($($rest)+))
    };

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::fn_decl!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::assign!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::fn_decl!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! assign {
    // Note that the alt (`//`) operator is separated by a space in this macro because it also
    // happens to be the comment token in Rust. The `tq_expr_and_str!()` macro will replace these
    // occurrences with the correct `//` form in the output string.
    (@rule ($($lhs:tt)+) / /= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Alt, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) |= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Pipe, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) += $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Add, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) -= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Sub, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) *= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Sub, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) /= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Sub, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) %= $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::AssignOp(BinaryOp::Sub, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) = $($rhs:tt)+) => {{
        let lhs = $crate::logical!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::Assign(Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::assign!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::logical!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::assign!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! logical {
    (@rule ($($lhs:tt)+) and $($rhs:tt)+) => {{
        let lhs = $crate::sum!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::Binary(BinaryOp::And, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) or $($rhs:tt)+) => {{
        let lhs = $crate::sum!($($lhs)+);
        let rhs = $crate::logical!($($rhs)+);
        Expr::Binary(BinaryOp::Or, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::logical!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::sum!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::logical!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! sum {
    (@rule ($($lhs:tt)+) + $($rhs:tt)+) => {{
        let lhs = $crate::product!($($lhs)+);
        let rhs = $crate::sum!($($rhs)+);
        Expr::Binary(BinaryOp::Add, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) - $($rhs:tt)+) => {{
        let lhs = $crate::product!($($lhs)+);
        let rhs = $crate::sum!($($rhs)+);
        Expr::Binary(BinaryOp::Sub, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)*) => {
        $crate::sum!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::product!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::sum!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! product {
    // Note that the alt (`//`) operator is separated by a space in this macro because it also
    // happens to be the comment token in Rust. The `tq_expr_and_str!()` macro will replace these
    // occurrences with the correct `//` form in the output string.
    (@rule ($($lhs:tt)+) / / $($rhs:tt)+) => {{
        let lhs = $crate::try_postfix!($($lhs)+);
        let rhs = $crate::product!($($rhs)+);
        Expr::Binary(BinaryOp::Alt, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) * $($rhs:tt)+) => {{
        let lhs = $crate::try_postfix!($($lhs)+);
        let rhs = $crate::product!($($rhs)+);
        Expr::Binary(BinaryOp::Mul, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) / $($rhs:tt)+) => {{
        let lhs = $crate::try_postfix!($($lhs)+);
        let rhs = $crate::product!($($rhs)+);
        Expr::Binary(BinaryOp::Div, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($lhs:tt)+) % $($rhs:tt)+) => {{
        let lhs = $crate::try_postfix!($($lhs)+);
        let rhs = $crate::product!($($rhs)+);
        Expr::Binary(BinaryOp::Mod, Box::new(lhs), Box::new(rhs))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)* ) => {
        $crate::product!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::try_postfix!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::product!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! try_postfix {
    (@rule ($($expr:tt)+) $(?)+) => {{
        Expr::Try(Box::new(ExprTry::new($crate::unary!($($expr)+), None)))
    }};

    (@rule ($($prev:tt)*) $next:tt $($rest:tt)* ) => {
        $crate::try_postfix!(@rule ($($prev)* $next) $($rest)*)
    };

    (@rule ($($expr:tt)+)) => {
        $crate::unary!($($expr)+)
    };

    ( $first:tt $($rest:tt)* ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::try_postfix!(@rule ($first) $($rest)*)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! unary {
    (@rule - $($expr:tt)+) => {{
        Expr::Unary(UnaryOp::Neg, Box::new($crate::term!($($expr)+)))
    }};

    (@rule ! $($expr:tt)+) => {{
        Expr::Unary(UnaryOp::Not, Box::new($crate::term!($($expr)+)))
    }};

    (@rule $($expr:tt)+) => {
        $crate::term!($($expr)+)
    };

    ( $($expr:tt)+ ) => {{
        #[allow(unused_imports)]
        use $crate::ast::*;
        #[allow(unused_imports)]
        use $crate::ast::tokens::*;
        $crate::unary!(@rule $($expr)+)
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! term {
    (($($expr:tt)+)) => {
        Expr::Paren(Box::new($crate::tq_expr!($($expr)+)))
    };

    ({ $($members:tt)* }) => {
        $crate::tq_construct!({ $($members)* })
    };

    ([ $($expr:tt)* ]) => {
        $crate::tq_construct!([ $($expr)* ])
    };

    (..) => {
        Expr::Filter(Box::new($crate::tq_filter!(..)))
    };

    (.$($path:tt)*) => {
        Expr::Filter(Box::new($crate::tq_filter!(.$($path)*)))
    };

    ($dollar:tt $var:ident $($path:tt)+) => {
        Expr::Filter(Box::new($crate::tq_filter!($dollar$var$($path)*)))
    };

    ($dollar:tt $var:ident) => {
        Expr::Variable($crate::tq_token!($dollar$var))
    };

    (false) => {
        Expr::Literal($crate::tq_token!(false))
    };

    (true) => {
        Expr::Literal($crate::tq_token!(true))
    };

    (inf) => {
        Expr::Literal($crate::tq_token!(inf))
    };

    (nan) => {
        Expr::Literal($crate::tq_token!(nan))
    };

    ($($fn_call:ident)::+) => {{
        let path = $crate::tq_token!(::$($fn_call)::+);
        Expr::FnCall(ExprFnCall::new(path, Vec::new()))
    }};

    ($($fn_call:ident)::+ ($($args:tt)+)) => {{
        let path = $crate::tq_token!(::$($fn_call)::+);
        let args = $crate::tq_fn_args!($($args)+).collect();
        Expr::FnCall(ExprFnCall::new(path, args))
    }};

    ($literal:expr) => {
        Expr::Literal($crate::tq_token!($literal))
    };
}
