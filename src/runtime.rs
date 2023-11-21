use std::collections::HashMap;
use std::io::Write;

use crate::value::Value;

pub struct ExecutionContext {
    variables: HashMap<String, Value>,
    stdout: std::io::Stdout,
}

impl ExecutionContext {
    pub fn new(stdout: std::io::Stdout) -> Self {
        Self { variables: HashMap::new(), stdout }
    }

    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &String) -> Option<&Value> {
        self.variables.get(name)
    }

    pub fn write(&mut self, value: &[Value]) {
        use Value::*;

        for value in value {
            match value {
                Numerical(number) => {
                    write!(self.stdout, "{}", number).unwrap();
                }
                Boolean(boolean) => {
                    write!(self.stdout, "{}", boolean).unwrap();
                }
                Void => {}
            }
        }

        writeln!(self.stdout).unwrap();
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
