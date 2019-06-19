use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_nonempty_list;
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::IResult;

use super::construct;
use super::{index, tokens};
use crate::ast::{ExprBinding, ExprPattern};

pub fn pattern(input: &str) -> IResult<&str, ExprPattern> {
    let variable = map(tokens::variable, ExprPattern::Variable);
    alt((variable, array, table))(input)
}

fn array(input: &str) -> IResult<&str, ExprPattern> {
    let pattern = terminated(pattern, tokens::space);
    let patterns = separated_nonempty_list(pair(char(','), tokens::space), pattern);
    let blah = delimited(pair(char('['), tokens::space), patterns, char(']'));
    map(blah, ExprPattern::Array)(input)
}

fn table(input: &str) -> IResult<&str, ExprPattern> {
    let key = terminated(construct::table_key, tokens::space);
    let value = terminated(pattern, tokens::space);
    let member = pair(key, preceded(pair(char('='), tokens::space), value));

    let members = separated_nonempty_list(pair(char(','), tokens::space), member);
    let table = delimited(pair(char('{'), tokens::space), members, char('}'));

    map(table, ExprPattern::Table)(input)
}

pub fn binding(input: &str) -> IResult<&str, ExprBinding> {
    let key_as = pair(tokens::keyword_as, tokens::space);
    let bind = pair(terminated(index, tokens::space), preceded(key_as, pattern));
    map(bind, |(expr, pat)| ExprBinding::new(expr, pat))(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    macro_rules! tq_pattern_with_str {
        ($($pat:tt)*) => {
            (
                $crate::tq_pattern!($($pat)+),
                stringify!($($pat)+).replace("$ ", "$"),
            )
        };
    }

    macro_rules! tq_binding_and_str {
        ($($expr:tt)*) => {{
            let (expr, string) = $crate::tq_expr_and_str!($($expr)+ | .);
            match expr {
                $crate::ast::Expr::Binary(_, lhs, _) => match *lhs {
                    $crate::ast::Expr::Binding(bind) => (*bind, string.replace(" | .", "")),
                    e => panic!(format!("tq_expr_and_str!() did not produce an `ExprBinding`: {:?}", e)),
                }
                _ => panic!("tq_expr_and_str!() did not produce an `Expr::Binary`"),
            }
        }};
    }

    #[test]
    fn pattern_array() {
        let (expected, string) = tq_pattern_with_str!([$foo]);
        let (_, actual) = all_consuming(pattern)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_table() {
        let (expected, string) = tq_pattern_with_str!({ foo = $bar });
        let (_, actual) = all_consuming(pattern)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_variable() {
        let (expected, string) = tq_pattern_with_str!($foo);
        let (_, actual) = all_consuming(pattern)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_array() {
        let (expected, string) = tq_binding_and_str!(. as [$foo]);
        let (_, actual) = all_consuming(binding)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_table() {
        let (expected, string) = tq_binding_and_str!(. as { foo = $bar });
        let (_, actual) = all_consuming(binding)(&string).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_variable() {
        let (expected, string) = tq_binding_and_str!(. as $foo);
        let (_, actual) = all_consuming(binding)(&string).unwrap();
        assert_eq!(expected, actual);
    }
}
