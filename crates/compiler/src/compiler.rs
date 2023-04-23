//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Compiler.cs>

pub use crate::compiler::compilation_job::*;
use crate::error_strategy::ErrorStrategy;
use crate::output::*;
use crate::prelude::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::{Diagnostic, FileParseResult, LexerErrorListener, ParserErrorListener};
use crate::string_table_manager::StringTableManager;
use crate::visitors::string_table_generator_visitor::StringTableGeneratorVisitor;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::CodePoint8BitCharStream;
use antlr_rust::token::Token;
use antlr_rust::token_factory::{CommonTokenFactory, TokenFactory};
use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};
use antlr_rust::{InputStream, Parser};
use std::rc::Rc;

mod compilation_job;

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

fn register_strings(job: &CompilationJob, mut previous: CompilationResult) -> CompilationResult {
    let mut parsed_files = Vec::new();

    // First pass: parse all files, generate their syntax trees,
    // and figure out what variables they've declared
    let mut string_table_manager = StringTableManager::default();
    for file in &job.files {
        println!("file: {file:?}");
        let parse_result = parse_syntax_tree(file, &mut previous.diagnostics);

        // ok now we will add in our lastline tags
        // we do this BEFORE we build our strings table otherwise the tags will get missed
        // this should probably be a flag instead of every time though
        // TODO
        // let lastLineTagger = new LastLineBeforeOptionsVisitor();
        //lastLineTagger.Visit(parseResult.Tree);
        let mut visitor =
            StringTableGeneratorVisitor::new(file.file_name.clone(), string_table_manager.clone());
        visitor.visit(&*parse_result.tree);
        println!("visitor: {visitor:?}");
        previous.diagnostics.extend(visitor.diagnostics);
        string_table_manager.extend(visitor.string_table_manager);
        parsed_files.push(parse_result);
    }
    previous
}

fn parse_syntax_tree<'a, 'b>(
    file: &'b File,
    diagnostics: &'a mut Vec<Diagnostic>,
) -> FileParseResult<'b> {
    let input = CodePoint8BitCharStream::new(file.source.as_bytes());
    let mut lexer = YarnSpinnerLexer::new(input);

    // turning off the normal error listener and using ours
    let file_name = file.file_name.clone();
    let lexer_error_listener = LexerErrorListener::new(file_name.clone());
    let lexer_error_listener_diagnostics = lexer_error_listener.diagnostics.clone();
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(lexer_error_listener));

    let tokens = CommonTokenStream::new(lexer);
    let mut parser = YarnSpinnerParser::with_strategy(tokens, ErrorStrategy::new());
    let parser_error_listener = ParserErrorListener::new(file.clone());
    let parser_error_listener_diagnostics = parser_error_listener.diagnostics.clone();

    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(parser_error_listener));

    let lexer_error_listener_diagnostics_borrowed = lexer_error_listener_diagnostics.borrow();
    let parser_error_listener_diagnostics_borrowed = parser_error_listener_diagnostics.borrow();
    let new_diagnostics = lexer_error_listener_diagnostics_borrowed
        .iter()
        .chain(parser_error_listener_diagnostics_borrowed.iter())
        .cloned();
    diagnostics.extend(new_diagnostics);

    FileParseResult::new(file_name, parser)
}

pub(crate) fn get_line_id_for_node_name(name: &str) -> String {
    format!("line:{name}")
}

pub(crate) fn add_hashtag_child<'input>(
    parent: &impl YarnSpinnerParserContext<'input>,
    text: String,
) {
    // Hack: need to convert the reference to an Rc somehow
    let parent = parent.get_children().next().unwrap().get_parent().unwrap();
    // Taken from C# implementation of `CommonToken`s constructor
    let string_id_token = CommonTokenFactory.create::<InputStream<&'input str>>(
        None,
        HASHTAG_TEXT,
        text.into(),
        0,
        0,
        0,
        0,
        -1,
    );
    // `new_with_text` was hacked into the generated parser. Also, `FooContextExt::new` is usually private...
    let hashtag = HashtagContextExt::new_with_text(Some(parent.clone()), 0, string_id_token);
    parent.add_child(hashtag);
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
        panic!("aa");
    }
}
