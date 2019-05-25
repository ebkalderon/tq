use toml::value::Datetime;

#[derive(Clone, Debug, PartialEq)]
pub struct Ident(String);

impl<T: ToString> From<T> for Ident {
    fn from(string: T) -> Self {
        Ident(string.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
pub struct Variable(Ident);

impl<T: Into<Ident>> From<T> for Variable {
    fn from(ident: T) -> Self {
        Variable(ident.into())
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

#[derive(Clone, Debug, PartialEq)]
pub enum FnParam {
    Ident(Ident),
    Variable(Variable),
}
