//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>

use crate::output::*;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::string_table_manager::StringTableManager;
use crate::visitors::last_line_before_options_visitor::LastLineBeforeOptionsVisitor;
use crate::visitors::string_table_generator_visitor::StringTableGeneratorVisitor;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTreeVisitorCompat;
pub use compilation_job::*;
use std::rc::Rc;
pub(crate) use utils::*;

mod compilation_job;
mod utils;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(compilation_job: CompilationJob) -> CompilationResult {
    // TODO: other steps
    let compiler_steps: Vec<&dyn CompilerStep> = vec![&add_built_in_types, &register_strings];

    let initial = CompilationResult {
        program: None,
        string_table: Default::default(),
        declarations: None,
        contains_implicit_string_tags: false,
        file_tags: Default::default(),
        diagnostics: vec![],
        debug_info: Default::default(),
    };

    compiler_steps
        .into_iter()
        .fold(initial, |acc, curr| curr.apply(&compilation_job, acc))
}

pub(crate) fn get_line_id_tag<'a>(
    hashtag_contexts: &[Rc<HashtagContextAll<'a>>],
) -> Option<Rc<HashtagContextAll<'a>>> {
    hashtag_contexts
        .iter()
        .find(|h| h.text.as_ref().expect("Hashtag held no text").get_text() == "line:")
        .cloned()
}

trait CompilerStep {
    fn apply(&self, job: &CompilationJob, previous: CompilationResult) -> CompilationResult;
}

impl<F> CompilerStep for F
where
    F: Fn(&CompilationJob, CompilationResult) -> CompilationResult,
{
    fn apply(&self, job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
        self(job, previous)
    }
}

fn add_built_in_types(_job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
    previous
}

fn register_strings(job: &CompilationJob, mut state: CompilationResult) -> CompilationResult {
    let mut parsed_files = Vec::new();

    // First pass: parse all files, generate their syntax trees,
    // and figure out what variables they've declared
    let mut string_table_manager: StringTableManager = state.string_table.into();
    for file in &job.files {
        let parse_result = parse_syntax_tree(file, &mut state.diagnostics);

        // ok now we will add in our lastline tags
        // we do this BEFORE we build our strings table otherwise the tags will get missed
        // this should probably be a flag instead of every time though
        let mut last_line_tagger = LastLineBeforeOptionsVisitor::default();
        last_line_tagger.visit(&*parse_result.tree);

        let mut visitor =
            StringTableGeneratorVisitor::new(file.file_name.clone(), string_table_manager.clone());
        visitor.visit(&*parse_result.tree);
        state.diagnostics.extend(visitor.diagnostics);
        string_table_manager.extend(visitor.string_table_manager);
        parsed_files.push(parse_result);
    }
    state.string_table = string_table_manager.into();
    state
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_call_compile_empty_without_crash() {
        compile(CompilationJob {
            files: vec![],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
    }

    #[test]
    fn can_call_compile_file_without_crash() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
foo
bar
a {1 + 3} cool expression
==="
            .to_string(),
        };
        compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
    }

    #[test]
    fn populates_string_table() {
        let file = File {
            file_name: "test.yarn".to_string(),
            source: "title: test
---
foo
bar
a {1 + 3} cool expression
==="
            .to_string(),
        };
        let result = compile(CompilationJob {
            files: vec![file],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
        let string_table = result.string_table;
        assert_eq!(string_table.len(), 3);
        assert_eq!(
            string_table["line:test.yarn-test-0"],
            StringInfo {
                text: "foo".to_string(),
                node_name: "test".to_string(),
                line_number: 3,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
        assert_eq!(
            string_table["line:test.yarn-test-1"],
            StringInfo {
                text: "bar".to_string(),
                node_name: "test".to_string(),
                line_number: 4,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
        assert_eq!(
            string_table["line:test.yarn-test-2"],
            StringInfo {
                text: "a {0} cool expression".to_string(),
                node_name: "test".to_string(),
                line_number: 5,
                file_name: "test.yarn".to_string(),
                is_implicit_tag: true,
                metadata: vec![],
            }
        );
    }
}

#[test]
fn catches_expression_errors() {
    use crate::prelude::*;
    let file = File {
        file_name: "test.yarn".to_string(),
        source: "title: test
---
foo
bar
a {very} cool expression
==="
        .to_string(),
    };
    let result = compile(CompilationJob {
        files: vec![file],
        library: None,
        compilation_type: CompilationType::FullCompilation,
        variable_declarations: vec![],
    });
    assert!(result.program.is_none());
    let diagnostics = result.diagnostics;
    assert_eq!(diagnostics.len(), 2);

    // TODO: Imo this is off by one, but I'm not sure if this is a bug in the original impl
    // or if there is a (+1) that will be done at some point that we have not implemented yet.
    let range = Position {
        line: 4,
        character: 7,
    }..=Position {
        line: 4,
        character: 8,
    };
    let context = "a {very} cool expression\n       ^".to_owned();
    let first_expected =
        Diagnostic::from_message("Unexpected \"}\" while reading a function call".to_string())
            .with_file_name("test.yarn".to_string())
            .with_range(range.clone())
            .with_context(context.clone())
            .with_severity(DiagnosticSeverity::Error);

    let second_expected =
        Diagnostic::from_message("mismatched input '}' expecting '('".to_string())
            .with_file_name("test.yarn".to_string())
            .with_range(range)
            .with_context(context)
            .with_severity(DiagnosticSeverity::Error);
    if diagnostics[0] == first_expected {
        assert_eq!(diagnostics[1], second_expected);
    } else {
        assert_eq!(diagnostics[0], second_expected);
        assert_eq!(diagnostics[1], first_expected);
    }
}
