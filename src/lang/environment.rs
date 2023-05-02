use std::collections::HashMap;

use super::Literal;

pub struct Environment {
    variables: HashMap<String, Option<Literal>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Option<Literal>) {
        self.variables.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Option<Literal>) {
        if self.contains(&name) {
            self.variables.insert(name, value);
        }
    }

    pub fn get(&self, name: &String) -> Option<Literal> {
        if self.variables.contains_key(name) {
            return self.variables.get(name).unwrap().clone();
        }

        None
    }

    pub fn contains(&self, name: &String) -> bool {
        self.variables.contains_key(name)
    }
}
