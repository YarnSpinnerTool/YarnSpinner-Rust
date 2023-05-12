//! Contains functions that were originally part of `compiler.rs` according to the original implementation,
//! but were moved to their own file for better organization.

use crate::error_strategy::ErrorStrategy;
use crate::listeners::*;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::prelude::generated::{yarnspinnerlexer, yarnspinnerparser};
use crate::prelude::*;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::CodePoint8BitCharStream;
use antlr_rust::token::{Token, TOKEN_DEFAULT_CHANNEL};
use antlr_rust::Parser;
use std::collections::HashSet;
use std::rc::Rc;
use yarn_slinger_core::prelude::*;
use yarn_slinger_core::types::FunctionType;

pub(crate) fn get_line_id_tag<'a>(
    hashtag_contexts: &[Rc<HashtagContextAll<'a>>],
) -> Option<Rc<HashtagContextAll<'a>>> {
    hashtag_contexts
        .iter()
        .find(|hashtag| {
            let hashtag_text = hashtag
                .text
                .as_ref()
                .expect("Hashtag held no text")
                .get_text();
            hashtag_text.starts_with("line:")
        })
        .cloned()
}

pub(crate) fn parse_syntax_tree<'a>(
    file: &'a File,
    diagnostics: &mut Vec<Diagnostic>,
) -> FileParseResult<'a> {
    let input = CodePoint8BitCharStream::new(file.source.as_bytes());
    let mut lexer = YarnSpinnerLexer::new(input, file.file_name.clone());

    // turning off the normal error listener and using ours
    let file_name = file.file_name.clone();
    let lexer_error_listener = LexerErrorListener::new(file_name.clone());
    let lexer_error_listener_diagnostics = lexer_error_listener.diagnostics.clone();
    let lexer_diagnostics = lexer.diagnostics.clone();
    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(lexer_error_listener));

    let tokens = CommonTokenStream::new(lexer);
    let mut parser = YarnSpinnerParser::with_strategy(tokens, ErrorStrategy::new());
    let parser_error_listener = ParserErrorListener::new(file.clone());
    let parser_error_listener_diagnostics = parser_error_listener.diagnostics.clone();

    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(parser_error_listener));

    // Must be read exactly here, because the error listeners running during the parse borrow the diagnostics mutably,
    // and we want to read them after.
    let tree = parser.dialogue().unwrap();

    let lexer_diagnostics_borrowed = lexer_diagnostics.borrow();
    let lexer_error_listener_diagnostics_borrowed = lexer_error_listener_diagnostics.borrow();
    let parser_error_listener_diagnostics_borrowed = parser_error_listener_diagnostics.borrow();
    let new_diagnostics = lexer_error_listener_diagnostics_borrowed
        .iter()
        .chain(lexer_diagnostics_borrowed.iter())
        .chain(parser_error_listener_diagnostics_borrowed.iter())
        .cloned();
    diagnostics.extend(new_diagnostics);

    FileParseResult::new(file_name, tree, Rc::new(parser))
}

pub(crate) fn get_line_id_for_node_name(name: &str) -> String {
    format!("line:{name}")
}

/// Gets the text of the documentation comments that either immediately
/// precede `context`, or are on the same line as `context`.
///
/// Documentation comments begin with a triple-slash (`///`), and
/// are used to describe variable declarations. If documentation
/// comments precede a declaration (that is, they're not on the same
/// line as the declaration), then they may span multiple lines, as long
/// as each line begins with a triple-slash.
///
/// If there are both doc comments preceding the declaration and on the same line,
/// only the the latter will be returned.
///
/// ## Implementation notes
///
/// The flag `allowCommentsAfter` was not ported because it was always set to `true` anyway.
pub(crate) fn get_document_comments<'input>(
    tokens: &ActualTokenStream<'input>,
    context: &impl YarnSpinnerParserContext<
        'input,
        TF = LocalTokenFactory<'input>,
        Ctx = YarnSpinnerParserContextType,
    >,
) -> String {
    let subsequent_comments = tokens.get_hidden_tokens_to_right(
        context.stop().get_token_index(),
        yarnspinnerlexer::COMMENTS as isize,
    );

    let subsequent_doc_comment = subsequent_comments
        .iter()
        // This comment is on the same line as the end of
        // the declaration
        .filter(|t| t.get_line() == context.stop().get_line())
        // The comment starts with a triple-slash
        .filter(|t| t.get_text().starts_with("///"))
        // Get its text
        .map(|t| t.get_text().replace("///", "").trim().to_owned())
        // Get the first one, or null
        .next();

    if let Some(subsequent_doc_comment) = subsequent_doc_comment {
        return subsequent_doc_comment;
    }

    let preceding_comments = tokens.get_hidden_tokens_to_left(
        context.start().get_token_index(),
        yarnspinnerlexer::COMMENTS as isize,
    );

    let preceding_doc_comments: Vec<_> = preceding_comments
        .iter()
        // There are no tokens on the main channel with this
        // one on the same line
        .filter(|t| {
            !tokens
                .get_tokens()
                .iter()
                .filter(|ot| ot.get_line() == t.get_line())
                .filter(|ot| {
                    ot.get_token_type() != yarnspinnerlexer::INDENT
                        && ot.get_token_type() != yarnspinnerlexer::DEDENT
                })
                .any(|ot| ot.get_channel() == TOKEN_DEFAULT_CHANNEL)
        })
        .filter(|t| t.get_text().starts_with("///"))
        // Get its text
        .map(|t| t.get_text().replace("///", "").trim().to_owned())
        .collect();
    preceding_doc_comments.join(" ")
}

/// Not part of original implementation, but needed because we lack some convenience methods
/// that the C# implementation of ANTLR would provide but antlr4rust does not.
pub(crate) fn add_hashtag_child<'input>(
    parent: &impl YarnSpinnerParserContext<'input>,
    text: impl Into<String>,
) {
    let parent = parent.ref_to_rc();
    let string_id_token = create_common_token(yarnspinnerparser::HASHTAG_TEXT, text);
    let invoking_state_according_to_original_implementation = 0;
    // `new_with_text` was hacked into the generated parser. Also, `FooContextExt::new` is usually private...
    let hashtag = HashtagContextExt::new_with_text(
        Some(parent.clone()),
        invoking_state_according_to_original_implementation,
        string_id_token,
    );
    parent.add_child(hashtag);
}

pub(crate) trait ContextRefExt<'input> {
    fn ref_to_rc(self) -> Rc<ActualParserContext<'input>>;
}

impl<'input, T> ContextRefExt<'input> for &T
where
    T: YarnSpinnerParserContext<'input> + ?Sized,
{
    fn ref_to_rc(self) -> Rc<ActualParserContext<'input>> {
        self.get_children()
            .next()
            .map(|child| child.get_parent().unwrap())
            .or_else(|| {
                let interval = self.get_source_interval();
                self.get_parent()
                    .unwrap()
                    .get_children()
                    .find(|child| child.get_source_interval() == interval)
            })
            .unwrap()
    }
}

/// Returns a collection of [`Declaration`] structs that
/// describe the functions present in `library`.
///
/// ## Implementation note
///
/// In contrast to the original implementation, we don't return any diagnostics
/// because Rust's type system already guarantees at compile-time that all registered
/// functions are valid and compatible with Yarn.
pub(crate) fn get_declarations_from_library(library: &Library) -> Vec<Declaration> {
    let operators: HashSet<_> = Type::EXPLICITLY_CONSTRUCTABLE
        .iter()
        .flat_map(|r#type| {
            r#type
                .methods()
                .keys()
                .map(|name| r#type.get_canonical_name_for_method(name))
                .collect::<Vec<_>>()
        })
        .collect();
    library
        .iter()
        // Operators are type checked by visitors instead
        .filter(|(name, _function)| !operators.contains(*name))
        .map(|(name, function)| {
            let mut function_type = FunctionType::default();
            let parameters = function
                .parameter_types()
                .into_iter()
                .map(|t| Type::try_from(t).unwrap())
                .map(Some)
                .collect();
            function_type.parameters = parameters;
            let return_type = Type::try_from(function.return_type()).unwrap();
            function_type.set_return_type(return_type);
            Declaration::new(name, function_type).with_source_file_name(DeclarationSource::External)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn warns_about_mixed_indentation() {
        let mut diagnostics = Vec::new();
        let mixed_indentation_input = File {
            file_name: "test.yarn".to_owned(),
            source: "title: Start
---
-> Option 1
\t   Nice.
==="
            .to_owned(),
        };
        let _parsed_file = parse_syntax_tree(&mixed_indentation_input, &mut diagnostics);
        assert_eq!(1, diagnostics.len());
        assert_eq!(
            Diagnostic::from_message("Indentation contains tabs and spaces")
                .with_context("\t   ")
                .with_start_line(3)
                .with_file_name("test.yarn")
                .with_range(
                    Position {
                        line: 3,
                        character: 0
                    }..Position {
                        line: 3,
                        character: 5
                    }
                )
                .with_severity(DiagnosticSeverity::Warning),
            diagnostics[0]
        );
    }
}
