use meadorc::{compiler::MeadorCompiler, runtime::ExecutionContext, value::Value};

#[test]
fn test_numeric_expression() {
    let code = "let x = 2.4;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(2.4))
    );
}

#[test]
fn test_functional_expression() {
    let code = "let x = sin(2.4);".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(0.6754631805511511))
    );
}

#[test]
fn test_boolean_expression() {
    let code = "let x = 2 > 1;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_boolean_binary_expression() {
    let code = "let x = 2 > 1 && 3 < 4;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Boolean(true))
    );
}

#[test]
fn test_parenthesis_expression() {
    let code = "let x = (2 + 2) * 2;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(8.0))
    );
}

#[test]
fn test_binary_expression() {
    let code = "let x = 2 + 2 * 2;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(6.0))
    );
}

#[test]
#[should_panic]
fn test_invalid_binary_expression() {
    let code = "let x = 2 + 2 *;".to_string();

    MeadorCompiler::compile(&code);
}

#[test]
#[should_panic]
fn test_incopatible_types_binary_expression() {
    let code = "let x = 2 + true;".to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();
}