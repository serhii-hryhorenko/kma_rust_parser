use meadorc::{compiler::MeadorCompiler, runtime::ExecutionContext, value::Value};

#[test]
fn test_complex_math() {
    let code = "let x = (6 / (1 + 2 ^ 2)) * 2;"
        .to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

    assert_eq!(
        context.get_variable(&"x".to_string()),
        Some(&Value::Numerical(2.4))
    );
}

#[test]
fn test_function_call() {
    let code = "let x = 2;
                let y = sin(x);
                let z = cos(x);"
        .to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

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
}

#[test]
fn test_negative_numbers() {
    let code = "let x = -2;
                let y = sin(x);
                let z = cos(x);"
        .to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();

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
}

#[test]
#[should_panic]
fn test_invalid_program_compilation() {
    let code = "let x = 2 + 2 * 2"
        .to_string();

    MeadorCompiler::compile(&code);
}

#[test]
#[should_panic]
fn test_runtime_error() {
    let code = "let x = 5 + true;"
        .to_string();

    let program = MeadorCompiler::compile(&code);
    let mut context = ExecutionContext::new(std::io::stdout());

    program.execute(&mut context).unwrap();
}