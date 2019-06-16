use std::path::PathBuf;

use pom::parser::*;

use super::expr::expr;
use super::tokens;
use crate::ast::{Stmt, StmtImportMod, StmtImportToml, StmtInclude, Stmts};

pub fn stmts<'a>() -> Parser<'a, u8, Stmts> {
    let module = tokens::keyword_module() * tokens::space() * expr() - sym(b';');
    let stmts = (tokens::space() * stmt()).repeat(0..);
    (module.opt() + stmts).map(|(module, stmts)| Stmts::new(module, stmts))
}

fn stmt<'a>() -> Parser<'a, u8, Stmt> {
    let import_toml = import_toml().map(Stmt::ImportToml);
    let import_mod = import_mod().map(Stmt::ImportMod);
    let include = include().map(Stmt::Include);
    let stmt = import_mod | import_toml | include;
    stmt - tokens::space() - sym(b';')
}

fn import_toml<'a>() -> Parser<'a, u8, StmtImportToml> {
    let keyword = tokens::keyword_import() + tokens::space();
    let file = path_buf() - tokens::space() - tokens::keyword_as() - tokens::space();
    let var = tokens::variable();
    let metadata = expr().opt();
    let stmt = keyword * file + var + metadata;
    stmt.map(|((file, var), meta)| StmtImportToml::new(file, var, meta))
}

fn import_mod<'a>() -> Parser<'a, u8, StmtImportMod> {
    let keyword = tokens::keyword_import() + tokens::space();
    let file = path_buf() - tokens::space() - tokens::keyword_as() - tokens::space();
    let path = tokens::ident_path();
    let metadata = expr().opt();
    let stmt = keyword * file + path + metadata;
    stmt.map(|((file, path), meta)| StmtImportMod::new(file, path, meta))
}

fn include<'a>() -> Parser<'a, u8, StmtInclude> {
    let keyword = tokens::keyword_include() + tokens::space();
    let file = path_buf();
    let metadata = expr().opt();
    let stmt = keyword * file + metadata;
    stmt.map(|(file, meta)| StmtInclude::new(file, meta))
}

fn path_buf<'a>() -> Parser<'a, u8, PathBuf> {
    tokens::string().map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tq_stmts_and_str;

    #[test]
    fn empty() {
        let (expected, text) = tq_stmts_and_str!();
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn module() {
        let (expected, text) = tq_stmts_and_str!(module "metadata";);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn import_mod() {
        let (expected, text) = tq_stmts_and_str!(import "path/to/mod" as FOO::BAR;);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(import "path/to/mod" as FOO "meta";);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn import_toml() {
        let (expected, text) = tq_stmts_and_str!(import "path/to/toml" as $foo;);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(import "path/to/toml" as $foo "meta";);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn include() {
        let (expected, text) = tq_stmts_and_str!(include "path/to/filter";);
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);

        let (expected, text) = tq_stmts_and_str!(include "path/to/filter" "meta";);
        let actual = stmts().parse(text.as_bytes()).unwrap();
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
        let actual = stmts().parse(text.as_bytes()).unwrap();
        assert_eq!(expected, actual);
    }
}
