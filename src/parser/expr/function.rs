use std::iter;

use pom::parser::*;

use super::{expr, tokens};
use crate::ast::{ExprFnCall, ExprFnDecl};

pub fn function_decl<'a>() -> Parser<'a, u8, ExprFnDecl> {
    let name = (tokens::space() + tokens::keyword_def() + tokens::space()) * tokens::ident_path();
    let param = || tokens::space() * tokens::fn_param() - tokens::space();
    let params = optional_arg_sequence(param) - sym(b':');
    let body = call(expr) - sym(b';');
    (name + params + body).map(|((name, args), body)| ExprFnDecl::new(name, args, body))
}

pub fn function_call<'a>() -> Parser<'a, u8, ExprFnCall> {
    let name = tokens::ident_path();
    let args = optional_arg_sequence(expr);
    (name + args).map(|(name, args)| ExprFnCall::new(name, args))
}

/// Parses an optional sequence of function arguments of type `T`.
///
/// If the function being parsed takes no arguments, this parser will return an empty `Vec`.
fn optional_arg_sequence<'a, T, F>(token: F) -> Parser<'a, u8, Vec<T>>
where
    T: 'a,
    F: 'a + Clone + Fn() -> Parser<'a, u8, T>,
{
    let seq = sym(b'(') * call(token.clone()) + (sym(b';') * call(token)).repeat(0..) - sym(b')');
    let args = seq.map(|(first, rest)| iter::once(first).chain(rest).collect());
    args.opt().map(Option::unwrap_or_default)
}
