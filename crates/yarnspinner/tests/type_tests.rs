//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TypeTests.cs>

use test_base::prelude::*;
use yarnspinner::core::*;
use yarnspinner::runtime::*;

mod test_base;

#[test]
fn test_variable_storage_extended_with_program_initial_values() {
    let source = "
        <<declare $int = 42>>
        <<declare $str = \"Hello\">>
        <<declare $bool = true>>
    ";

    let result = compile_test_source(source).unwrap();

    let mut dialogue = Dialogue::new(
        Box::new(MemoryVariableStorage::new()),
        Box::new(StringTableTextProvider::new()),
    );
    dialogue.replace_program(result.program);

    let variable_storage = dialogue.variable_storage();
    assert_eq!(
        variable_storage.get("$int").unwrap(),
        YarnValue::Number(42.0)
    );
    assert_eq!(
        variable_storage.get("$str").unwrap(),
        YarnValue::String("Hello".to_string())
    );
    assert_eq!(
        variable_storage.get("$bool").unwrap(),
        YarnValue::Boolean(true)
    );
}

#[test]
fn test_type_conversion() {
    let source = "
            string + string(number): {\"1\" + string(1)}
            string + string(bool): {\"1\" + string(true)}

            number + number(string): {1 + number(\"1\")}
            number + number(bool): {1 + number(true)}

            bool and bool(string): {true and bool(\"true\")}
            bool and bool(number): {true and bool(1)}
            ";
    let test_base = TestBase::new().with_test_plan(
        TestPlan::new()
            .expect_line("string + string(number): 11")
            .expect_line("string + string(bool): 1true")
            .expect_line("number + number(string): 2")
            .expect_line("number + number(bool): 2")
            .expect_line("bool and bool(string): true")
            .expect_line("bool and bool(number): true"),
    );
    let result = compile_test_source(source).unwrap();

    test_base.with_compiled(result).run_standard_testcase();
}

#[test]
#[should_panic = "Failed to convert a Yarn value to a number: ParseFloatError(ParseFloatError { kind: Invalid })"]
fn test_type_conversion_failure_to_number() {
    let source = "{number(\"hello\")}";
    let test_base =
        TestBase::new().with_test_plan(TestPlan::new().expect_line("test failure if seen"));
    let result = compile_test_source(source).unwrap();
    test_base.with_compiled(result).run_standard_testcase();
}

#[test]
#[should_panic = "Failed to convert a Yarn value to a bool: ParseBoolError(ParseBoolError"]
fn test_type_conversion_failure_to_bool() {
    let source = "{bool(\"hello\")}";
    let test_base =
        TestBase::new().with_test_plan(TestPlan::new().expect_line("test failure if seen"));
    let result = compile_test_source(source).unwrap();
    test_base.with_compiled(result).run_standard_testcase();
}
