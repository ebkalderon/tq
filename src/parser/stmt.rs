use std::path::PathBuf;

use pom::parser::*;

use super::expr::expr;
use super::tokens;
use crate::ast::{Stmt, StmtImportMod, StmtImportToml, StmtInclude};

pub fn stmt<'a>() -> Parser<'a, u8, Stmt> {
    let import_toml = import_toml().map(Stmt::ImportToml);
    let import_mod = import_mod().map(Stmt::ImportMod);
    let include = include().map(Stmt::Include);
    let stmt = import_mod | import_toml | include;
    tokens::space() * stmt - tokens::space() - sym(b';')
}

fn import_toml<'a>() -> Parser<'a, u8, StmtImportToml> {
    let keyword = seq(b"import") + tokens::space();
    let file = path_buf() - tokens::space() - seq(b"as") - tokens::space();
    let var = tokens::variable() - tokens::space();
    let metadata = expr().opt();
    let stmt = keyword * (file + var + metadata);
    stmt.map(|((file, var), meta)| StmtImportToml::new(file, var, meta))
}

fn import_mod<'a>() -> Parser<'a, u8, StmtImportMod> {
    let keyword = seq(b"import") + tokens::space();
    let file = path_buf() - tokens::space() - seq(b"as") - tokens::space();
    let path = tokens::ident_path() - tokens::space();
    let metadata = expr().opt();
    let stmt = keyword * (file + path + metadata);
    stmt.map(|((file, path), meta)| StmtImportMod::new(file, path, meta))
}

fn include<'a>() -> Parser<'a, u8, StmtInclude> {
    let keyword = seq(b"include") + tokens::space();
    let file = path_buf() - tokens::space();
    let metadata = expr().opt();
    let stmt = keyword * (file + metadata);
    stmt.map(|(file, meta)| StmtInclude::new(file, meta))
}

fn path_buf<'a>() -> Parser<'a, u8, PathBuf> {
    tokens::string().map(PathBuf::from)
}
