//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TagTests.cs>

use crate::test_base::*;
use yarn_slinger_compiler::prelude::*;

mod test_base;

#[test]
fn test_no_options_line_not_tagged() {
    let compilation_job =
        CompilationJob::from_test_source("title:Start\n---\nline without options #line:1\n===\n");
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1"];
    assert!(!contains_last_line_tag(&info));
}

#[test]
fn test_line_before_options_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1"];
    assert!(contains_last_line_tag(&info));
}

#[test]
fn test_line_not_before_options_not_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline not before options #line:0\nline before options #line:1\n-> option 1\n-> option 2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(!contains_last_line_tag(&info));
}

#[test]
fn test_line_after_options_not_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "title:Start\n---\nline before options #line:1\n-> option 1\n-> option 2\nline after options #line:2\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:2"];
    assert!(!contains_last_line_tag(&info));
}

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
    assert!(contains_last_line_tag(&info));

    let info = &result.string_table["line:1b"];
    assert!(contains_last_line_tag(&info));
}

#[test]
fn test_if_interior_lines_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "
<<if true>>
line before options #line:0
-> option 1
-> option 2
<<endif>>
            ",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(contains_last_line_tag(&info));
}

#[test]
fn test_if_interior_lines_not_tagged_last_line() {
    let compilation_job = CompilationJob::from_test_source(
        "
<<if true>>
line before options #line:0
<<endif>>
-> option 1
-> option 2
",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(!contains_last_line_tag(&info));
}

#[test]
fn test_nested_option_lines_not_tagged() {
    let compilation_job = CompilationJob::from_test_source(
        "
-> option 1
    inside options #line:1a
-> option 2
-> option 3
",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:1a"];
    assert!(!contains_last_line_tag(&info));
}

#[test]
fn test_interrupted_lines_not_tagged() {
    let compilation_job = CompilationJob::from_test_source(
        "
line before command #line:0
<<custom command>>
-> option 1
line before declare #line:1
<<declare $value = 0>>
-> option 1
line before set #line:2
<<set $value = 0>>
-> option 1
line before jump #line:3
<<jump nodename>>
line before call #line:4
<<call function()>>
            ",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(!contains_last_line_tag(&info));
    let info = &result.string_table["line:1"];
    assert!(!contains_last_line_tag(&info));
    let info = &result.string_table["line:2"];
    assert!(!contains_last_line_tag(&info));
    let info = &result.string_table["line:3"];
    assert!(!contains_last_line_tag(&info));
    let info = &result.string_table["line:4"];
    assert!(!contains_last_line_tag(&info));
}

#[test]
fn test_line_is_last_before_another_node_not_tagged() {
    let compilation_job = CompilationJob::from_test_source(
        "title: Start\n---\nlast line #line:0\n===\ntitle: Second\n---\n-> option 1\n===\n",
    );
    let result = compile(compilation_job).unwrap_pretty();

    let info = &result.string_table["line:0"];
    assert!(!contains_last_line_tag(&info));
}

fn contains_last_line_tag(info: &StringInfo) -> bool {
    info.metadata.contains(&"lastline".to_owned())
}
