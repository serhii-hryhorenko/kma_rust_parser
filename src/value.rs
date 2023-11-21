#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Numerical(f64),
    Boolean(bool),
    Void,
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Numerical(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Boolean(value)
    }
}
