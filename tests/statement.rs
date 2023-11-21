use meadorc::{compiler::MeadorCompiler, runtime::ExecutionContext, value::Value};

use anyhow::Result;

#[test]
fn test_conditional_statement() -> Result<()> {
    let code = "if 5 > 2 {
            let x = 3;
        } else {
            let x = 4;
        }"
    .to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(3.0))
    );

    Ok(())
}

#[test]
fn test_variable_assignment() -> Result<()> {
    let code = "let x = 2 + 2 * 2;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(6.0))
    );

    Ok(())
}

#[test]
fn test_while_loop() -> Result<()> {
    let code = "let x = 0;
                while x < 10 {
                    let x = x + 1;
                }"
    .to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(10.0))
    );

    Ok(())
}

#[test]
fn test_code_block_statement() -> Result<()> {
    let code = "let x = 2;
                {
                    let y = 3;
                    let x = x + y;
                }"
    .to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(5.0))
    );

    Ok(())
}
