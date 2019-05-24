use std::collections::HashMap;

use crate::ast::{FnDecl, Key, Pattern, Stmt, Value, Variable};

pub trait Evaluatable {
    fn eval(&self, env: &mut Environment);
}

#[derive(Debug, Default)]
pub struct Environment {
    functions: HashMap<FnDecl, Vec<Stmt>>,
    variables: HashMap<Variable, Value>,
}

impl Environment {
    pub fn define_function(&mut self, decl: FnDecl, stmts: Vec<Stmt>) {
        self.functions.insert(decl, stmts);
    }

    pub fn bind_variable(&mut self, var: Variable, val: Value) {
        self.variables.insert(var, val);
    }
}
