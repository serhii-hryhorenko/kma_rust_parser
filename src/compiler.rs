use std::str::FromStr;

use anyhow::Result;
use pest::error::LineColLocation;
use pest::iterators::Pair;
use pest::{Parser, Position};
use pest_derive::Parser;
use thiserror::Error;

use crate::bi_operator::BiOperator;
use crate::expression::Expression;
use crate::runtime::{ExecutionContext, RuntimeError};
use crate::statement::Statement;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MeadorParser;

pub struct MeadorCompiler;

#[derive(Debug, Error)]
pub enum CompilationError {
    #[error("Invalid statement at position {pos}: {context}")]
    Statement { pos: ErrorLocation, context: String },
    #[error("Invalid expression at position {pos}: {context}")]
    Expression { pos: ErrorLocation, context: String },
    #[error("Invalid operator at position {pos}: {context}")]
    Operator { pos: ErrorLocation, context: String },
    #[error("Invalid value at position {pos} {context}")]
    Value { pos: ErrorLocation, context: String },
    #[error("Invalid start of program at position {pos}: {context}")]
    StartOfProgram { pos: ErrorLocation, context: String },
}

#[derive(Debug)]
pub enum ErrorLocation {
    LineCol { line: usize, column: usize },
    Position(usize),
}

impl From<LineColLocation> for ErrorLocation {
    fn from(line_col: LineColLocation) -> Self {
        match line_col {
            LineColLocation::Pos((line, column)) => Self::LineCol { line, column },
            LineColLocation::Span((line, column), _) => Self::LineCol { line, column },
        }
    }
}

impl<'a> From<Position<'a>> for ErrorLocation {
    fn from(position: Position) -> Self {
        Self::Position(position.pos())
    }
}

impl std::fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::LineCol { line, column } => write!(f, "line {}, column {}", line, column),
            Self::Position(pos) => write!(f, "position {}", pos),
        }
    }
}

impl MeadorCompiler {
    pub fn compile(code: &str) -> Result<Program, CompilationError> {
        let parsed_statements = MeadorParser::parse(Rule::program, code)
            .map_err(|err| CompilationError::StartOfProgram {
                pos: err.line_col.into(),
                context: "Failed to parse program".to_string(),
            })?
            .next()
            .ok_or(CompilationError::StartOfProgram {
                pos: ErrorLocation::LineCol { line: 0, column: 0 },
                context: "No statements found in program".to_string(),
            })?
            .into_inner();

        let mut statements = Vec::new();

        for statement in parsed_statements {
            match statement.as_rule() {
                Rule::EOI => break,
                Rule::statement => {
                    let statement = statement.into_inner().next().unwrap();
                    let statement = Self::compile_statement(statement)?;
                    statements.push(statement)
                }
                invalid_rule => {
                    return Err(CompilationError::Statement {
                        pos: ErrorLocation::Position(statement.as_span().start_pos().pos()),
                        context: format!("Unexpected input: {:?}", invalid_rule),
                    })
                }
            }
        }

        Ok(Program { statements })
    }

    fn compile_statement(statement: Pair<Rule>) -> Result<Statement, CompilationError> {
        let statement = match statement.as_rule() {
            Rule::function_call_stmt => {
                let expression = statement.into_inner().next().unwrap();
                Statement::FunctionCall(Self::compile_function_call(expression)?)
            }
            Rule::variable_declaration => {
                let mut inner = statement.into_inner();
                let name = inner.next().unwrap().as_str().to_string();

                let expression = inner.next().unwrap();
                let expression = Self::compile_expression(expression)?;

                Statement::Assignment(name, expression)
            }
            Rule::if_stmt => {
                let mut inner = statement.into_inner();

                let condition = inner.next().unwrap();
                let condition = Self::compile_expression(condition)?;

                let body = inner.next().unwrap().into_inner().next().unwrap();
                let body = Self::compile_statement(body)?;

                let else_body = inner.next().map(|statement| {
                    let else_statement = statement.into_inner().next().unwrap();
                    Self::compile_statement(else_statement)
                });

                if let Some(else_body) = else_body {
                    Statement::Conditional(condition, Box::new(body), Some(Box::new(else_body?)))
                } else {
                    Statement::Conditional(condition, Box::new(body), None)
                }
            }
            Rule::while_loop => {
                let mut inner = statement.into_inner();

                let condition = inner.next().unwrap();
                let condition = Self::compile_expression(condition)?;

                let body = inner.next().unwrap().into_inner().next().unwrap();
                let body = Self::compile_statement(body)?;

                Statement::Loop(condition, Box::new(body))
            }
            Rule::code_block => {
                let statements: Result<Vec<Statement>, CompilationError> = statement
                    .into_inner()
                    .map(|stmt| stmt.into_inner().next().unwrap())
                    .map(|stmt| Self::compile_statement(stmt))
                    .collect();

                Statement::CodeBlock(statements?)
            }
            invalid_rule => {
                return Err(CompilationError::Statement {
                    pos: ErrorLocation::Position(statement.as_span().start_pos().pos()),
                    context: format!("Unexpected input: {:?}", invalid_rule),
                })
            }
        };

        Ok(statement)
    }

    fn compile_expression(pair: Pair<Rule>) -> Result<Expression, CompilationError> {
        let mut inner = pair.into_inner();

        let left = inner.next().unwrap().into_inner().next().unwrap();
        let left = Self::compile_value_expression(left)?;

        if let Some(operator) = Self::parse_operator(inner.next()) {
            Self::compile_binary_expression(left, operator, inner)
        } else {
            Ok(left)
        }
    }

    fn compile_binary_expression(
        left: Expression,
        operator: BiOperator,
        mut inner: pest::iterators::Pairs<Rule>,
    ) -> Result<Expression, CompilationError> {
        let right = inner.next().unwrap().into_inner().next().unwrap();
        let right = Self::compile_value_expression(right)?;

        if let Some(next_operator) = Self::parse_operator(inner.next()) {
            if next_operator.precedence() > operator.precedence() {
                let right = Self::compile_binary_expression(right, next_operator, inner)?;
                Ok(Expression::BinaryExpression(
                    Box::new(left),
                    operator,
                    Box::new(right),
                ))
            } else {
                let lh = Expression::BinaryExpression(Box::new(left), operator, Box::new(right));
                Self::compile_binary_expression(lh, next_operator, inner)
            }
        } else {
            Ok(Expression::BinaryExpression(
                Box::new(left),
                operator,
                Box::new(right),
            ))
        }
    }

    fn parse_operator(pair: Option<Pair<Rule>>) -> Option<BiOperator> {
        pair.and_then(|pair| match pair.as_rule() {
            Rule::bi_operator => {
                let operator = pair.as_str();
                Some(BiOperator::from_str(operator).unwrap())
            }
            _ => None,
        })
    }

    fn compile_value_expression(value: Pair<Rule>) -> Result<Expression, CompilationError> {
        let value = match value.as_rule() {
            Rule::int | Rule::decimal => {
                let number = value.as_str().trim().parse::<f64>().map_err(|_| {
                    CompilationError::Value {
                        pos: ErrorLocation::Position(value.as_span().start_pos().pos()),
                        context: "Failed to parse number".to_string(),
                    }
                })?;
                Expression::Number(number)
            }
            Rule::boolean => {
                let boolean = value.as_str().trim().parse::<bool>().map_err(|_| {
                    CompilationError::Value {
                        pos: ErrorLocation::Position(value.as_span().start_pos().pos()),
                        context: "Failed to parse boolean".to_string(),
                    }
                })?;

                Expression::Boolean(boolean)
            }
            Rule::parenthesis => {
                let expression = value.into_inner().next().unwrap();
                let expression = Self::compile_expression(expression)?;

                Expression::BracketExpression(Box::new(expression))
            }
            Rule::function_call => Self::compile_function_call(value)?,
            Rule::ident => {
                let name = value.as_str().to_string();
                Expression::Variable(name)
            }
            _ => {
                return Err(CompilationError::Value {
                    pos: ErrorLocation::Position(value.as_span().start_pos().pos()),
                    context: format!("Unexpected input: {:?}", value),
                })
            }
        };

        Ok(value)
    }

    fn compile_function_call(pair: Pair<Rule>) -> Result<Expression, CompilationError> {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let arguments: Result<Vec<Expression>, CompilationError> = inner
            .map(|expression| Self::compile_expression(expression))
            .collect();

        Ok(Expression::Function(name, arguments?))
    }
}

pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn execute(&self, context: &mut ExecutionContext) -> Result<(), RuntimeError> {
        for statement in &self.statements {
            statement.execute(context)?;
        }

        Ok(())
    }
}
