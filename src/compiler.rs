use std::str::FromStr;

use pest::Parser;
use pest::iterators::{Pair, Tokens};
use pest::Token;
use pest_derive::Parser;

use crate::bi_operator::BiOperator;
use crate::expression::Expression;
use crate::statement::Statement;
use crate::runtime::{ExecutionContext, RuntimeError};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MeadorParser;

pub struct MeadorCompiler;

impl MeadorCompiler {
    pub fn compile(code: &String) -> Program {
        let parsed_statements = MeadorParser::parse(Rule::program, code)
            .expect("Failed to parse program")
            .next().unwrap()
            .into_inner();

        let mut statements = Vec::new();

        for statement in parsed_statements {
            match statement.as_rule() {
                Rule::EOI => break,
                Rule::statement => {
                    let statement = statement.into_inner().next().unwrap();
                    let statement =  Self::compile_statement(statement);
                    statements.push(statement)
                },
                _ => panic!("Unknown statement: {:?}", statement)
            }
        }

        Program { statements }
    }

    fn compile_statement(stmt: Pair<Rule>) -> Statement {
        match stmt.as_rule() {
            Rule::function_call_stmt => {
                let expression = stmt.into_inner().next().unwrap();
                Statement::FunctionCall(Self::compile_function_call(expression))
            }
            Rule::variable_declaration => {
                let mut inner = stmt.into_inner();
                let name = inner.next().unwrap().as_str().to_string();

                let expression = inner.next().unwrap();
                let expression = Self::compile_expression(expression);

                println!("Variable declaration: {} = {:?}", name, expression);

                Statement::Assignment(name, expression)
            }
            Rule::if_stmt => {
                let mut inner = stmt.into_inner();
                let condition = inner.next().unwrap();
                let condition = Self::compile_expression(condition);
                let body = inner.next().unwrap();
                let body = Self::compile_statement(body);
                let else_body = inner.next().map(|statement| Self::compile_statement(statement));

                Statement::Conditional(condition, Box::new(body), else_body.map(|statement| Box::new(statement)))
            }
            Rule::while_loop => {
                let mut inner = stmt.into_inner();

                let condition = inner.next().unwrap();
                let condition = Self::compile_expression(condition);

                let body = inner.next().unwrap().into_inner().next().unwrap();
                let body = Self::compile_statement(body);

                Statement::Loop(condition, Box::new(body))
            }
            Rule::code_block => {
                let inner = stmt.into_inner()
                    .map(|stmt| stmt.into_inner().next().unwrap());

                let statements = inner.map(|stmt| Self::compile_statement(stmt)).collect();

                Statement::CodeBlock(statements)
            }
            _ => panic!("Unknown statement: {:?}", stmt.as_rule())
        }
    }

    fn compile_expression(pair: Pair<Rule>) -> Expression {
        let mut inner = pair.into_inner();

        let left = inner.next().unwrap().into_inner().next().unwrap();
        let mut left = Self::compile_value_expression(left);

        // Binary expression
        if let Some(operator) = Self::parse_operator(inner.next()) {
            let mut last_operator = operator;

            let right = inner.next().unwrap().into_inner().next().unwrap();
            let mut right = Self::compile_value_expression(right);

            while let Some(next_operator) = Self::parse_operator(inner.next()) {
                let next_right = inner.next().unwrap().into_inner().next().unwrap();
                let next_right = Self::compile_value_expression(next_right);

                // If the next operator has a higher precedence, we need to swap the left and right
                if next_operator.precedence() > operator.precedence() {
                    right = Expression::BinaryExpression(Box::new(right), operator, Box::new(next_right));
                } else {
                    left = Expression::BinaryExpression(Box::new(left), operator, Box::new(right));
                    right = next_right;
                }

                last_operator = next_operator;
            }

            Expression::BinaryExpression(Box::new(left), operator, Box::new(right))
        } else {
            left
        }
    }

    fn parse_operator(pair: Option<Pair<Rule>>) -> Option<BiOperator> {
        pair.and_then(|pair| {
            match pair.as_rule() {
                Rule::bi_operator => {
                    let operator = pair.as_str();
                    Some(BiOperator::from_str(operator).unwrap())
                }
                _ => None
            }
        })
    }

    fn compile_value_expression(value: Pair<Rule>) -> Expression {
        match value.as_rule() {
            Rule::int | Rule::decimal => {
                let number = value.as_str().trim().parse::<f64>().unwrap();
                Expression::Number(number)
            }
            Rule::parenthesis => {
                let expression = value.into_inner().next().unwrap();
                let expression = Self::compile_expression(expression);

                Expression::BracketExpression(Box::new(expression))
            }
            Rule::function_call => Self::compile_function_call(value),
            Rule::ident => {
                let name = value.as_str().to_string();
                Expression::Variable(name)
            }
            _ => panic!("Unknown value: {:?}", value.as_rule()),
        }
    }

    fn compile_function_call(pair: Pair<Rule>) -> Expression {
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let arguments = inner.map(|expression| Self::compile_expression(expression)).collect();

        Expression::Function(name, arguments)
    }

    fn compile_operator(pair: Pair<Rule>) -> BiOperator {
        match pair.as_rule() {
            Rule::bi_operator => {
                let operator = pair.as_str();
                BiOperator::from_str(operator).unwrap()
            }
            _ => panic!("Unknown operator: {:?}", pair)
        }
    }
}

pub struct Program {
    statements: Vec<Statement>
}

impl Program {
    pub fn execute(&self) -> Result<(), RuntimeError> {
        let mut context = ExecutionContext::new();

        for statement in &self.statements {
            statement.execute(&mut context)?;
        }

        Ok(())
    }
}