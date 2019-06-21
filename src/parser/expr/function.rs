use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::separated_nonempty_list;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use super::{expr, tokens};
use crate::ast::tokens::FnParam;
use crate::ast::{Expr, ExprFnCall, ExprFnDecl};

pub fn function_decl(input: &str) -> IResult<&str, ExprFnDecl> {
    let key_def = pair(tokens::keyword_def, tokens::space);
    let name = delimited(key_def, tokens::ident_path, tokens::space);
    let params = terminated(opt_param_sequence, pair(char(':'), tokens::space));
    let body = terminated(expr, char(';'));

    map(tuple((name, params, body)), |(name, params, body)| {
        ExprFnDecl::new(name, params, body)
    })(input)
}

pub fn function_call(input: &str) -> IResult<&str, ExprFnCall> {
    let args = preceded(tokens::space, opt_arg_sequence);
    let call = pair(tokens::ident_path, args);
    map(call, |(name, args)| ExprFnCall::new(name, args))(input)
}

fn opt_param_sequence(input: &str) -> IResult<&str, Vec<FnParam>> {
    let param = delimited(tokens::space, tokens::fn_param, tokens::space);
    let params = separated_nonempty_list(char(';'), param);
    let open = pair(tokens::space, char('('));
    let seq = delimited(open, params, pair(char(')'), tokens::space));
    map(opt(seq), Option::unwrap_or_default)(input)
}

fn opt_arg_sequence(input: &str) -> IResult<&str, Vec<Expr>> {
    let args = separated_nonempty_list(pair(char(';'), tokens::space), expr);
    let open = tuple((tokens::space, char('('), tokens::space));
    let seq = delimited(open, args, pair(char(')'), tokens::space));
    map(opt(seq), Option::unwrap_or_default)(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    macro_rules! tq_fn_call_and_str {
        ($($expr:tt)+) => {{
            let (expr, string) = $crate::tq_expr_and_str!($($expr)+);
            match expr {
                $crate::ast::Expr::FnCall(call) => (call, string.trim().to_string()),
                e => panic!(format!("tq_expr_and_str!() did not produce an `ExprFnCall`: {:?}", e)),
            }
        }};
    }

    macro_rules! tq_fn_decl_and_str {
        ($($expr:tt)+) => {{
            let (expr, string) = $crate::tq_expr_and_str!($($expr)+ .);
            match expr {
                $crate::ast::Expr::FnDecl(decl, _) => (*decl, string.trim().trim_end_matches(" .").to_string()),
                e => panic!(format!("tq_expr_and_str!() did not produce an `ExprFnDecl`: {:?}", e)),
            }
        }};
    }

    #[test]
    fn declaration_simple() {
        let (expected, string) = tq_fn_decl_and_str!(def foo: .;);
        let (_, actual) = all_consuming(function_decl)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn declaration_path() {
        let (expected, string) = tq_fn_decl_and_str!(def foo::bar: .;);
        let (_, actual) = all_consuming(function_decl)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn declaration_with_arguments() {
        let (expected, string) = tq_fn_decl_and_str!(def foo(first; $second): .;);
        let (_, actual) = all_consuming(function_decl)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn call_simple() {
        let (expected, string) = tq_fn_call_and_str!(foo);
        let (_, actual) = all_consuming(function_call)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn call_path() {
        let (expected, string) = tq_fn_call_and_str!(foo::bar);
        let (_, actual) = all_consuming(function_call)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn call_with_arguments() {
        let (expected, string) = tq_fn_call_and_str!(foo(1; 2));
        let (_, actual) = all_consuming(function_call)(&string).unwrap();
        assert_eq!(expected, actual);
    }
}
