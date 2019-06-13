use pom::parser::*;

use super::construct;
use super::{tokens, unary};
use crate::ast::{ExprBinding, ExprPattern};

pub fn pattern<'a>() -> Parser<'a, u8, ExprPattern> {
    let variable = tokens::variable().map(ExprPattern::Variable);
    let array = array();
    let table = table();
    tokens::space() * (variable | array | table) - tokens::space()
}

fn array<'a>() -> Parser<'a, u8, ExprPattern> {
    let patterns = list(call(pattern), sym(b','));
    (sym(b'[') * patterns - sym(b']')).map(ExprPattern::Array)
}

fn table<'a>() -> Parser<'a, u8, ExprPattern> {
    let assign = construct::table_key() + (sym(b'=') * call(pattern));
    (sym(b'{') * list(assign, sym(b',')) - sym(b'}')).map(ExprPattern::Table)
}

pub fn binding<'a>() -> Parser<'a, u8, ExprBinding> {
    let bind = call(unary) + (seq(b"as") * call(pattern));
    bind.map(|(expr, pat)| ExprBinding::new(expr, pat))
}

#[cfg(test)]
mod tests {
    use pom::Error as ParseError;

    use super::*;
    use crate::ast::Expr;
    use crate::{tq_expr_and_str, tq_pattern};

    fn parse_binding(s: &str) -> Result<Expr, ParseError> {
        binding()
            .parse(s.as_bytes())
            .map(Box::new)
            .map(Expr::Binding)
    }

    #[test]
    fn pattern_array() {
        let expected = tq_pattern!([$foo]);
        let actual = pattern().parse(b"[$foo]").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_table() {
        let expected = tq_pattern!({ foo = $bar });
        let actual = pattern().parse(b"{ foo = $bar }").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn pattern_variable() {
        let expected = tq_pattern!($foo);
        let actual = pattern().parse(b"$foo").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_array() {
        let (expected, expr) = tq_expr_and_str!(. as [$foo]);
        let actual = parse_binding(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_table() {
        let (expected, expr) = tq_expr_and_str!(. as { foo = $bar });
        let actual = parse_binding(&expr).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn binding_variable() {
        let (expected, expr) = tq_expr_and_str!(. as $foo);
        let actual = parse_binding(&expr).unwrap();
        assert_eq!(expected, actual);
    }
}
