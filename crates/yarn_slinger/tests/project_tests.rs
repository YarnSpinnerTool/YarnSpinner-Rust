//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/ProjectTests.cs>
//!
//! The following tests test behavior that is currently out of scope for this project and were thus omitted:
//! - TestDeclarationFilesAreGenerated: Tests functionality that, quote "Is intended to be called by tools that let the user manage variable declarations."

use regex::Regex;
use std::collections::HashSet;
use test_base::prelude::*;
use yarn_slinger::compiler::*;
use yarn_slinger_core::prelude::*;

mod test_base;

#[test]
fn test_loading_nodes() {
    let path = test_data_path().join("Projects/Basic/Test.yarn");
    let result = Compiler::new().read_file(path).compile().unwrap();

    let dialogue = TestBase::default().with_compilation(result).dialogue;

    // high-level test: load the file, verify it has the nodes we want,
    // and run one

    assert_eq!(3, dialogue.node_names().unwrap().count());
    assert!(dialogue.node_exists("TestNode"));
    assert!(dialogue.node_exists("AnotherTestNode"));
    assert!(dialogue.node_exists("ThirdNode"));
}

#[test]
fn test_line_tags_are_added() {
    // Arrange
    let original_text = "title: Program
---
// A comment. No line tag is added.
A single line, with no line tag.
A single line, with a line tag. #line:expected_abc123

-> An option, with no line tag.
-> An option, with a line tag. #line:expected_def456

A line with no tag, but a comment at the end. // a comment
A line with a tag, and a comment. #line:expected_ghi789 // a comment

A line with a conditional and no line tag. <<if false>>
A line with a conditional, a comment, and no line tag. <<if false>> // a comment

A line with a conditional and a line tag. <<if false>> #line:expected_jkl123
A line with a conditional, a comment and a line tag. <<if false>>  #line:expected_mno456 // a comment

-> An option with a conditional and no line tag. <<if false>>
-> An option with a conditional, a comment, and no line tag. <<if false>> // a comment
-> An option with a conditional and a line tag.  <<if false>> #line:expected_pqr789
-> An option with a conditional, a comment and a line tag.  <<if false>> #line:expected_stu123 // a comment

// A comment with no text:
//
// A comment with a single space:
//

// single symbol tests
🧑🏾‍❤️‍💋‍🧑🏻
🧑🏾‍❤️‍💋‍🧑🏻 // with comment
🧑🏾‍❤️‍💋‍🧑🏻#line:abc122
🧑🏾‍❤️‍💋‍🧑🏻 #line:abc124 // with a comment

// after emoji tests
🧑🏾‍❤️‍💋‍🧑🏻 text after emoji
🧑🏾‍❤️‍💋‍🧑🏻 text after emoji // with a comment
🧑🏾‍❤️‍💋‍🧑🏻 text after emoji #line:abc125
🧑🏾‍❤️‍💋‍🧑🏻 text after emoji #line:abc126 // with a comment

// before emoji tests
text before emoji 🧑🏾‍❤️‍💋‍🧑🏻
text before emoji 🧑🏾‍❤️‍💋‍🧑🏻 // with a comment
text before emoji 🧑🏾‍❤️‍💋‍🧑🏻 #line:abc127
text before emoji 🧑🏾‍❤️‍💋‍🧑🏻 #line:abc128 // with a comment

// emoji between tests
before 🧑🏾‍❤️‍💋‍🧑🏻after
before 🧑🏾‍❤️‍💋‍🧑🏻after #line:abc129
before 🧑🏾‍❤️‍💋‍🧑🏻after // with a comment
before 🧑🏾‍❤️‍💋‍🧑🏻after #line:abc130 // with a comment

// multi-moji tests
🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻
🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻 // with a comment
🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻 #line:abc131
🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻 #line:abc132 // with a comment

// testing command structures to make sure the tagger hasn't botched the whitespace
<<declare $a = 0>>
<<set $a to 5>>
<<if $a == 5>>
<<generic command goes here>>
<<endif>>
===";
    {
        let file = File {
            file_name: "input".to_string(),
            source: original_text.to_string(),
        };
        // This original input should compile without errors.
        Compiler::new()
            .add_file(file)
            .with_compilation_type(CompilationType::StringsOnly)
            .compile()
            .unwrap();
    }

    // Act

    let output = Compiler::add_tags_to_lines(original_text, Vec::new())
        .unwrap()
        .unwrap();

    let file = File {
        file_name: "input".to_string(),
        source: output.clone(),
    };
    let compilation = Compiler::new()
        .add_file(file)
        .with_compilation_type(CompilationType::StringsOnly)
        .compile()
        .unwrap();

    // Assert
    let line_tag_regex = Regex::new(r"#line:\w+").unwrap();
    let line_tag_after_comment = Regex::new(r"//.*#line:\w+").unwrap();

    // Ensure that the right number of tags in total is present
    let expected_existing_tags = 17;
    let expected_new_tags = 17;
    let expected_total_tags = expected_existing_tags + expected_new_tags;

    let line_tag_regex_matches = line_tag_regex.captures_iter(&output).count();
    assert_eq!(line_tag_regex_matches, expected_total_tags);

    // No tags were added after a comment
    for line in output.lines() {
        assert!(
            !line_tag_after_comment.is_match(line),
            "'{line}' should not contain a tag after a comment"
        );
    }

    let expected_results = [
        (
            Some("line:expected_abc123"),
            "A single line, with a line tag.",
        ),
        ("line:expected_def456".into(), "An option, with a line tag."),
        (
            "line:expected_ghi789".into(),
            "A line with a tag, and a comment.",
        ),
        (None, "A line with a conditional and no line tag."),
        (
            None,
            "A line with a conditional, a comment, and no line tag.",
        ),
        (
            "line:expected_jkl123".into(),
            "A line with a conditional and a line tag.",
        ),
        (
            "line:expected_mno456".into(),
            "A line with a conditional, a comment and a line tag.",
        ),
        (None, "An option with a conditional and no line tag."),
        (
            None,
            "An option with a conditional, a comment, and no line tag.",
        ),
        (
            "line:expected_pqr789".into(),
            "An option with a conditional and a line tag.",
        ),
        (
            "line:expected_stu123".into(),
            "An option with a conditional, a comment and a line tag.",
        ),
        (None, "A single line, with no line tag."),
        (None, "An option, with no line tag."),
        (None, "A line with no tag, but a comment at the end."),
        // single symbol tests
        (None, "🧑🏾‍❤️‍💋‍🧑🏻"),
        (None, "🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc122".into(), "🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc124".into(), "🧑🏾‍❤️‍💋‍🧑🏻"),
        // // after emoji tests
        (None, "🧑🏾‍❤️‍💋‍🧑🏻 text after emoji"),
        (None, "🧑🏾‍❤️‍💋‍🧑🏻 text after emoji"),
        ("line:abc125".into(), "🧑🏾‍❤️‍💋‍🧑🏻 text after emoji"),
        ("line:abc126".into(), "🧑🏾‍❤️‍💋‍🧑🏻 text after emoji"),
        // // before emoji tests
        (None, "text before emoji 🧑🏾‍❤️‍💋‍🧑🏻"),
        (None, "text before emoji 🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc127".into(), "text before emoji 🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc128".into(), "text before emoji 🧑🏾‍❤️‍💋‍🧑🏻"),
        // // emoji between tests
        (None, "before 🧑🏾‍❤️‍💋‍🧑🏻after"),
        ("line:abc129".into(), "before 🧑🏾‍❤️‍💋‍🧑🏻after"),
        (None, "before 🧑🏾‍❤️‍💋‍🧑🏻after"),
        ("line:abc130".into(), "before 🧑🏾‍❤️‍💋‍🧑🏻after"),
        // // multi-moji tests
        (None, "🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻"),
        (None, "🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc131".into(), "🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻"),
        ("line:abc132".into(), "🧑🏾‍❤️‍💋‍🧑🏻🧑🏾‍❤️‍💋‍🧑🏻"),
    ];

    assert_eq!(line_tag_regex_matches, expected_results.len());

    // used to keep track of all line ids we have already seen
    // this is because we need to make sure we see every line in the string table
    let mut visited_ids = HashSet::new();

    for (tag, line) in expected_results
        .iter()
        .map(|(tag, line)| (tag.map(|s| LineId(s.to_string())), line.to_string()))
    {
        println!("checking tag: {:#?} line: {:#?}", tag, line);
        if let Some(tag) = tag {
            assert_eq!(line, compilation.string_table.get(&tag).unwrap().text);
            // flagging this ID as having been visited
            let prev = visited_ids.insert(tag.clone());
            if !prev {
                println!(
                    "{:#?}",
                    compilation
                        .string_table
                        .iter()
                        .filter(|(k, _)| **k == tag)
                        .collect::<Vec<_>>()
                );
                panic!("Duplicate line tag: {}", tag);
            }
        } else {
            // Implementation note: this branch looks different from the original because the C# version depends on the order of the string table, which is not guaranteed.

            // a line exists that has this text
            let matching_entries = compilation
                .string_table
                .iter()
                .filter(|(_, v)| v.text == line)
                .filter(|(k, _)| !visited_ids.contains(k))
                .filter(|(k, _)| expected_results.iter().all(|(t, _)| *t != Some(&k.0)))
                .collect::<Vec<_>>();

            // that line has a line tag
            for (line_tag, _) in matching_entries {
                assert!(line_tag.0.starts_with("line:"));

                // that line is not a duplicate of any other line tag
                let all_line_tags = compilation.string_table.keys();
                assert_eq!(all_line_tags.filter(|t| t.0 == line_tag.0).count(), 1);

                // flagging this ID as having been visited
                visited_ids.insert(line_tag.clone());
            }
        }
    }

    // we have seen every line in the string table
    let in_string_table_but_not_visited = compilation
        .string_table
        .iter()
        .filter(|(k, _)| !visited_ids.contains(k))
        .collect::<Vec<_>>();
    println!("{:#?}", in_string_table_but_not_visited);
    assert_eq!(visited_ids.len(), compilation.string_table.len());
}

#[test]
fn test_debug_output_is_produced() {
    let file = File {
        file_name: "input".to_owned(),
        source: create_test_node_with_name("This is a test node.", "DebugTesting"),
    };
    let result = Compiler::new().add_file(file).compile().unwrap();

    // We should have a single DebugInfo object, because we compiled a single node
    assert_eq!(1, result.debug_info.len());

    // The first instruction of the only node should begin on the third line
    println!("{:?}", result.debug_info);
    let first_line_info = result.debug_info.values().next().unwrap().get_line_info(0);

    assert_eq!("input", first_line_info.file_name);
    assert_eq!("DebugTesting", first_line_info.node_name);
    assert_eq!(2, first_line_info.position.unwrap().line);
    assert_eq!(0, first_line_info.position.unwrap().character);
}
