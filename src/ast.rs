use std::fmt::{Display, Formatter, Result as FmtResult};
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
    /// `empty`
    Empty,

    /// `12`, `+4.0`, `false`, `"foo"`, `'bar'`
    Literal(Literal),
    /// `$foo`
    Variable(Variable),
    /// `[1, 2, 3, 4]`, `[map(. + 1)]`
    Array(Option<Box<Expr>>),
    /// `{ foo = "bar", baz = 5 }`
    Table(Vec<(TableKey, Expr)>),

    /// `-12`
    /// `!15.0`
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
    FnDecl(Box<ExprFnDecl>, Box<Expr>),
    /// `keys`
    /// `map(. + 1)`
    FnCall(ExprFnCall),

    /// `label $foo`
    Label(Label),
    /// `label $out | ... break $out ...`
    Break(Label),

    /// `if A then B elsif C else D end`
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

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Expr::Empty => fmt.write_str("empty"),
            Expr::Literal(ref lit) => write!(fmt, "{}", lit),
            Expr::Variable(ref var) => write!(fmt, "{}", var),
            Expr::Array(ref inner) => {
                let expr = inner.as_ref().map(|e| e.to_string()).unwrap_or_default();
                write!(fmt, "[{}]", expr)
            }
            Expr::Table(ref table) => {
                let table: Vec<_> = table
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect();
                write!(fmt, "{{{}}}", table.join(", "))
            }

            Expr::Unary(ref op, ref expr) => match **expr {
                Expr::Assign(_, _) | Expr::AssignOp(_, _, _) | Expr::Binary(_, _, _) => {
                    write!(fmt, "{}({})", op, expr)
                }
                _ => write!(fmt, "{}{}", op, expr),
            },
            Expr::Binary(ref op, ref lhs, ref rhs) => match op {
                BinaryOp::And | BinaryOp::Or | BinaryOp::Comma => {
                    write!(fmt, "{}{} {}", lhs, op, rhs)
                }
                op => write!(fmt, "{} {} {}", lhs, op, rhs),
            },
            Expr::Assign(ref lhs, ref rhs) => write!(fmt, "{} = {}", lhs, rhs),
            Expr::AssignOp(ref op, ref lhs, ref rhs) => write!(fmt, "{} {}= {}", lhs, op, rhs),

            Expr::Filter(ref filter) => write!(fmt, "{}", filter),
            Expr::Index(ref expr, ref index) => match **expr {
                Expr::Assign(_, _) | Expr::AssignOp(_, _, _) | Expr::Binary(_, _, _) => {
                    write!(fmt, "({}){}", expr, index)
                }
                _ => write!(fmt, "{}{}", expr, index),
            },
            Expr::Binding(ref binding) => write!(fmt, "{}", binding),

            Expr::FnDecl(ref decl, ref expr) => write!(fmt, "{} {}", decl, expr),
            Expr::FnCall(ref call) => write!(fmt, "{}", call),

            Expr::Label(ref label) => write!(fmt, "label {}", label),
            Expr::Break(ref label) => write!(fmt, "break {}", label),

            Expr::IfElse(ref expr) => write!(fmt, "{}", expr),
            Expr::Reduce(ref expr) => write!(fmt, "{}", expr),
            Expr::Foreach(ref expr) => write!(fmt, "{}", expr),
            Expr::Try(ref expr) => write!(fmt, "{}", expr),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TableKey {
    Field(Ident),
    Variable(Variable),
    Literal(Literal),
    Expr(Expr),
}

impl Display for TableKey {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            TableKey::Field(ref ident) => write!(fmt, "{}", ident),
            TableKey::Variable(ref var) => write!(fmt, "{}", var),
            TableKey::Literal(ref lit) => write!(fmt, "{}", lit),
            TableKey::Expr(ref expr) => write!(fmt, "({})", expr),
        }
    }
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

impl Display for Filter {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Filter::Identity => fmt.write_str("."),
            Filter::Recurse => fmt.write_str(".."),
            Filter::Field(ref ident) => write!(fmt, ".{}", ident),
            Filter::Index(ref index) => write!(fmt, ".{}", index),
            Filter::Path(ref lhs, ref rhs) => match **rhs {
                Filter::Index(ref expr) => write!(fmt, "{}{}", lhs, expr),
                _ => write!(fmt, "{}{}", lhs, rhs),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOp {
    /// The unary `-` operator.
    Neg,
    /// The unary `!` operator.
    Not,
}

impl Display for UnaryOp {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            UnaryOp::Neg => fmt.write_str("-"),
            UnaryOp::Not => fmt.write_str("!"),
        }
    }
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

impl Display for BinaryOp {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            BinaryOp::Add => fmt.write_str("+"),
            BinaryOp::Sub => fmt.write_str("-"),
            BinaryOp::Mul => fmt.write_str("*"),
            BinaryOp::Div => fmt.write_str("/"),
            BinaryOp::Mod => fmt.write_str("%"),
            BinaryOp::Eq => fmt.write_str("=="),
            BinaryOp::NotEq => fmt.write_str("!="),
            BinaryOp::LessThan => fmt.write_str("<"),
            BinaryOp::LessThanEq => fmt.write_str("<="),
            BinaryOp::GreaterThan => fmt.write_str(">"),
            BinaryOp::GreaterThanEq => fmt.write_str(">="),
            BinaryOp::And => fmt.write_str("and"),
            BinaryOp::Or => fmt.write_str("or"),
            BinaryOp::Alt => fmt.write_str("//"),
            BinaryOp::Comma => fmt.write_str(","),
            BinaryOp::Pipe => fmt.write_str("|"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBinding {
    expr: Expr,
    pattern: ExprPattern,
}

impl ExprBinding {
    pub fn new(expr: Expr, pattern: ExprPattern) -> Self {
        ExprBinding { expr, pattern }
    }
}

impl Display for ExprBinding {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self.expr {
            Expr::Assign(_, _) | Expr::AssignOp(_, _, _) | Expr::Binary(_, _, _) => {
                write!(fmt, "({}) as {}", self.expr, self.pattern)
            }
            _ => write!(fmt, "{} as {}", self.expr, self.pattern),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprIndex {
    Iter,
    Exact(Expr),
    Slice(ExprSlice),
}

impl Display for ExprIndex {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ExprIndex::Iter => write!(fmt, "[]"),
            ExprIndex::Exact(ref expr) => write!(fmt, "[{}]", expr),
            ExprIndex::Slice(ref slice) => write!(fmt, "[{}]", slice),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprSlice {
    Lower(Expr),
    Upper(Expr),
    Range(Expr, Expr),
}

impl Display for ExprSlice {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ExprSlice::Lower(ref bound) => write!(fmt, ":{}", bound),
            ExprSlice::Upper(ref bound) => write!(fmt, "{}:", bound),
            ExprSlice::Range(ref upper, ref lower) => write!(fmt, "{}:{}", upper, lower),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ExprPattern {
    Variable(Variable),
    Array(Vec<ExprPattern>),
    Table(Vec<(TableKey, ExprPattern)>),
}

impl Display for ExprPattern {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            ExprPattern::Variable(ref var) => write!(fmt, "{}", var),
            ExprPattern::Array(ref pats) => {
                let pats: Vec<_> = pats.iter().map(|p| p.to_string()).collect();
                write!(fmt, "[{}]", pats.join(", "))
            }
            ExprPattern::Table(ref table) => {
                let table: Vec<_> = table
                    .iter()
                    .map(|(k, v)| format!("{} = {}", k, v))
                    .collect();
                write!(fmt, "{{{}}}", table.join(", "))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnCall {
    path: IdentPath,
    args: Vec<Expr>,
}

impl ExprFnCall {
    pub fn new(path: IdentPath, args: Vec<Expr>) -> Self {
        ExprFnCall { path, args }
    }
}

impl Display for ExprFnCall {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        if self.args.is_empty() {
            write!(fmt, "{}", self.path)
        } else {
            let args: Vec<_> = self.args.iter().map(|arg| arg.to_string()).collect();
            write!(fmt, "{}({})", self.path, args.join("; "))
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprFnDecl {
    name: IdentPath,
    params: Vec<FnParam>,
    body: Expr,
}

impl ExprFnDecl {
    pub fn new(name: IdentPath, params: Vec<FnParam>, body: Expr) -> Self {
        ExprFnDecl { name, params, body }
    }
}

impl Display for ExprFnDecl {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let params: Vec<_> = self.params.iter().map(|param| param.to_string()).collect();
        let params = params.join("; ");

        if params.is_empty() {
            write!(fmt, "def {}: {};", self.name, self.body)
        } else {
            write!(fmt, "def {}({}): {};", self.name, params, self.body)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprIfElse {
    main_clause: (Expr, Expr),
    alt_clauses: Vec<(Expr, Expr)>,
    fallback: Expr,
}

impl ExprIfElse {
    pub fn new(main_clause: (Expr, Expr), alt_clauses: Vec<(Expr, Expr)>, fallback: Expr) -> Self {
        ExprIfElse {
            main_clause,
            alt_clauses,
            fallback,
        }
    }
}

impl Display for ExprIfElse {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let (main_cond, main_body) = &self.main_clause;
        let alts: String = self
            .alt_clauses
            .iter()
            .map(|(cond, body)| format!("elsif {} then {} ", cond, body))
            .collect();

        write!(
            fmt,
            "if {} then {} {}else {} end",
            main_cond, main_body, alts, self.fallback
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprReduce {
    binding: ExprBinding,
    acc: Expr,
    eval: Expr,
}

impl ExprReduce {
    pub fn new(binding: ExprBinding, acc: Expr, eval: Expr) -> Self {
        ExprReduce { binding, acc, eval }
    }
}

impl Display for ExprReduce {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "reduce {} ({}; {})", self.binding, self.acc, self.eval)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprForeach {
    binding: ExprBinding,
    init: Expr,
    update: Expr,
    extract: Expr,
}

impl ExprForeach {
    pub fn new(binding: ExprBinding, init: Expr, update: Expr, extract: Expr) -> Self {
        ExprForeach {
            binding,
            init,
            update,
            extract,
        }
    }
}

impl Display for ExprForeach {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(
            fmt,
            "foreach {} ({}; {}; {})",
            self.binding, self.init, self.update, self.extract
        )
    }
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

impl Display for ExprTry {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        if let Some(ref catch) = &self.fallback {
            write!(fmt, "try {} catch {}", self.expr, catch)
        } else {
            match self.expr {
                Expr::Assign(_, _) | Expr::AssignOp(_, _, _) | Expr::Binary(_, _, _) => {
                    write!(fmt, "({})?", self.expr)
                }
                _ => write!(fmt, "{}?", self.expr),
            }
        }
    }
}
