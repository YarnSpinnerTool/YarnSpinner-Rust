//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TypeTests.cs>
//!
//! Tests that check runtime validation of registered functions were omitted,
//! because Rust's type system already guarantees correctness at compile time.
//! This affects the following tests:
//! - `TestFailingFunctionDeclarationReturnType`
//! - `TestFailingFunctionDeclarationParameterType`
//!
//! Our [`Declaration`] and [`types::FunctionType`] types already support builder semantics, so the following tests were omitted:
//! - `TestDeclarationBuilderCanBuildDeclarations`
//! - `TestFunctionTypeBuilderCanBuildTypes`
//!
//! Because of our different (and imo better) visibility granularity, we have no access to `Type::EXPLICITLY_CONSTRUCTABLE`,
//! so the following (fairly useless) test was omitted:
//! - `TestBuiltinTypesAreEnumerated`

use test_base::prelude::*;
use yarn_slinger::prelude::*;

mod test_base;

#[test]
fn test_variable_declarations_parsed() {
    let compilation_job = CompilationJob::from_test_source(
        r#"
            <<declare $int = 5>>
            <<declare $str = "yes">>

            // These value changes are allowed,
            // because they match the declared type
            <<set $int = 6>>
            <<set $str = "no">>
            <<set $bool = false>>

            // Declarations are allowed anywhere in the program
            <<declare $bool = true>>
"#,
    );
    let result = compile(compilation_job).unwrap_pretty();
    let expected_declarations = &[
        Declaration::new("$int", Type::Number)
            .with_default_value(5.0)
            .with_range(
                Position {
                    line: 3,
                    character: 22,
                }..Position {
                    line: 3,
                    character: 26,
                },
            )
            .with_source_node_name("Start")
            .with_source_file_name("<input>"),
        Declaration::new("$str", Type::String)
            .with_default_value("yes")
            .with_range(
                Position {
                    line: 4,
                    character: 22,
                }..Position {
                    line: 4,
                    character: 26,
                },
            )
            .with_source_node_name("Start")
            .with_source_file_name("<input>"),
        Declaration::new("$bool", Type::Boolean)
            .with_default_value(true)
            .with_range(
                Position {
                    line: 13,
                    character: 22,
                }..Position {
                    line: 13,
                    character: 27,
                },
            )
            .with_source_node_name("Start")
            .with_source_file_name("<input>"),
    ];

    let actual_declarations = result.declarations;
    for (expected, actual) in expected_declarations
        .iter()
        .zip(actual_declarations.into_iter())
    {
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.r#type, actual.r#type);
        assert_eq!(expected.default_value, actual.default_value);
        assert_eq!(expected.range, actual.range);
        assert_eq!(expected.source_node_name, actual.source_node_name);
        assert_eq!(expected.source_file_name, actual.source_file_name);
    }
}

#[test]
fn test_declarations_can_appear_in_other_files() {
    // Create two separately-compiled compilation units that each
    // declare a variable that's modified by the other
    let source_a = create_test_node_with_name(
        "
            <<declare $varB = 1>>
            <<set $varA = 2>>
            ",
        "NodeA",
    );
    let source_b = create_test_node_with_name(
        "
            <<declare $varA = 1>>
            <<set $varB = 2>>
            ",
        "NodeB",
    );
    let compilation_job = CompilationJob::default()
        .with_file(File {
            file_name: "sourceA".to_owned(),
            source: source_a,
        })
        .with_file(File {
            file_name: "sourceB".to_owned(),
            source: source_b,
        });
    let _result = compile(compilation_job).unwrap_pretty();
}

#[test]
fn test_importing_variable_declarations() {
    let compilation_job =
        CompilationJob::from_test_source("<<set $int = 6>> // no error; declaration is imported")
            .with_variable_declaration(
                Declaration::new("$int", Type::Number).with_default_value(0.0),
            );

    let result = compile(compilation_job).unwrap_pretty();
    // No variables are declared in the source code, so we should
    // expect an empty collection of variable declarations
    assert!(result.declarations.is_empty())
}

#[test]
fn test_variable_declarations_disallow_duplicates() {
    let compilation_job = CompilationJob::from_test_source(
        "
            <<declare $int = 5>>
            <<declare $int = 6>> // error! redeclaration of $int
            ",
    );

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("$int has already been declared")));
}

#[test]
fn test_expressions_disallow_mismatched_types() {
    let compilation_job = CompilationJob::from_test_source(
        "
            <<declare $int = 5>>
            <<set $int = \"5\">> // error, can't assign string to a variable declared int
            ",
    );

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message == "$int (Number) cannot be assigned a String"));
}

#[test]
fn test_expressions_allows_using_undeclared_variable() {
    for source in [
        "<<set $str = \"hi\">>", //  // in commands
        "{$str + 1}",            // in inline expressions
    ] {
        let compilation_job = CompilationJob::from_test_source(source);
        let _result = compile(compilation_job).unwrap_pretty();
    }
}

#[test]
fn test_expressions_require_compatible_types() {
    for declare in [true, false] {
        let source = format!(
            "
           {}
           {}
           {}

           <<set $int = 1>>
           <<set $int = 1 + 1>>
           <<set $int = 1 - 1>>
           <<set $int = 1 * 2>>
           <<set $int = 1 / 2>>
           <<set $int = 1 % 2>>
           <<set $int += 1>>
           <<set $int -= 1>>
           <<set $int *= 1>>
           <<set $int /= 1>>
           <<set $int %= 1>>

           <<set $str = \"hello\">>
           <<set $str = \"hel\" + \"lo\">>

           <<set $bool = true>>
           <<set $bool = 1 > 1>>
           <<set $bool = 1 < 1>>
           <<set $bool = 1 <= 1>>
           <<set $bool = 1 >= 1>>

           <<set $bool = \"hello\" == \"hello\">>
           <<set $bool = \"hello\" != \"goodbye\">>
           <<set $bool = 1 == 1>>
           <<set $bool = 1 != 2>>
           <<set $bool = true == true>>
           <<set $bool = true != false>>

           <<set $bool = (1 + 1) > 2>>
           ",
            declare
                .then_some("<<declare $int = 0>>")
                .unwrap_or_default(),
            declare
                .then_some("<<declare $bool = false>>")
                .unwrap_or_default(),
            declare
                .then_some("<<declare $str = \"\">>")
                .unwrap_or_default()
        );

        let compilation_job = CompilationJob::from_test_source(&source);
        let result = compile(compilation_job).unwrap_pretty();

        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$int" && d.r#type == Type::Number));
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$bool" && d.r#type == Type::Boolean));
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$str" && d.r#type == Type::String));
    }
}

#[test]
fn test_null_not_allowed() {
    let compilation_job =
        CompilationJob::from_test_source("<<declare $err = null>> // error, null not allowed");

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Null is not a permitted type")));
}

#[test]
fn test_function_signatures() {
    let mut test_base = TestBase::default();
    test_base
        .dialogue
        .library_mut()
        .register_function("func_void_bool", || true)
        .register_function("func_int_bool", |_i: i32| true)
        .register_function("func_int_int_bool", |_i: i32, _j: i32| true)
        .register_function("func_string_string_bool", |_i: String, _j: String| true);

    for source in [
        "<<set $bool = func_void_bool()>>",
        "<<set $bool = func_int_bool(1)>>",
        "<<set $bool = func_int_int_bool(1, 2)>>",
        "<<set $bool = func_string_string_bool(\"1\", \"2\")>>",
    ] {
        let compilation_job =
            CompilationJob::from_test_source(source).with_library(test_base.library().clone());
        let result = compile(compilation_job).unwrap_pretty();

        // The variable '$bool' should have an implicit declaration. The
        // type of the variable should be Boolean, because that's the return
        // type of all of the functions we declared.
        assert_eq!(1, result.declarations.len());
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$bool" && d.r#type == Type::Boolean));
    }
}
#[test]
fn test_operators_are_type_checked() {
    let test_base = TestBase::default();
    for operation in [
        "= 1 + 1", "= 1 / 1", "= 1 - 1", "= 1 * 1", "= 1 % 1", "+= 1", "-= 1", "/= 1", "*= 1",
    ] {
        for declared in [true, false] {
            let source = format!(
                "{}\n<<set $var {operation}>>",
                declared
                    .then_some("<<declare $var = 0>>")
                    .unwrap_or_default(),
            );

            let compilation_job =
                CompilationJob::from_test_source(&source).with_library(test_base.library().clone());
            let result = compile(compilation_job).unwrap_pretty();

            assert!(result
                .declarations
                .iter()
                .any(|d| d.name == "$var" && d.r#type == Type::Number));
        }
    }
}

#[test]
fn test_failing_function_signatures() {
    let mut test_base = TestBase::default();
    test_base
        .dialogue
        .library_mut()
        .register_function("func_void_bool", || true)
        .register_function("func_int_bool", |_i: i32| true)
        .register_function("func_int_int_bool", |_i: i32, _j: i32| true)
        .register_function("func_string_string_bool", |_i: String, _j: String| true);

    for (source, expected_exception_message) in [
        (
            "<<set $bool = func_void_bool(1)>>",
            "expects 0 parameters, but received 1",
        ),
        (
            "<<set $bool = func_int_bool()>>",
            "expects 1 parameter, but received 0",
        ),
        (
            "<<set $bool = func_int_bool(true)>>",
            "expects a Number, not a Bool",
        ),
        (
            "<<set $bool = func_string_string_bool(\"1\", 2)>>",
            "expects a String, not a Number",
        ),
        (
            "<<set $int = func_void_bool()>>",
            "$int (Number) cannot be assigned a Bool",
        ),
    ] {
        let failing_source = format!("<<declare $bool = false>>\n<<declare $int = 1>>\n{source}",);

        let compilation_job = CompilationJob::from_test_source(&failing_source)
            .with_library(test_base.library().clone());
        let result = compile(compilation_job).unwrap_err();
        println!("{}", result);

        let diagnostic_messages = result
            .diagnostics
            .iter()
            .map(|d| d.message.clone())
            .collect::<Vec<_>>();

        assert!(diagnostic_messages
            .iter()
            .any(|m| m.contains(expected_exception_message)));
    }
}

#[test]
fn test_initial_values() {
    let source = "
            <<declare $int = 42>>
            <<declare $str = \"Hello\">>
            <<declare $bool = true>>
            // internal decls
            {$int}
            {$str}
            {$bool}
            // external decls
            {$external_int}
            {$external_str}
            {$external_bool}
            ";
    let test_base = TestBase::new().with_test_plan(
        TestPlan::new()
            // internal decls
            .expect_line("42")
            .expect_line("Hello")
            // ## Implementation note:
            // The original uses the default C# bool to string conversion, which capitalizes the first letter,
            // so this would be "True" instead.
            .expect_line("true")
            // external decls
            .expect_line("42")
            .expect_line("Hello")
            // ## Implementation note: See above
            .expect_line("true"),
    );

    let compilations_job = CompilationJob::from_test_source(source)
        .with_library(test_base.library().clone())
        .with_variable_declaration(
            Declaration::new("$external_str", Type::String).with_default_value("Hello"),
        )
        .with_variable_declaration(
            Declaration::new("$external_int", Type::Boolean).with_default_value(true),
        )
        .with_variable_declaration(
            Declaration::new("$external_bool", Type::Number).with_default_value(42),
        );

    let result = compile(compilations_job).unwrap_pretty();

    let mut variable_storage = test_base.variable_storage();
    variable_storage.set("$external_str".to_string(), "Hello".into());
    variable_storage.set("$external_int".to_string(), 42.into());
    variable_storage.set("$external_bool".to_string(), true.into());

    test_base.with_compilation(result).run_standard_testcase();
}

#[test]
fn test_explicit_types() {
    let compilation_job = CompilationJob::from_test_source(
        r#"
        <<declare $str = "hello" as string>>
        <<declare $int = 1 as number>>
        <<declare $bool = false as bool>>
        "#,
    );
    let result = compile(compilation_job).unwrap_pretty();

    let variable_declarations: Vec<_> = result
        .declarations
        .iter()
        .filter(|d| d.name.starts_with('$'))
        .collect();

    assert!(variable_declarations
        .iter()
        .any(|d| d.name == "$str" && d.r#type == Type::String));
    assert!(variable_declarations
        .iter()
        .any(|d| d.name == "$int" && d.r#type == Type::Number));
    assert!(variable_declarations
        .iter()
        .any(|d| d.name == "$bool" && d.r#type == Type::Boolean));
}

#[test]
fn test_explicit_types_must_match_value() {
    for test in [
        r#"<<declare $str = "hello" as number>>"#,
        r#"<<declare $int = 1 as bool>>"#,
        r#"<<declare $bool = false as string>>"#,
    ] {
        let compilation_job = CompilationJob::from_test_source(test);
        let _result = compile(compilation_job).unwrap_err();
    }
}

#[test]
fn test_variable_declaration_annotations() {
    let compilation_job = CompilationJob::from_test_source(
        r#"
        /// prefix: a number
        <<declare $prefix_int = 42>>

        /// prefix: a string
        <<declare $prefix_str = "Hello">>

        /// prefix: a bool
        <<declare $prefix_bool = true>>

        <<declare $suffix_int = 42>> /// suffix: a number

        <<declare $suffix_str = "Hello">> /// suffix: a string

        <<declare $suffix_bool = true>> /// suffix: a bool

        // No declaration before
        <<declare $none_int = 42>> // No declaration after

        /// Multi-line
        /// doc comment
        <<declare $multiline = 42>>
        "#,
    );

    let result = compile(compilation_job).unwrap_pretty();

    let expected_declarations = vec![
        Declaration::new("$prefix_int", Type::Number)
            .with_default_value(42.0)
            .with_description("prefix: a number"),
        Declaration::new("$prefix_str", Type::String)
            .with_default_value("Hello")
            .with_description("prefix: a string"),
        Declaration::new("$prefix_bool", Type::Boolean)
            .with_default_value(true)
            .with_description("prefix: a bool"),
        Declaration::new("$suffix_int", Type::Number)
            .with_default_value(42.0)
            .with_description("suffix: a number"),
        Declaration::new("$suffix_str", Type::String)
            .with_default_value("Hello")
            .with_description("suffix: a string"),
        Declaration::new("$suffix_bool", Type::Boolean)
            .with_default_value(true)
            .with_description("suffix: a bool"),
        Declaration::new("$none_int", Type::Number).with_default_value(42.0),
        Declaration::new("$multiline", Type::Number)
            .with_default_value(42.0)
            .with_description("Multi-line doc comment"),
    ];
    let actual_declarations = result.declarations;

    assert_eq!(expected_declarations.len(), actual_declarations.len());

    for (expected, actual) in expected_declarations.iter().zip(actual_declarations.iter()) {
        assert_eq!(expected.name, actual.name);
        assert_eq!(expected.r#type, actual.r#type);
        assert_eq!(expected.default_value, actual.default_value);
        assert_eq!(expected.description, actual.description);
    }
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
    let compilations_job =
        CompilationJob::from_test_source(source).with_library(test_base.library().clone());
    let result = compile(compilations_job).unwrap_pretty();

    test_base.with_compilation(result).run_standard_testcase();
}

#[test]
#[should_panic = "Failed to convert a Yarn value to a number: ParseFloatError(ParseFloatError { kind: Invalid })"]
fn test_type_conversion_failure_to_number() {
    let source = "{number(\"hello\")}";
    let test_base =
        TestBase::new().with_test_plan(TestPlan::new().expect_line("test failure if seen"));
    let compilations_job =
        CompilationJob::from_test_source(source).with_library(test_base.library().clone());
    let result = compile(compilations_job).unwrap_pretty();
    test_base.with_compilation(result).run_standard_testcase();
}

#[test]
#[should_panic = "Failed to convert a Yarn value to a bool: ParseBoolError(ParseBoolError"]
fn test_type_conversion_failure_to_bool() {
    let source = "{bool(\"hello\")}";
    let test_base =
        TestBase::new().with_test_plan(TestPlan::new().expect_line("test failure if seen"));
    let compilations_job =
        CompilationJob::from_test_source(source).with_library(test_base.library().clone());
    let result = compile(compilations_job).unwrap_pretty();
    test_base.with_compilation(result).run_standard_testcase();
}

#[test]
#[ignore]
fn test_implicit_function_declarations() {
    todo!("Not ported yet");
}

#[test]
fn test_implicit_variable_declarations() {
    for (value, type_name) in [("1", "Number"), ("\"hello\"", "String"), ("true", "Bool")] {
        let compilation_job = CompilationJob::from_test_source(&format!("<<set $v = {value}>>"));

        let result = compile(compilation_job).unwrap_pretty();

        assert_eq!(1, result.declarations.len());
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$v" && d.r#type.name() == type_name));
    }
}

#[test]
#[ignore]
fn test_nested_implicit_function_declarations() {
    todo!("Not ported yet");
}

#[test]
fn test_multiple_implicit_redeclarations_of_function_parameter_count_fail() {
    let compilation_job = CompilationJob::from_test_source(
        r#"
        {func(1)}
        {func(2, 2)} // wrong number of parameters (previous decl had 1)
        "#,
    );

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);

    assert_eq!(
        "Function \"func\" expects 1 parameter, but received 2",
        result.diagnostics[0].message,
    );
}

#[test]
fn test_multiple_implicit_redeclarations_of_function_parameter_type_fail() {
    let compilation_job = CompilationJob::from_test_source(
        "
        {func(1)}
        {func(true)} // wrong type of parameter (previous decl had number)
        ",
    );

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);

    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("expects a Number, not a Bool")));
}

#[test]
fn test_if_statement_expressions_must_be_boolean() {
    let compilation_job = CompilationJob::from_test_source(
        r#"
        <<declare $str = "hello" as string>>
        <<declare $bool = true>>

        <<if $bool>> // ok
        Hello
        <<endif>>

        <<if $str>> // error, must be a bool
        Hello
        <<endif>>
        "#,
    );

    let result = compile(compilation_job).unwrap_err();
    println!("{}", result);

    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Terms of 'if statement' must be Bool, not String")));
}
