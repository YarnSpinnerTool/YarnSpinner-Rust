//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TagTests.cs>

use crate::test_base::*;
use yarn_slinger_compiler::prelude::*;

mod test_base;

/*
      [Fact]
       void TestNoOptionsLineNotTagged()
       {
           var source = "title:Start\n---\nline without options #line:1\n===\n";

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));
           result.Diagnostics.Should().BeEmpty();

           var info = result.StringTable["line:1"];

           info.metadata.Should().NotContain("lastline");
       }
*/

#[test]
fn test_no_options_line_not_tagged() {
    let compilation_job =
        CompilationJob::from_test_source("title:Start\n---\nline without options #line:1\n===\n");
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1"];
    assert!(!info.metadata.contains(&"lastline".to_owned()));
}

/*

       [Fact]
       void TestLineBeforeOptionsTaggedLastLine()
       {
           var source = "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\n===\n";

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));
           result.Diagnostics.Should().BeEmpty();

           var info = result.StringTable["line:1"];

           info.metadata.Should().Contain("lastline");
       }
*/

#[test]
fn test_line_before_options_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1"];
    assert!(info.metadata.contains(&"lastline".to_owned()));
}

/*

       [Fact]
       void TestLineNotBeforeOptionsNotTaggedLastLine()
       {
           var source = "title:Start\n---\nline not before options #line:0\nline before options #line:1\n-> option 1\n-> option 2\n===\n";

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));
           result.Diagnostics.Should().BeEmpty();

           var info = result.StringTable["line:0"];

           info.metadata.Should().NotContain("lastline");
       }
*/

#[test]
fn test_line_not_before_options_not_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline not before options #line:0\nline before options #line:1\n-> option 1\n-> option 2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(!info.metadata.contains(&"lastline".to_owned()));
}

/*
       [Fact]
       void TestLineAfterOptionsNotTaggedLastLine()
       {
           var source = "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\nline after options #line:2\n===\n";

           var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));
           result.Diagnostics.Should().BeEmpty();

           var info = result.StringTable["line:2"];

           info.metadata.Should().NotContain("lastline");
       }

*/

#[test]
fn test_line_after_options_not_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\nline after options #line:2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:2"];
    assert!(!info.metadata.contains(&"lastline".to_owned()));
}

/*
 [Fact]
        void TestNestedOptionLinesTaggedLastLine()
        {
            var source = CreateTestNode(@"
line before options #line:1
-> option 1
    line 1a #line:1a
    line 1b #line:1b
    -> option 1a
    -> option 1b
-> option 2
-> option 3
");

            var result = Compiler.Compile(CompilationJob.CreateFromString("input", source));
            result.Diagnostics.Should().BeEmpty();
            var info = result.StringTable["line:1"];
            info.metadata.Should().Contain("lastline");

            info = result.StringTable["line:1b"];
            info.metadata.Should().Contain("lastline");
        }
 */

#[test]
fn test_nested_option_lines_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "
line before options #line:1
-> option 1
    line 1a #line:1a
    line 1b #line:1b
    -> option 1a
    -> option 1b
-> option 2
-> option 3
",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1"];
    assert!(info.metadata.contains(&"lastline".to_owned()));

    let info = &result.string_table["line:1b"];
    assert!(info.metadata.contains(&"lastline".to_owned()));
}
