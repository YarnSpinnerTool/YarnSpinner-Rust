//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>

pub(crate) use self::{antlr_rust_ext::*, utils::*};
use crate::output::*;
use crate::parser::generated::yarnspinnerparser::HashtagContextAll;
use crate::prelude::FileParseResult;
use crate::string_table_manager::StringTableManager;
use crate::visitors::*;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::{ParseTreeVisitorCompat, ParseTreeWalker};
pub use compilation_job::*;
use rusty_yarn_spinner_core::types::*;
use std::collections::HashMap;
use std::rc::Rc;

mod antlr_rust_ext;
mod compilation_job;
mod utils;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(compilation_job: CompilationJob) -> CompilationResult {
    // TODO: other steps
    let compiler_steps: Vec<&dyn CompilerStep> = vec![&register_strings, &get_declarations];

    let initial = CompilationIntermediate::default();
    let result = compiler_steps
        .into_iter()
        .fold(initial, |acc, curr| curr.apply(&compilation_job, acc))
        .result;

    result
}

trait CompilerStep<'a> {
    fn apply(
        &self,
        job: &'a CompilationJob,
        previous: CompilationIntermediate<'a>,
    ) -> CompilationIntermediate<'a>;
}

impl<'a, F> CompilerStep<'a> for F
where
    F: Fn(&'a CompilationJob, CompilationIntermediate<'a>) -> CompilationIntermediate<'a>,
{
    fn apply(
        &self,
        job: &'a CompilationJob,
        previous: CompilationIntermediate<'a>,
    ) -> CompilationIntermediate<'a> {
        self(job, previous)
    }
}

fn get_declarations<'a>(
    _job: &'a CompilationJob,
    mut state: CompilationIntermediate<'a>,
) -> CompilationIntermediate<'a> {
    // Find the variable declarations in these files.
    for file in &state.parsed_files {
        /*
        let mut variable_declaration_visitor = DeclarationVisitor::new(
            file.file_name.clone(),
            job.variable_declarations.clone(),
            in_yarn_explicitly_constructable_types(),
            file.tree.get_tokens());

        var variableDeclarationVisitor = new DeclarationVisitor(parsedFile.Name, existingDeclarations, typeDeclarations, parsedFile.Tokens);

            var newDiagnosticList = new List<Diagnostic>();

            variableDeclarationVisitor.Visit(parsedFile.Tree);

            newDiagnosticList.AddRange(variableDeclarationVisitor.Diagnostics);

            // Upon exit, newDeclarations will now contain every variable
            // declaration we found
            newDeclarations = variableDeclarationVisitor.NewDeclarations;

            fileTags = variableDeclarationVisitor.FileTags;

            diagnostics = newDiagnosticList;

                knownVariableDeclarations.AddRange(newDeclarations);
                derivedVariableDeclarations.AddRange(newDeclarations);
                diagnostics.AddRange(declarationDiagnostics);

                fileTags.Add(parsedFile.Name, newFileTags);
            }
            */
    }
    state
}

fn in_yarn_explicitly_constructable_types() -> Vec<BuiltinType> {
    vec![
        BuiltinType::Any(AnyType),
        BuiltinType::Number(NumberType),
        BuiltinType::String(StringType),
        BuiltinType::Boolean(BooleanType),
        // Undefined types are not explicitly constructable
    ]
}

fn register_strings<'a>(
    job: &'a CompilationJob,
    mut state: CompilationIntermediate<'a>,
) -> CompilationIntermediate<'a> {
    // First pass: parse all files, generate their syntax trees,
    // and figure out what variables they've declared
    let mut string_table_manager: StringTableManager = state.result.string_table.into();
    for file in &job.files {
        let parse_result = parse_syntax_tree(file, &mut state.result.diagnostics);

        // ok now we will add in our lastline tags
        // we do this BEFORE we build our strings table otherwise the tags will get missed
        // this should probably be a flag instead of every time though
        let mut last_line_tagger = LastLineBeforeOptionsVisitor::default();
        last_line_tagger.visit(&*parse_result.tree);

        let mut visitor =
            StringTableGeneratorVisitor::new(file.file_name.clone(), string_table_manager.clone());
        visitor.visit(&*parse_result.tree);
        state.result.diagnostics.extend(visitor.diagnostics);
        string_table_manager.extend(visitor.string_table_manager);
        state.parsed_files.push(parse_result);
    }
    state.result.string_table = string_table_manager.into();
    state
}

#[derive(Default)]
struct CompilationIntermediate<'input> {
    pub(crate) result: CompilationResult,
    pub(crate) parsed_files: Vec<FileParseResult<'input>>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::*;

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

    #[test]
    fn catches_expression_errors() {
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
}
