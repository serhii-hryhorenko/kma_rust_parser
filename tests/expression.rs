use meadorc::{compiler::MeadorCompiler, runtime::ExecutionContext, value::Value};

use anyhow::Result;

#[test]
fn test_numeric_expression() -> Result<()> {
    let code = "let x = 2.4;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(2.4))
    );

    Ok(())
}

#[test]
fn test_functional_expression() -> Result<()> {
    let code = "let x = sin(2.4);".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(0.6754631805511511))
    );

    Ok(())
}

#[test]
fn test_boolean_expression() -> Result<()> {
    let code = "let x = 2 > 1;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Boolean(true))
    );

    Ok(())
}

#[test]
fn test_boolean_binary_expression() -> Result<()> {
    let code = "let x = 2 > 1 && 3 < 4;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Boolean(true))
    );

    Ok(())
}

#[test]
fn test_parenthesis_expression() -> Result<()> {
    let code = "let x = (2 + 2) * 2;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(8.0))
    );

    Ok(())
}

#[test]
fn test_binary_expression() -> Result<()> {
    let code = "let x = 2 + 2 * 2;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(6.0))
    );

    Ok(())
}

#[test]
#[should_panic]
fn test_invalid_binary_expression() {
    let code = "let x = 2 + 2 *;".to_string();

    MeadorCompiler::compile(&code).unwrap();
}

#[test]
#[should_panic]
fn test_incopatible_types_binary_expression() {
    let code = "let x = 2 + true;".to_string();

    let program = MeadorCompiler::compile(&code).unwrap();
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();
}
