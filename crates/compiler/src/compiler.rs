//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>

pub(crate) use self::{antlr_rust_ext::*, utils::*};
use crate::output::*;
use crate::prelude::FileParseResult;
use crate::string_table_manager::StringTableManager;
use crate::visitors::*;
use antlr_rust::tree::ParseTreeVisitorCompat;
pub use compilation_job::*;
use rusty_yarn_spinner_core::types::*;

mod antlr_rust_ext;
mod compilation_job;
mod utils;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(compilation_job: CompilationJob) -> CompilationResult {
    // TODO: other steps
    let compiler_steps: Vec<&dyn CompilerStep> = vec![&register_strings, &get_declarations];

    let initial = CompilationIntermediate::default();
    compiler_steps
        .into_iter()
        .fold(initial, |acc, curr| curr.apply(&compilation_job, acc))
        .result
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
    job: &'a CompilationJob,
    mut state: CompilationIntermediate<'a>,
) -> CompilationIntermediate<'a> {
    // Find the variable declarations in these files.
    for file in &state.parsed_files {
        let mut variable_declaration_visitor = DeclarationVisitor::new(
            file.name.clone(),
            job.variable_declarations.clone(),
            in_yarn_explicitly_constructable_types(),
            file.tokens(),
        );

        variable_declaration_visitor.visit(&*file.tree);
        let result = &mut state.result;
        result
            .diagnostics
            .extend_from_slice(&variable_declaration_visitor.diagnostics);
        result
            .declarations
            .extend(variable_declaration_visitor.new_declarations);
        result
            .file_tags
            .insert(file.name.clone(), variable_declaration_visitor.file_tags);
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

        let mut visitor = StringTableGeneratorVisitor::new(
            file.file_name.clone(),
            string_table_manager.clone(),
            parse_result.tokens(),
        );
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
}
