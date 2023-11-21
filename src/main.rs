use crate::compiler::MeadorCompiler;
use runtime::{RuntimeError, ExecutionContext};

mod compiler;
mod runtime;
mod expression;
mod statement;
mod value;
mod bi_operator;

pub fn main() -> Result<(), RuntimeError> {
    let code = "print(2 * 2 + 2 * 2);".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)
}