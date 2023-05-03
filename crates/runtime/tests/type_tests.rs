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
        Declaration::default()
            .with_name("$int")
            .with_type(Type::Number)
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
        Declaration::default()
            .with_name("$str")
            .with_type(Type::String)
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
        Declaration::default()
            .with_name("$bool")
            .with_type(Type::Boolean)
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
