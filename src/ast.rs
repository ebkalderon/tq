use std::path::PathBuf;

use toml::value::Datetime;

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub struct Label(pub Ident);

#[derive(Debug)]
pub struct Variable(pub Ident);

#[derive(Debug)]
pub enum Key {
    Ident(Ident),
    String(String),
}

#[derive(Debug)]
pub enum UnaryOp {
    /// The unary `+` operator.
    Pos,
    /// The unary `-` operator.
    Neg,
}

#[derive(Debug)]
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
    /// The binary `|` operator.
    Pipe,
    /// The binary `//` operator.
    Alt,
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Datetime(Datetime),
    Null,
}

#[derive(Debug)]
pub struct Reduce {
    pub expr: Expr,
    pub var: Variable,
    pub acc: Expr,
    pub eval: Expr,
}

#[derive(Debug)]
pub struct ForEach {
    pub expr: Expr,
    pub var: Variable,
    pub init: Expr,
    pub update: Expr,
    pub extract: Expr,
}

#[derive(Debug)]
pub struct FnDecl {
    pub name: Ident,
    pub params: Vec<Parameter>,
}

#[derive(Debug)]
pub enum Parameter {
    Ident(Ident),
    Variable(Variable),
}

#[derive(Debug)]
pub enum Slice {
    Lower(Box<Expr>),
    Upper(Box<Expr>),
    Exact(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Index {
    Exact(Box<Expr>),
    Slice(Slice),
    Iterate,
}

#[derive(Debug)]
pub enum Pattern {
    Variable(Variable),
    Array(Vec<Pattern>),
    Table(Vec<(Key, Pattern)>),
}

#[derive(Debug)]
pub enum Expr {
    /// `.`
    Identity,
    /// `..`
    Recurse,

    /// `12`, `-4.0`, `false`, `"foo"`, `'bar'`
    Value(Value),
    /// `[1, 2, 3, 4]`, `[map(. + 1)]`
    Array(Vec<Expr>),
    /// `thing = { foo = "bar", baz = 5 }`
    Table(Vec<(Key, Expr)>),
    /// `map(. + 1)`
    FnCall(Ident, Vec<Expr>),
    /// `$bar`
    Variable(Variable),

    /// `-12`, `+15.0`
    Unary(UnaryOp, Box<Expr>),
    /// `.foo + 5`, `.dependencies[] | .version`
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    /// `.package.name = "foo"`
    Assign(Box<Expr>, Box<Expr>),
    /// `.package.authors[] += "suffix"`
    AssignOp(BinaryOp, Box<Expr>, Box<Expr>),
    /// `.dependencies, .dev-dependencies, .build-dependencies`
    Comma(Vec<Expr>),

    /// `.package`, `.dependencies.log`
    Field(Box<Expr>, Ident),
    /// `.package.authors[]`, `.package.authors[1]`, `.package.authors[2:5]`
    Index(Box<Expr>, Index),
    /// `.package as $pkg`
    /// `.package as { name = $name, authors = $authors }`
    Binding(Box<Expr>, Pattern),

    /// `label $foo`
    Label(Label),
    /// `label $out | ... break $out ...`
    Break(Label),

    /// `if A then B elif C else D end`
    IfElse(Vec<(Expr, Expr)>, Box<Expr>),
    /// `reduce EXPR as $var (ACC; EVAL)`
    Reduce(Box<Reduce>),
    /// `foreach EXPR as $var (INIT; UPDATE; EXTRACT)`
    For {
        cond: Box<Expr>,
        counter: Pattern,
        init: Box<Expr>,
        update: Box<Expr>,
        extract: Option<Box<Expr>>,
    },
    /// `.package.name?`, `try .package.name`, `try .package.name catch 'nonexistent'`
    Try(Box<Expr>, Option<Box<Expr>>),

    /// `def increment: . + 1;`
    /// `def addvalue(f): f as $f | map(. + $f);`
    /// `def addvalue($f): map(. + $f);`
    Fn(FnDecl, Vec<Stmt>),
}

#[derive(Debug)]
pub enum Stmt {
    /// `include "foo/bar";`
    IncludeMod(PathBuf),
    /// `import "foo/bar";`, `import "foo/bar" as bar;`
    ImportMod(PathBuf, Option<Ident>),
    /// `import "foo/bar" as $bar;`
    ImportToml(PathBuf, Variable),
    /// Main expression to evaluate.
    Expr(Expr),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_ast() {
        let val = ::grammar::FilterParser::new().parse("import \"blah/thing\" as $blah; .").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("import 'thing'; { blah = { thing = map(. + 1) } }").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("thing = 5 == 5").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("{}").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("[]").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse(".foo?").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("{ thing = 1, blah = .package.thing }").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse("[1, 2, 3] | map(. + 1)").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse(".package[], .dependencies[] | . + 1").unwrap();
        println!("{:?}", val);

        let val = ::grammar::FilterParser::new().parse(".name.thing").unwrap();
        println!("{:?}", val);
    }
}
