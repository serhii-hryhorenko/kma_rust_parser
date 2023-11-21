use std::str::FromStr;

use crate::bi_operator::BiOperator;
use crate::runtime::{ExecutionContext, RuntimeError};
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Boolean(bool),
    BinaryExpression(Box<Expression>, BiOperator, Box<Expression>),
    BracketExpression(Box<Expression>),
    Function(String, Vec<Expression>),
    Variable(String),
}

impl Expression {
    pub fn evaluate(&self, context: &mut ExecutionContext) -> Result<Value, RuntimeError> {
        match self {
            Expression::Number(number) => Ok(Value::from(*number)),
            Expression::Boolean(boolean) => Ok(Value::from(*boolean)),
            Expression::BracketExpression(expression) => expression.evaluate(context),
            Expression::BinaryExpression(left, operator, right) => {
                let left = left.evaluate(context)?;
                let right = right.evaluate(context)?;

                Ok(operator.apply(left, right)?)
            }
            Expression::Function(name, arguments) => {
                let function = Function::from_str(name)?;

                let arguments = arguments
                    .iter()
                    .map(|argument| argument.evaluate(context))
                    .collect::<Result<Vec<Value>, RuntimeError>>()?;

                Ok(function.apply(arguments.as_slice(), context)?)
            }
            Expression::Variable(name) => {
                if let Some(value) = context.get_variable(name) {
                    Ok(*value)
                } else {
                    Err(format!("Unknown variable: {}", name).into())
                }
            }
        }
    }
}

enum Function {
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Print,
}

impl FromStr for Function {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sin" => Ok(Function::Sin),
            "cos" => Ok(Function::Cos),
            "tan" => Ok(Function::Tan),
            "asin" => Ok(Function::Asin),
            "acos" => Ok(Function::Acos),
            "atan" => Ok(Function::Atan),
            "print" => Ok(Function::Print),
            _ => Err(format!("Unknown function: {}", value)),
        }
    }
}

impl Function {
    fn apply(
        &self,
        arguments: &[Value],
        context: &mut ExecutionContext,
    ) -> Result<Value, RuntimeError> {
        use Function::*;
        use Value::Numerical;

        let result = match self {
            Sin => {
                if let [Numerical(number)] = arguments {
                    number.sin().into()
                } else {
                    return Err("Invalid arguments for sin function".to_string().into());
                }
            }
            Cos => {
                if let [Numerical(number)] = arguments {
                    number.cos().into()
                } else {
                    return Err("Invalid arguments for cos function".to_string().into());
                }
            }
            Tan => {
                if let [Numerical(number)] = arguments {
                    number.tan().into()
                } else {
                    return Err("Invalid arguments for tan function".to_string().into());
                }
            }
            Asin => {
                if let [Numerical(number)] = arguments {
                    number.asin().into()
                } else {
                    return Err("Invalid arguments for asin function".to_string().into());
                }
            }
            Acos => {
                if let [Numerical(number)] = arguments {
                    number.acos().into()
                } else {
                    return Err("Invalid arguments for acos function".to_string().into());
                }
            }
            Atan => {
                if let [Numerical(number)] = arguments {
                    number.atan().into()
                } else {
                    return Err("Invalid arguments for atan function".to_string().into());
                }
            }
            Print => {
                context.write(arguments);
                Value::Void
            }
        };

        Ok(result)
    }
}
