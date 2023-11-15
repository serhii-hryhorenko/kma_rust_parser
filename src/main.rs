use crate::compiler::MeadorCompiler;
use runtime::RuntimeError;

mod compiler;
mod runtime;
mod expression;
mod statement;
mod bi_operator;


pub fn main() -> Result<(), RuntimeError> {
    let code = "let x = (6 / (1 + 2 * 2)) * 2;
                        print(x);
                        let y = 2;
                        let z = x + y;
                        ".to_string();

    let program = MeadorCompiler::compile(&code);

    program.execute()
}