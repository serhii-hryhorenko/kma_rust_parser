use meadorc::{compiler::MeadorCompiler, runtime::ExecutionContext, value::Value};

use anyhow::Result;

#[test]
fn test_complex_math() -> Result<()> {
    let code = "let x = (6 / (1 + 2 ^ 2)) * 2;".to_string();

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
fn test_function_call() -> Result<()> {
    let code = "let x = 2;
                let y = sin(x);
                let z = cos(x);"
        .to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(2.0))
    );
    assert_eq!(
        context.get_variable(&"y".to_string()),
        Some(&Value::Numerical(0.9092974268256817))
    );
    assert_eq!(
        context.get_variable(&"z".to_string()),
        Some(&Value::Numerical(-0.4161468365471424))
    );

    Ok(())
}

#[test]
fn test_negative_numbers() -> Result<()> {
    let code = "let x = -2;
                let y = sin(x);
                let z = cos(x);"
        .to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context)?;

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(-2.0))
    );
    assert_eq!(
        context.get_variable(&"y".to_string()),
        Some(&Value::Numerical(-0.9092974268256817))
    );
    assert_eq!(
        context.get_variable(&"z".to_string()),
        Some(&Value::Numerical(-0.4161468365471424))
    );

    Ok(())
}

#[test]
fn test_invalid_program_compilation() -> Result<()> {
    let code = "let x = 2 + 2 * 2".to_string();

    let result = MeadorCompiler::compile(&code);

    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_runtime_error() -> Result<()> {
    let code = "let x = 5 + true;".to_string();

    let program = MeadorCompiler::compile(&code)?;
    let mut context = ExecutionContext::new(std::io::stdout());

    let result = program.execute(&mut context);

    assert!(result.is_err());

    Ok(())
}
