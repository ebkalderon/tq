use std::path::PathBuf;

use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, pair, preceded, terminated, tuple};
use nom::IResult;

use super::expr::expr;
use super::tokens;
use crate::ast::{Stmt, StmtImportMod, StmtImportToml, StmtInclude, Stmts};

pub fn stmts(input: &str) -> IResult<&str, Stmts> {
    let key_module = pair(tokens::keyword_module, tokens::space);
    let module = opt(delimited(key_module, expr, char(';')));
    let stmts = many0(preceded(tokens::space, stmt));
    map(pair(module, stmts), |(module, stmts)| {
        Stmts::new(module, stmts)
    })(input)
}

fn stmt(input: &str) -> IResult<&str, Stmt> {
    let import_toml = map(import_toml, Stmt::ImportToml);
    let import_mod = map(import_mod, Stmt::ImportMod);
    let include = map(include, Stmt::Include);
    let stmt = alt((import_mod, import_toml, include));
    terminated(stmt, pair(tokens::space, char(';')))(input)
}

fn import_toml(input: &str) -> IResult<&str, StmtImportToml> {
    let keyword = pair(tokens::keyword_import, tokens::space);
    let file = terminated(path_buf, pair(tokens::space, tokens::keyword_as));
    let var = delimited(tokens::space, tokens::variable, tokens::space);
    let metadata = opt(expr);
    let stmt = preceded(keyword, tuple((file, var, metadata)));
    map(stmt, |(file, var, meta)| {
        StmtImportToml::new(file, var, meta)
    })(input)
}

fn import_mod(input: &str) -> IResult<&str, StmtImportMod> {
    let keyword = pair(tokens::keyword_import, tokens::space);
    let file = terminated(path_buf, pair(tokens::space, tokens::keyword_as));
    let path = delimited(tokens::space, tokens::ident_path, tokens::space);
    let metadata = opt(expr);
    let stmt = preceded(keyword, tuple((file, path, metadata)));
    map(stmt, |(file, path, meta)| {
        StmtImportMod::new(file, path, meta)
    })(input)
}

fn include(input: &str) -> IResult<&str, StmtInclude> {
    let keyword = pair(tokens::keyword_include, tokens::space);
    let file = terminated(path_buf, tokens::space);
    let metadata = opt(expr);
    let stmt = preceded(keyword, pair(file, metadata));
    map(stmt, |(file, meta)| StmtInclude::new(file, meta))(input)
}

fn path_buf(input: &str) -> IResult<&str, PathBuf> {
    map(tokens::string, PathBuf::from)(input)
}

#[cfg(test)]
mod tests {
    use nom::combinator::all_consuming;

    use super::*;

    macro_rules! tq_stmts_and_str {
        ($($stmts:tt)*) => {
            (
                $crate::tq_stmts!($($stmts)*),
                stringify!($($stmts)*)
                    .replace('\n', " ")
                    .replace(" :: ", "::")
                    .replace("$ ", "$"),
            )
        };
    }

    #[test]
    fn empty() {
        let (expected, text) = tq_stmts_and_str!();
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn module() {
        let (expected, text) = tq_stmts_and_str!(module "metadata";);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn import_mod() {
        let (expected, text) = tq_stmts_and_str!(import "path/to/mod" as FOO::BAR;);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(import "path/to/mod" as FOO "meta";);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn import_toml() {
        let (expected, text) = tq_stmts_and_str!(import "path/to/toml" as $foo;);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(import "path/to/toml" as $foo "meta";);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn include() {
        let (expected, text) = tq_stmts_and_str!(include "path/to/filter";);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(include "path/to/filter" "meta";);
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn complex_sequence() {
        let (expected, text) = tq_stmts_and_str! {
            module "meta";
            import "foo" as IDENT;
            import "bar" as $var;
            include "hello" "meta";
        };
        let (_, actual) = all_consuming(stmts)(&text).unwrap();
        assert_eq!(expected, actual);
    }
}
