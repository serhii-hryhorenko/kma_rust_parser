use crate::{expression::Value, runtime::RuntimeError};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum BiOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Conjuction,
    Disjunction,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl BiOperator {
    pub fn apply(&self, left: Value, right: Value) -> Result<Value, RuntimeError> {
        use BiOperator as Bi;
        use Value::*;

        let result = match (left, right) {
            (Numerical(left), Numerical(right)) => match self {
                Bi::Add => (left + right).into(),
                Bi::Subtract => (left - right).into(),
                Bi::Multiply => (left * right).into(),
                Bi::Divide => (left / right).into(),
                Bi::Power => left.powf(right).into(),
                Bi::LessThan => (left < right).into(),
                Bi::LessThanOrEqual => (left <= right).into(),
                Bi::GreaterThan => (left > right).into(),
                Bi::GreaterThanOrEqual => (left >= right).into(),
                _ => return Err(format!("Invalid types for numerical binary operator: {:?} and {:?}", left, right).into())
            },
            (Boolean(left), Boolean(right)) => match self {
                BiOperator::Conjuction => left && right,
                BiOperator::Disjunction => left || right,
                _ => return Err(
                    format!("Invalid types for logical binary operator: {:?} and {:?}", left, right).into()
                )
            }.into(),

            _ => return Err(format!("Invalid types for binary operator: {:?} and {:?}", left, right).into())
        };

        Ok(result)
    }

    pub fn precedence(&self) -> u8 {
        use BiOperator as Bi;

        match self {
            Bi::Add | BiOperator::Subtract => 1,
            Bi::Multiply | BiOperator::Divide => 2,
            Bi::Power => 3,
            Bi::Conjuction => 4,
            Bi::Disjunction => 5,
            Bi::LessThan | BiOperator::LessThanOrEqual => 6,
            Bi::GreaterThan | BiOperator::GreaterThanOrEqual => 6,
        }
    }
}

impl FromStr for BiOperator {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use BiOperator as Bi;

        let op = match value {
            "+" => Bi::Add,
            "-" => Bi::Subtract,
            "*" => Bi::Multiply,
            "/" => Bi::Divide,
            "^" => Bi::Power,
            "&&" => Bi::Conjuction,
            "||" => Bi::Disjunction,
            "<" => Bi::LessThan,
            "<=" => Bi::LessThanOrEqual,
            ">" => Bi::GreaterThan,
            ">=" => Bi::GreaterThanOrEqual,
            _ => return Err(format!("Unknown BiOperator: {}", value)),
        };

        Ok(op)
    }
}
