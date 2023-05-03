//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TypeTests.cs>
//!
//! Tests that check runtime validation of registered functions were omitted,
//! because Rust's type system already guarantees correctness at compile time.
//! This affects the following tests:
//! - `TestFailingFunctionDeclarationReturnType`
//! - `TestFailingFunctionDeclarationParameterType`

use crate::test_base::*;
use yarn_slinger_compiler::prelude::*;
use yarn_slinger_core::prelude::*;

mod test_base;
/*
[Fact]
       void TestVariableDeclarationsParsed()
       {
           var source = CreateTestNode(@"
           <<declare $int = 5>>
           <<declare $str = ""yes"">>

           // These value changes are allowed,
           // because they match the declared type
           <<set $int = 6>>
           <<set $str = ""no"">>
           <<set $bool = false>>

           // Declarations are allowed anywhere in the program
           <<declare $bool = true>>
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Diagnostics.Should().BeEmpty();

           var expectedDeclarations = new List<Declaration>() {
               new Declaration {
                   Name = "$int",
                   Type = BuiltinTypes.Number,
                   DefaultValue = 5f,
                   Range = new Yarn.Compiler.Range {
                       Start = {
                           Line = 3,
                           Character = 22,
                       },
                       End = {
                           Line = 3,
                           Character = 26,
                       }
                   },
                   SourceNodeName = "Start",
                   SourceFileName = "input",
               },
               new Declaration {
                   Name = "$str",
                   Type = BuiltinTypes.String,
                   DefaultValue = "yes",
                   Range = new Yarn.Compiler.Range {
                       Start = {
                           Line = 4,
                           Character = 22,
                       },
                       End = {
                           Line = 4,
                           Character = 26,
                       }
                   },
                   SourceNodeName = "Start",
                   SourceFileName = "input",
               },
               new Declaration {
                   Name = "$bool",
                   Type = BuiltinTypes.Boolean,
                   DefaultValue = true,
                   Range = new Yarn.Compiler.Range {
                       Start = {
                           Line = 13,
                           Character = 22,
                       },
                       End = {
                           Line = 13,
                           Character = 27,
                       }
                   },
                   SourceNodeName = "Start",
                   SourceFileName = "input",
               },
           };

           var actualDeclarations = new List<Declaration>(result.Declarations);

           for (int i = 0; i < expectedDeclarations.Count; i++)
           {
               Declaration expected = expectedDeclarations[i];
               Declaration actual = actualDeclarations[i];

               actual.Name.Should().Be(expected.Name);
               actual.Type.Should().Be(expected.Type);
               actual.DefaultValue.Should().Be(expected.DefaultValue);
               actual.Range.Should().Be(expected.Range);
               actual.SourceNodeName.Should().Be(expected.SourceNodeName);
               actual.SourceFileName.Should().Be(expected.SourceFileName);
           }
       }
*/

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
        .into_iter()
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

/*
[Fact]
       public void test_declarations_can_appear_in_other_files()
       {
           // Create two separately-compiled compilation units that each
           // declare a variable that's modified by the other
           var sourceA = CreateTestNode(@"
           <<declare $varB = 1>>
           <<set $varA = 2>>
           ", "NodeA");

           var sourceB = CreateTestNode(@"
           <<declare $varA = 1>>
           <<set $varB = 2>>
           ", "NodeB");

           var compilationJob = new CompilationJob
           {
               Files = new[] {
                   new CompilationJob.File { FileName = "sourceA", Source = sourceA  },
                   new CompilationJob.File { FileName = "sourceB", Source = sourceB  },
               },
           };

           var result = Compiler.Compile(compilationJob);

           result.Diagnostics.Should().BeEmpty();
       }
*/

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

/*
       [Fact]
       public void TestImportingVariableDeclarations()
       {
           var source = CreateTestNode(@"
           <<set $int = 6>> // no error; declaration is imported
           ");

           var declarations = new[] {
               new Declaration {
                   Name = "$int",
                   Type = BuiltinTypes.Number,
                   DefaultValue = 0,
               }
           };

           CompilationJob compilationJob = CompilationJob.CreateFromString("input", source);

           // Provide the declarations
           compilationJob.VariableDeclarations = declarations;

           // Should compile with no errors because $int was declared
           var result = Compiler.Compile(compilationJob);

           result.Diagnostics.Should().BeEmpty();

           // No variables are declared in the source code, so we should
           // expect an empty collection of variable declarations
           result.Declarations.Should().BeEmpty();
       }

*/

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

/*
       [Fact]
       public void TestVariableDeclarationsDisallowDuplicates()
       {
           var source = CreateTestNode(@"
           <<declare $int = 5>>
           <<declare $int = 6>> // error! redeclaration of $int
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Diagnostics.Should().Contain(p => p.Message.Contains("$int has already been declared"));
       }
*/

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

/*

       [Fact]
       public void TestExpressionsDisallowMismatchedTypes()
       {
           var source = CreateTestNode(@"
           <<declare $int = 5>>
           <<set $int = ""5"">> // error, can't assign string to a variable declared int
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Diagnostics.Should().Contain(p => p.Message == "$int (Number) cannot be assigned a String");
       }
*/

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

/*
       [Theory]
       [InlineData(@"<<set $str = ""hi"">>")] // in commands
       [InlineData(@"{$str + 1}")] // in inline expressions
       public void TestExpressionsAllowsUsingUndeclaredVariables(string testSource)
       {
           var source = CreateTestNode($@"
           {testSource}
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Diagnostics.Should().BeEmpty();
       }
*/

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

/*
[Theory]
       [CombinatorialData]
       public void TestExpressionsRequireCompatibleTypes(bool declare)
       {
           var source = CreateTestNode($@"
           {(declare ? "<<declare $int = 0>>" : "")}
           {(declare ? "<<declare $bool = false>>" : "")}
           {(declare ? "<<declare $str = \"\">>" : "")}

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

           <<set $str = ""hello"">>
           <<set $str = ""hel"" + ""lo"">>

           <<set $bool = true>>
           <<set $bool = 1 > 1>>
           <<set $bool = 1 < 1>>
           <<set $bool = 1 <= 1>>
           <<set $bool = 1 >= 1>>

           <<set $bool = ""hello"" == ""hello"">>
           <<set $bool = ""hello"" != ""goodbye"">>
           <<set $bool = 1 == 1>>
           <<set $bool = 1 != 2>>
           <<set $bool = true == true>>
           <<set $bool = true != false>>

           <<set $bool = (1 + 1) > 2>>
           ");

           // Should compile with no exceptions
           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Declarations.Should().Contain(d => d.Name == "$int").Which.Type.Should().Be(BuiltinTypes.Number);
           result.Declarations.Should().Contain(d => d.Name == "$bool").Which.Type.Should().Be(BuiltinTypes.Boolean);
           result.Declarations.Should().Contain(d => d.Name == "$str").Which.Type.Should().Be(BuiltinTypes.String);

           result.Diagnostics.Should().BeEmpty();
       }

*/

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

/*

       [Fact]
       public void TestNullNotAllowed()
       {
           var source = CreateTestNode(@"
           <<declare $err = null>> // error, null not allowed
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));

           result.Diagnostics.Should().Contain(p => p.Message.Contains("Null is not a permitted type"));
       }
*/

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

/*
       [Theory]
       [InlineData("<<set $bool = func_void_bool()>>")]
       [InlineData("<<set $bool = func_int_bool(1)>>")]
       [InlineData("<<set $bool = func_int_int_bool(1, 2)>>")]
       [InlineData(@"<<set $bool = func_string_string_bool(""1"", ""2"")>>")]
       public void TestFunctionSignatures(string source)
       {
           dialogue.Library.RegisterFunction("func_void_bool", () => true);
           dialogue.Library.RegisterFunction("func_int_bool", (int i) => true);
           dialogue.Library.RegisterFunction("func_int_int_bool", (int i, int j) => true);
           dialogue.Library.RegisterFunction("func_string_string_bool", (string i, string j) => true);

           var correctSource = CreateTestNode(source);

           // Should compile with no exceptions
           var result = Compiler.Compile(CompilationJob.CreateFromString("input", correctSource, dialogue.Library));

           // We should have no diagnostics.
           result.Diagnostics.Should().BeEmpty();

           // The variable '$bool' should have an implicit declaration. The
           // type of the variable should be Boolean, because that's the return
           // type of all of the functions we declared.
           result.Declarations.Where(d => d.Name == "$bool")
               .Should().ContainSingle().Which.Type.Should().Be(BuiltinTypes.Boolean);
       }
*/

#[test]
fn test_function_signatures() {
    let mut test_base = TestBase::default();
    test_base
        .dialogue
        .library
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
        let compilation_job = CompilationJob::from_test_source(source)
            .with_library(test_base.dialogue.library.clone());
        let result = compile(compilation_job).unwrap_pretty();

        // The variable '$bool' should have an implicit declaration. The
        // type of the variable should be Boolean, because that's the return
        // type of all of the functions we declared.
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$bool" && d.r#type == Type::Boolean));
    }
}

/*
       public void TestOperatorsAreTypeChecked([CombinatorialValues(
           "= 1 + 1",
           "= 1 / 1",
           "= 1 - 1",
           "= 1 * 1",
           "= 1 % 1",
           "+= 1",
           "-= 1",
           "/= 1",
           "*= 1"
           )] string operation, bool declared)
       {

           string source = CreateTestNode($@"
               {(declared ? "<<declare $var = 0>>" : "")}
               <<set $var {operation}>>
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source, dialogue.Library));

           result.Declarations.Should().Contain(d => d.Name == "$var")
               .Which.Type.Should().Be(BuiltinTypes.Number);

           result.Diagnostics.Should().BeEmpty();

       }
*/

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

            let compilation_job = CompilationJob::from_test_source(&source)
                .with_library(test_base.dialogue.library.clone());
            let result = compile(compilation_job).unwrap_pretty();

            assert!(result
                .declarations
                .iter()
                .any(|d| d.name == "$var" && d.r#type == Type::Number));
        }
    }
}

/*
[Theory]
       [InlineData("<<set $bool = func_void_bool(1)>>", "expects 0 parameters, but received 1")]
       [InlineData("<<set $bool = func_int_bool()>>", "expects 1 parameter, but received 0")]
       [InlineData("<<set $bool = func_int_bool(true)>>", "expects a Number, not a Bool")]
       [InlineData(@"<<set $bool = func_string_string_bool(""1"", 2)>>", "expects a String, not a Number")]
       [InlineData("<<set $int = func_void_bool()>>", @"$int (Number) cannot be assigned a Bool")]
       public void TestFailingFunctionSignatures(string source, string expectedExceptionMessage)
       {
           dialogue.Library.RegisterFunction("func_void_bool", () => true);
           dialogue.Library.RegisterFunction("func_int_bool", (int i) => true);
           dialogue.Library.RegisterFunction("func_int_int_bool", (int i, int j) => true);
           dialogue.Library.RegisterFunction("func_string_string_bool", (string i, string j) => true);

           var failingSource = CreateTestNode($@"
               <<declare $bool = false>>
               <<declare $int = 1>>
               {source}
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", failingSource, dialogue.Library));

           var diagnosticMessages = result.Diagnostics.Select(d => d.Message);

           diagnosticMessages.Should().ContainMatch($"*{expectedExceptionMessage}*");
       }
*/

#[test]
fn test_failing_function_signatures() {
    let mut test_base = TestBase::default();
    test_base
        .dialogue
        .library
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
            .with_library(test_base.dialogue.library.clone());
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
#[ignore]
fn test_initial_values() {
    todo!("Not ported yet")
}

/*
       [Fact]
       public void TestExplicitTypes()
       {
           var source = CreateTestNode(@"
           <<declare $str = ""hello"" as string>>
           <<declare $int = 1 as number>>
           <<declare $bool = false as bool>>
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source, dialogue.Library));

           result.Diagnostics.Should().BeEmpty();

           var variableDeclarations = result.Declarations.Where(d => d.Name.StartsWith("$"));

           variableDeclarations.Should().Contain(d => d.Name == "$str").Which.Type.Should().Be(BuiltinTypes.String);
           variableDeclarations.Should().Contain(d => d.Name == "$int").Which.Type.Should().Be(BuiltinTypes.Number);
           variableDeclarations.Should().Contain(d => d.Name == "$bool").Which.Type.Should().Be(BuiltinTypes.Boolean);
       }
*/

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
        .filter(|d| d.name.starts_with("$"))
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

/*

       [Theory]
       [InlineData(@"<<declare $str = ""hello"" as number>>")]
       [InlineData(@"<<declare $int = 1 as bool>>")]
       [InlineData(@"<<declare $bool = false as string>>")]
       public void TestExplicitTypesMustMatchValue(string test)
       {
           var source = CreateTestNode(test);

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source, dialogue.Library));

           result.Diagnostics.Should().Contain(d => d.Severity == Diagnostic.DiagnosticSeverity.Error);
       }
*/

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

/*
[Fact]
        public void TestVariableDeclarationAnnotations()
        {
            var source = CreateTestNode(@"
            /// prefix: a number
            <<declare $prefix_int = 42>>

            /// prefix: a string
            <<declare $prefix_str = ""Hello"">>

            /// prefix: a bool
            <<declare $prefix_bool = true>>

            <<declare $suffix_int = 42>> /// suffix: a number

            <<declare $suffix_str = ""Hello"">> /// suffix: a string

            <<declare $suffix_bool = true>> /// suffix: a bool

            // No declaration before
            <<declare $none_int = 42>> // No declaration after

            /// Multi-line
            /// doc comment
            <<declare $multiline = 42>>

            ");

            var result = Compiler.Compile(CompilationJob.CreateFromString("input", source, dialogue.Library));

            result.Diagnostics.Should().BeEmpty();

            var expectedDeclarations = new List<Declaration>() {
                new Declaration {
                    Name = "$prefix_int",
                    Type = BuiltinTypes.Number,
                    DefaultValue = 42f,
                    Description = "prefix: a number",
                },
                new Declaration {
                    Name = "$prefix_str",
                    Type = BuiltinTypes.String,
                    DefaultValue = "Hello",
                    Description = "prefix: a string",
                },
                new Declaration {
                    Name = "$prefix_bool",
                    Type = BuiltinTypes.Boolean,
                    DefaultValue = true,
                    Description = "prefix: a bool",
                },
                new Declaration {
                    Name = "$suffix_int",
                    Type = BuiltinTypes.Number,
                    DefaultValue = 42f,
                    Description = "suffix: a number",
                },
                new Declaration {
                    Name = "$suffix_str",
                    Type = BuiltinTypes.String,
                    DefaultValue = "Hello",
                    Description = "suffix: a string",
                },
                new Declaration {
                    Name = "$suffix_bool",
                    Type = BuiltinTypes.Boolean,
                    DefaultValue = true,
                    Description = "suffix: a bool",
                },
                new Declaration {
                    Name = "$none_int",
                    Type = BuiltinTypes.Number,
                    DefaultValue = 42f,
                    Description = null,
                },
                new Declaration {
                    Name = "$multiline",
                    Type = BuiltinTypes.Number,
                    DefaultValue = 42f,
                    Description = "Multi-line doc comment",
                },
            };

            var actualDeclarations = new List<Declaration>(result.Declarations);

            actualDeclarations.Count().Should().Be(expectedDeclarations.Count());

            for (int i = 0; i < expectedDeclarations.Count; i++)
            {
                Declaration expected = expectedDeclarations[i];
                Declaration actual = actualDeclarations[i];

                actual.Name.Should().Be(expected.Name);
                actual.Type.Should().Be(expected.Type);
                actual.DefaultValue.Should().Be(expected.DefaultValue);
                actual.Description.Should().Be(expected.Description);
            }

        }
 */

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
#[ignore]
fn test_type_conversion() {
    todo!("Not ported yet");
}

#[test]
#[ignore]
fn test_type_conversion_failure() {
    todo!("Not ported yet");
}

#[test]
#[ignore]

fn test_implicit_function_declarations() {
    todo!("Not ported yet");
}

/*
       [Theory]
       [InlineData("1", "Number")]
       [InlineData("\"hello\"", "String")]
       [InlineData("true", "Bool")]
       public void TestImplicitVariableDeclarations(string value, string typeName) {
           var source = CreateTestNode($@"
           <<set $v = {value}>>
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("<input>", source));

           result.Diagnostics.Should().BeEmpty();

           result.Declarations.Should().ContainSingle(d => d.Name == "$v")
               .Which.Type.Name.Should().Be(typeName);
       }

*/

#[test]
fn test_implicit_variable_declarations() {
    for (value, type_name) in vec![("1", "Number"), ("\"hello\"", "String"), ("true", "Bool")] {
        let compilation_job = CompilationJob::from_test_source(&format!("<<set $v = {value}>>"));

        let result = compile(compilation_job).unwrap_pretty();

        assert_eq!(1, result.declarations.len());
        assert!(result
            .declarations
            .iter()
            .any(|d| d.name == "$v" && d.r#type.name() == type_name));
    }
}
