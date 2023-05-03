use crate::test_base::*;
use yarn_slinger_compiler::prelude::*;

mod test_base;

#[test]
fn test_malformed_if_statement() {
    let compilation_job = CompilationJob::from_test_source("<<if true>> // error: no endif");
    let result = compile(compilation_job).unwrap_err();

    println!("{}", result);
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Expected an <<endif>> to match the <<if>> statement on line 3")));
}

/*
       public void TestExtraneousElse() {
           var source = CreateTestNode(@"
           <<if true>>
           One
           <<else>>
           Two
           <<else>>
           Three
           <<endif>>");

           var result = Compiler.Compile(CompilationJob.CreateFromString("<input>", source));

           result.Diagnostics.Should().Contain(d => d.Message.Contains("More than one <<else>> statement in an <<if>> statement isn't allowed"));
           result.Diagnostics.Should().Contain(d => d.Message.Contains("Unexpected \"endif\" while reading a statement"));

       }
*/
#[test]
fn test_extraneous_else() {
    let compilation_job = CompilationJob::from_test_source(
        "<<if true>>\n\
            One\n\
            <<else>>\n\
            Two\n\
            <<else>> // error: more than one else\n\
            Three\n\
            <<endif>>",
    );
    let result = compile(compilation_job).unwrap_err();

    println!("{}", result);
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("More than one <<else>> statement in an <<if>> statement isn't allowed")));
    assert!(result.diagnostics.iter().any(|d| d
        .message
        .contains("Unexpected \"endif\" while reading a statement")));
}

/*

       [Fact]
       public void TestEmptyCommand() {
           var source = CreateTestNode(@"
           <<>>
           ");

           var result = Compiler.Compile(CompilationJob.CreateFromString("<input>", source));

           result.Diagnostics.Should().Contain(d => d.Message.Contains("Command text expected"));
       }

*/

#[test]
fn test_empty_command() {
    let compilation_job = CompilationJob::from_test_source("\n<<>>\n");
    let result = compile(compilation_job).unwrap_err();

    println!("{}", result);
    assert!(result
        .diagnostics
        .iter()
        .any(|d| d.message.contains("Command text expected")));
}
