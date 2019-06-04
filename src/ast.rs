use std::collections::BTreeMap;
use std::path::PathBuf;

use self::tokens::{FnParam, Ident, IdentPath, Label, Literal, Variable};

pub mod tokens;

mod macros;

#[derive(Clone, Debug, PartialEq)]
pub enum Stmt {
    /// `import "my_tq_module" as FOO;`
    ImportMod(StmtImportMod),
    /// `import "./path/to/toml" as $FOO;`
    ImportToml(StmtImportToml),
    /// `include "./path/to/tq/module";`
    Include(StmtInclude),
    /// `module "foo";`
    Module(StmtModule),
    /// An expression (must occur at the end of the filter).
    Expr(Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtImportMod {
    file: PathBuf,
    path: IdentPath,
    metadata: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtImportToml {
    file: PathBuf,
    variable: Variable,
    metadata: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtInclude {
    file: PathBuf,
    metadata: Option<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StmtModule {
    metadata: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    /// `foo`
    Field(Ident),
    /// `$foo`
    Variable(Variable),
    /// `12`, `-4.0`, `false`, `"foo"`, `'bar'`
    Literal(Literal),
    /// `[1, 2, 3, 4]`, `[map(. + 1)]`
    Array(Option<Box<Expr>>),
    /// `{ foo = "bar", baz = 5 }`
    Table(Vec<(Expr, Expr)>),

    /// `-12`
    /// `+15.0`
    Unary(UnaryOp, Box<Expr>),
    /// `.foo + 5`
    /// `.dependencies[] | .version`
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    /// `.package.name = "foo"`
    Assign(Box<Expr>, Box<Expr>),
    /// `.package.authors[] += "suffix"`
    AssignOp(BinaryOp, Box<Expr>, Box<Expr>),

    /// `.package`
    /// `.dependencies.log`
    /// `.dependencies["log"]`
    /// `.dependencies[]`
    Filter(Box<Filter>),
    /// `my_func(. + 1)[0]`
    /// `"hello world"[4:]`
    /// `[1, 2, 3][1:2]`
    /// `{ foo = 1, bar = 2 }[]`
    Index(Box<Expr>, Box<ExprIndex>),

    /// `.package as $pkg`
    /// `.package as { name = $name, authors = $authors }`
    Binding(Box<ExprBinding>),

    /// `def increment: . + 1;`
    /// `def addvalue(f): f as $f | map(. + $f);`
    /// `def addvalue($f): map(. + $f);`
    FnDecl(ExprFnDecl),
    /// `keys`
    /// `map(. + 1)`
    FnCall(ExprFnCall),

    /// `label $foo`
    Label(Label),
    /// `label $out | ... break $out ...`
    Break(Label),

    /// `if A then B elif C else D end`
    IfElse(Box<ExprIfElse>),
    /// `reduce EXPR as $var (ACC; EVAL)`
    Reduce(Box<ExprReduce>),
    /// `foreach EXPR as $var (INIT; UPDATE; EXTRACT)`
    Foreach(Box<ExprForeach>),
    /// `.package.name?`
    /// `try .package.name`
    /// `try .package.name catch 'nonexistent'`
    Try(Box<ExprTry>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Filter {
    /// `.`
    Identity,
    /// `..`
    Recurse,

    /// `foo`
    Field(Ident),
    /// `.foo[1]`
    /// `.foo[2:5]`
    /// `.foo["hello"]`
    Index(Box<ExprIndex>),
    /// `.foo.bar["baz"][]`
    Path(Box<Filter>, Box<Filter>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    /// The unary `-` operator.
    Neg,
    /// The unary `!` operator.
    Not,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOp {
    /// The binary `+` operator.
    Add,
    /// The binary `-` operator.
    Sub,
    /// The binary `*` operator.
    Mul,
    /// The binary `/` operator.
    Div,
    /// The binary `%` operator.
    Mod,
    /// The binary `==` operator.
    Eq,
    /// The binary `!=` operator.
    NotEq,
    /// The binary `<` operator.
    LessThan,
    /// The binary `<=` operator.
    LessThanEq,
    /// The binary `>` operator.
    GreaterThan,
    /// The binary `>=` operator.
    GreaterThanEq,
    /// The binary `and` operator.
    And,
    /// The binary `or` operator.
    Or,
    /// The binary `//` operator.
    Alt,
    /// The binary `,` operator.
    Comma,
    /// The binary `|` operator.
    Pipe,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBinding {
    expr: Expr,
    pattern: ExprPattern,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprIndex {
    Iter,
    Exact(Expr),
    Slice(ExprSlice),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprSlice {
    Lower(Expr),
    Upper(Expr),
    Range(Expr, Expr),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprPattern {
    Variable(Variable),
    Array(Vec<ExprPattern>),
    Table(BTreeMap<Expr, ExprPattern>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnCall {
    path: IdentPath,
    args: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnDecl {
    name: IdentPath,
    params: Vec<FnParam>,
    body: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIfElse {
    clauses: Vec<(Expr, Expr)>,
    fallback: Expr,
}

impl ExprIfElse {
    pub fn new(clauses: Vec<(Expr, Expr)>, fallback: Expr) -> Self {
        ExprIfElse { clauses, fallback }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReduce {
    binding: ExprBinding,
    acc: Expr,
    eval: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprForeach {
    binding: ExprBinding,
    init: Expr,
    update: Expr,
    extract: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprTry {
    expr: Expr,
    fallback: Option<Expr>,
}

impl ExprTry {
    pub fn new(expr: Expr, fallback: Option<Expr>) -> Self {
        ExprTry { expr, fallback }
    }
}
