use std::path::PathBuf;

use pom::parser::*;

use super::expr::expr;
use super::tokens;
use crate::ast::{Stmt, StmtImportMod, StmtImportToml, StmtInclude, Stmts};

pub fn stmts<'a>() -> Parser<'a, u8, Stmts> {
    let module = tokens::keyword_module() * tokens::space() * expr() - sym(b';');
    let stmts = tokens::space() * stmt().repeat(0..);
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
