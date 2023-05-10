//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/ErrorHandlingTests.cs>

use crate::test_base::*;
use test_base::prelude::*;
use yarn_slinger::prelude::*;

mod test_base;

#[test]
fn test_malformed_if_statement() {
    let compiler = Compiler::from_test_source("<<if true>> // error: no endif");
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Expected an <<endif>> to match the <<if>> statement on line 3")));
}

#[test]
fn test_extraneous_else() {
    let compiler = Compiler::from_test_source(
        "<<if true>>\n\
            One\n\
            <<else>>\n\
            Two\n\
            <<else>> // error: more than one else\n\
            Three\n\
            <<endif>>",
    );
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("More than one <<else>> statement in an <<if>> statement isn't allowed")));
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Unexpected \"endif\" while reading a statement")));
}

#[test]
fn test_empty_command() {
    let compiler = Compiler::from_test_source("\n<<>>\n");
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Command text expected")));
}

#[test]
fn test_invalid_variable_name_in_set_or_declare() {
    let compiler = Compiler::from_test_source("\n<<set test = 1>>\n");
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message == "Variable names need to start with a $"));

    let compiler = Compiler::from_test_source("\n<<declare test = 1>>\n");
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message == "Variable names need to start with a $"));
}

#[test]
fn test_invalid_function_call() {
    let compiler = Compiler::from_test_source("<<if someFunction(>><<endif>>");
    let result = compile(compiler).unwrap_err();

    println!("{}", result);
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Unexpected \">>\" while reading a function call")));
}
