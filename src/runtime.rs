use std::collections::HashMap;

use crate::expression::Value;

pub struct ExecutionContext {
    variables: HashMap<String, Value>,
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &String) -> Option<&Value> {
        self.variables.get(name)
    }
}

#[derive(Debug)]
pub struct RuntimeError {
    message: String
}

impl RuntimeError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Runtime error: {}", self.message)
    }
}

impl std::error::Error for RuntimeError {}

impl From<String> for RuntimeError {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
