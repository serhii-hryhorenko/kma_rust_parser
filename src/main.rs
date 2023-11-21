use crate::compiler::MeadorCompiler;
use runtime::{RuntimeError, ExecutionContext};
use compiler::CompilationError;

mod compiler;
mod runtime;
mod expression;
mod statement;
mod value;
mod bi_operator;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    RuntimeError(RuntimeError),
    CompileError(CompilationError),
    InvalidSourcePath(String)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
       match self {
           Error::RuntimeError(error) => write!(f, "{}", error),
           Error::CompileError(error) => write!(f, "{}", error),
           Error::InvalidSourcePath(path) => write!(f, "Invalid source path: {}", path)
       }
    }
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
    let code = std::env::args().nth(1)
        .ok_or_else(|| Error::InvalidSourcePath("No source file provided".to_string()))
        .map(std::path::PathBuf::from)
        .and_then(|path| {
            std::fs::read_to_string(path)
                .map_err(|_| Error::InvalidSourcePath("Failed to read source file".to_string()))
        })?;

    let program = MeadorCompiler::compile(&code).map_err(Error::from)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).map_err(Error::from)
}