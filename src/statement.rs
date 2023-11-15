use crate::expression::{Expression, Value};
use crate::runtime::ExecutionContext;
use crate::runtime::RuntimeError;

#[derive(Debug)]
pub enum Statement {
    FunctionCall(Expression),
    Assignment(String, Expression),
    Conditional(Expression, Box<Statement>, Option<Box<Statement>>),
    Loop(Expression, Box<Statement>),
    CodeBlock(Vec<Statement>),
}

impl From<Expression> for Statement {
    fn from(expression: Expression) -> Statement {
        Statement::FunctionCall(expression)
    }
}

impl Statement {
    pub fn execute(&self, context: &mut ExecutionContext) -> Result<(), RuntimeError> {
        match self {
            Statement::FunctionCall(call) => {
                call.evaluate(context)?;
            }
            Statement::Assignment(name, expression) => {
                let value = expression.evaluate(context)?;
                context.set_variable(name.clone(), value);
            }
            Statement::Conditional(condition, body, else_body) => {
                match condition.evaluate(context)? {
                    Value::Boolean(true) => body.execute(context)?,
                    Value::Boolean(false) => {
                        if let Some(else_body) = else_body {
                            else_body.execute(context)?;
                        }
                    }
                    value => {
                        return Err(format!("Invalid condition return type: {:?}", value).into());
                    }
                }
            }
            Statement::Loop(condition, body) => {
                while let Value::Boolean(true) = condition.evaluate(context)? {
                    body.execute(context)?;
                }
            }
            Statement::CodeBlock(statements) => {
                for statement in statements {
                    statement.execute(context)?;
                }
            }
        }

        Ok(())
    }
}
