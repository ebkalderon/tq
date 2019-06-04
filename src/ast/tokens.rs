use std::fmt::{Display, Formatter, Result as FmtResult};

use toml::value::Datetime;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Ident(String);

impl<'a> From<&'a str> for Ident {
    fn from(s: &'a str) -> Self {
        Ident(s.to_owned())
    }
}

impl From<String> for Ident {
    fn from(s: String) -> Self {
        Ident(s)
    }
}

impl Display for Ident {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct IdentPath(Vec<Ident>);

impl<T, U> From<U> for IdentPath
where
    T: Into<Ident>,
    U: IntoIterator<Item = T>,
{
    fn from(iter: U) -> Self {
        IdentPath(iter.into_iter().map(Into::into).collect())
    }
}

impl Display for IdentPath {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        let idents: Vec<_> = self.0.iter().map(|i| i.to_string()).collect();
        write!(fmt, "{}", idents.join("::"))
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Label(Variable);

impl<T: Into<Ident>> From<T> for Label {
    fn from(ident: T) -> Self {
        Label(Variable::from(ident))
    }
}

impl Display for Label {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Variable(Ident);

impl<T: Into<Ident>> From<T> for Variable {
    fn from(ident: T) -> Self {
        Variable(ident.into())
    }
}

impl Display for Variable {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        write!(fmt, "${}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Boolean(bool),
    Datetime(Datetime),
    Float(f64),
    Integer(i64),
    String(String),
}

impl From<bool> for Literal {
    fn from(boolean: bool) -> Self {
        Literal::Boolean(boolean)
    }
}

impl From<Datetime> for Literal {
    fn from(datetime: Datetime) -> Self {
        Literal::Datetime(datetime)
    }
}

impl From<f64> for Literal {
    fn from(float: f64) -> Self {
        Literal::Float(float)
    }
}

impl From<i64> for Literal {
    fn from(int: i64) -> Self {
        Literal::Integer(int)
    }
}

impl<'a> From<&'a str> for Literal {
    fn from(s: &'a str) -> Self {
        Literal::String(s.to_owned())
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

impl Display for Literal {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            Literal::Boolean(ref b) => write!(fmt, "{}", b),
            Literal::Datetime(ref dt) => write!(fmt, "{}", dt),
            Literal::Float(ref f) => write!(fmt, "{}", f),
            Literal::Integer(ref i) => write!(fmt, "{}", i),
            Literal::String(ref s) => write!(fmt, "\"{}\"", s),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum FnParam {
    /// Treat as though a function call with no arguments.
    Function(IdentPath),
    /// Shorthand for `f as $f` (take the result of the function and treat as a value).
    Variable(Variable),
}

impl From<IdentPath> for FnParam {
    fn from(path: IdentPath) -> Self {
        FnParam::Function(path)
    }
}

impl From<Variable> for FnParam {
    fn from(var: Variable) -> Self {
        FnParam::Variable(var)
    }
}

impl Display for FnParam {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match *self {
            FnParam::Function(ref path) => write!(fmt, "{}", path),
            FnParam::Variable(ref var) => write!(fmt, "{}", var),
        }
    }
}
