use toml::{self, Value as TomlValue};

use crate::ast::Stmt;
use crate::env::Environment;

#[derive(Debug)]
pub struct Machine {
    env: Environment,
    toml: TomlValue,
}

impl Machine {
    pub fn new<T: AsRef<str>>(toml: T) -> Self {
        Machine {
            env: Environment::default(),
            toml: toml::from_str(toml.as_ref()).unwrap(),
        }
    }

    pub fn execute(&mut self, _filter: &[Stmt]) {
        unimplemented!();
    }
}
