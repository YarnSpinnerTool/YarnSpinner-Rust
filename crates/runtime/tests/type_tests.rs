//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TypeTests.cs>

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
