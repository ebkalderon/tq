use std::collections::BTreeMap;
use std::path::PathBuf;

use self::tokens::{FnParam, Ident, IdentPath, Literal, Variable};

pub mod tokens;

#[derive(Clone, Debug, PartialEq)]
pub struct Filter(Vec<Stmt>);

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    Import(StmtImport),
    Include(StmtInclude),
    Module(StmtModule),
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtImport {
    file: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtInclude {
    file: PathBuf,
    path: IdentPath,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtModule {
    name: Ident,
    metadata: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // Core primitives.
    Identity,
    Ident(Ident),
    Literal(Literal),
    Variable(Variable),

    // Compound primitives.
    Array(Vec<Expr>),
    Table(BTreeMap<Ident, Expr>),

    // Piping filters together.
    Pipe(Box<Expr>, Box<Expr>),

    // Function declaration and calls.
    FnDecl(ExprFnDecl),
    FnCall(ExprFnCall),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnDecl {
    name: IdentPath,
    params: Vec<FnParam>,
    body: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnCall {
    path: IdentPath,
    args: Vec<Expr>,
}
