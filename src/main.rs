use crate::compiler::MeadorCompiler;
use runtime::{RuntimeError, ExecutionContext};
use compiler::CompilationError;

mod compiler;
mod runtime;
mod expression;
mod statement;
mod value;
mod bi_operator;

#[derive(Debug)]
pub enum Error {
    RuntimeError(RuntimeError),
    CompileError(CompilationError),
}

impl From<RuntimeError> for Error {
    fn from(error: RuntimeError) -> Error {
        Error::RuntimeError(error)
    }
}

impl From<CompilationError> for Error {
    fn from(error: CompilationError) -> Error {
        Error::CompileError(error)
    }
}

pub fn main() -> Result<(), Error> {
    let code = "print(2 * 2 + 2 * 2)".to_string();

    let program = MeadorCompiler::compile(&code).map_err(Error::from)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).map_err(Error::from)
}