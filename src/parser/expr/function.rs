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
    let name = preceded(key_def, tokens::ident_path);
    let params = terminated(opt_param_sequence, pair(char(':'), tokens::space));
    let body = terminated(expr, char(';'));

    map(tuple((name, params, body)), |(name, params, body)| {
        ExprFnDecl::new(name, params, body)
    })(input)
}

pub fn function_call(input: &str) -> IResult<&str, ExprFnCall> {
    let call = pair(tokens::ident_path, opt_arg_sequence);
    map(call, |(name, args)| ExprFnCall::new(name, args))(input)
}

fn opt_param_sequence(input: &str) -> IResult<&str, Vec<FnParam>> {
    let param = terminated(tokens::fn_param, tokens::space);
    let params = separated_nonempty_list(pair(char(';'), tokens::space), param);
    let seq = delimited(pair(char('('), tokens::space), params, char(')'));
    map(opt(seq), Option::unwrap_or_default)(input)
}

fn opt_arg_sequence(input: &str) -> IResult<&str, Vec<Expr>> {
    let args = separated_nonempty_list(pair(char(';'), tokens::space), expr);
    let seq = delimited(pair(char('('), tokens::space), args, char(')'));
    map(opt(seq), Option::unwrap_or_default)(input)
}
