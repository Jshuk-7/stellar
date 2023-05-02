use std::collections::HashMap;

use super::Literal;

#[derive(Clone)]
pub struct Environment {
    enclosing: Option<Box<Environment>>,
    variables: HashMap<String, Option<Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None, 
            variables: HashMap::new(),
        }
    }

    pub fn from(enclosing: Box<Environment>) -> Self {
        Self {
            enclosing: Some(enclosing),
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Option<Literal>) {
        if self.contains(&name) {
            self.variables.insert(name.clone(), value.clone());
        }

        if let Some(ref mut env) = self.enclosing {
            env.assign(name, value);
        }
    }

    pub fn get(&self, name: &String) -> Option<Literal> {
        if self.variables.contains_key(name) {
            return self.variables.get(name).unwrap().clone();
        }

        if let Some(env) = &self.enclosing {
            return env.get(name);
        }

        None
    }

    pub fn contains(&self, name: &String) -> bool {
        self.variables.contains_key(name)
    }
}
