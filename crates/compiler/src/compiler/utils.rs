use crate::error_strategy::ErrorStrategy;
use crate::prelude::generated::yarnspinnerlexer::YarnSpinnerLexer;
use crate::prelude::generated::yarnspinnerparser;
use crate::prelude::generated::yarnspinnerparser::{
    HashtagContextExt, YarnSpinnerParser, YarnSpinnerParserContext,
};
use crate::prelude::{Diagnostic, File, FileParseResult, LexerErrorListener, ParserErrorListener};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::CodePoint8BitCharStream;
use antlr_rust::token_factory::{CommonTokenFactory, TokenFactory};
use antlr_rust::{InputStream, Parser};

pub(crate) fn parse_syntax_tree<'a>(
    file: &'a File,
    diagnostics: &mut Vec<Diagnostic>,
) -> FileParseResult<'a> {
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

    // Must be read exactly here, because the error listeners running during the parse borrow the diagnostics mutably,
    // and we want to read them after.
    let tree = parser.dialogue().unwrap();

    let lexer_error_listener_diagnostics_borrowed = lexer_error_listener_diagnostics.borrow();
    let parser_error_listener_diagnostics_borrowed = parser_error_listener_diagnostics.borrow();
    let new_diagnostics = lexer_error_listener_diagnostics_borrowed
        .iter()
        .chain(parser_error_listener_diagnostics_borrowed.iter())
        .cloned();
    diagnostics.extend(new_diagnostics);

    FileParseResult::new(file_name, tree, parser)
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
        yarnspinnerparser::HASHTAG_TEXT,
        text.into(),
        0,
        0,
        0,
        0,
        -1,
    );
    let invoking_state_according_to_original_implementation = 0;
    // `new_with_text` was hacked into the generated parser. Also, `FooContextExt::new` is usually private...
    let hashtag = HashtagContextExt::new_with_text(
        Some(parent.clone()),
        invoking_state_according_to_original_implementation,
        string_id_token,
    );
    parent.add_child(hashtag);
}
