//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/IndentAwareLexer.cs>
//!
//! The C# implementation uses inheritance to do this.
//! More specifically, the lexer generated by ANTLR derives from the `IndentAwareLexer`
//! directly, and the `IndentAwareLexer` derives from the ANTLR Lexer base class.
//! Instead of this, we use a proxy/wrapper around the generated lexer to handle everything correctly.

use super::generated::yarnspinnerlexer::{
    self, LocalTokenFactory, YarnSpinnerLexer as GeneratedYarnSpinnerLexer,
};
use crate::listeners::Diagnostic;
use crate::prelude::{create_common_token, DiagnosticSeverity, TokenExt};
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::rule_context::CustomRuleContext;
use antlr_rust::token::CommonToken;
use antlr_rust::{
    char_stream::CharStream,
    token::{Token, TOKEN_DEFAULT_CHANNEL},
    token_factory::{CommonTokenFactory, TokenFactory},
    InputStream, Lexer, TokenSource,
};
use std::cell::RefCell;
use std::ops::{Deref, DerefMut, Range};
use std::rc::Rc;
use yarn_slinger_core::collections::*;
use yarn_slinger_core::prelude::*;

// To ensure we don't accidentally use the wrong lexer, this will produce errors on use.
#[allow(dead_code)]
type YarnSpinnerLexer = ();

antlr_rust::tid! { impl<'input, Input> TidAble<'input> for IndentAwareYarnSpinnerLexer<'input, Input> where Input:CharStream<From<'input>> }

/// A Lexer subclass that detects newlines and generates indent and dedent tokens accordingly.
///
/// ## Implementation notes
///
/// In contrast to the original implementation, the warnings emitted by this lexer are actually respected in the diagnostics.
pub(crate) struct IndentAwareYarnSpinnerLexer<
    'input,
    Input: CharStream<From<'input>>,
    TF: TokenFactory<'input> = LocalTokenFactory<'input>,
> {
    raw_input: &'input str,
    base: GeneratedYarnSpinnerLexer<'input, Input>,
    lookahead_lexer: GeneratedYarnSpinnerLexer<'input, Input>,
    hit_eof: bool,
    /// Holds the last observed token from the stream.
    /// Used to see if a line is blank or not.
    last_token: Option<TF::Tok>,
    /// The collection of tokens that we have seen, but have not yet
    /// returned. This is needed when NextToken encounters a newline,
    /// which means we need to buffer indents or dedents. [`next_token`]
    /// only returns a single [`Token`] at a time, which
    /// means we use this list to buffer it.
    pending_tokens: Queue<TF::Tok>,
    /// A flag to say the last line observed was a shortcut or not.
    /// Used to determine if tracking indents needs to occur.
    line_contains_shortcut: bool,
    /// Keeps track of the last indentation encountered.
    /// This is used to see if depth has changed between lines.
    last_indent: isize,
    /// A stack keeping track of the levels of indentations we have seen so far that are relevant to shortcuts.
    unbalanced_indents: Stack<isize>,
    /// holds the line number of the last seen option.
    /// Lets us work out if the blank line needs to end the option.
    last_seen_option_content: Option<isize>,
    file_name: String,
    pub(crate) diagnostics: Rc<RefCell<Vec<Diagnostic>>>,
}

impl<'input, Input: CharStream<From<'input>>> Deref for IndentAwareYarnSpinnerLexer<'input, Input> {
    type Target = GeneratedYarnSpinnerLexer<'input, Input>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> DerefMut
    for IndentAwareYarnSpinnerLexer<'input, Input>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl<'input, Input: CharStream<From<'input>>> TokenSource<'input>
    for IndentAwareYarnSpinnerLexer<'input, Input>
{
    type TF = LocalTokenFactory<'input>;

    fn next_token(&mut self) -> <Self::TF as TokenFactory<'input>>::Tok {
        if self.hit_eof && !self.pending_tokens.0.is_empty() {
            // We have hit the EOF, but we have tokens still pending.
            // Start returning those tokens.
            self.pending_tokens.dequeue().unwrap()
        } else if self.base.input().size() == 0 {
            self.hit_eof = true;
            create_common_token(antlr_rust::token::TOKEN_EOF, "<EOF>")
        } else {
            // Get the next token, which will enqueue one or more new
            // tokens into the pending tokens queue.
            self.check_next_token();

            // `check_next_token` will always set at least one pending token if `self.base.input().size() > 0`
            // if `self.base.input().size() == 0`, the branch returning the EOF token is already entered ahead of this.
            self.pending_tokens.dequeue().unwrap()
        }
    }

    fn get_input_stream(&mut self) -> Option<&mut dyn antlr_rust::int_stream::IntStream> {
        self.base.get_input_stream()
    }

    fn get_source_name(&self) -> String {
        self.base.get_source_name()
    }

    fn get_token_factory(&self) -> &'input Self::TF {
        self.base.get_token_factory()
    }
}

/// Copied from generated/yarnspinnerlexer.rs
type From<'a> = <LocalTokenFactory<'a> as TokenFactory<'a>>::From;

impl<'input, Input: CharStream<From<'input>>> IndentAwareYarnSpinnerLexer<'input, Input>
where
    &'input LocalTokenFactory<'input>: Default,
{
    pub fn new(
        input: Input,
        lookahead_input: Input,
        raw_input: &'input str,
        file_name: String,
    ) -> Self {
        let mut lookahead_lexer = GeneratedYarnSpinnerLexer::new(lookahead_input);
        lookahead_lexer.next_token();
        IndentAwareYarnSpinnerLexer {
            file_name,
            raw_input,
            base: GeneratedYarnSpinnerLexer::new(input),
            lookahead_lexer,
            hit_eof: false,
            last_token: Default::default(),
            pending_tokens: Default::default(),
            line_contains_shortcut: false,
            last_indent: Default::default(),
            unbalanced_indents: Default::default(),
            last_seen_option_content: None,
            diagnostics: Default::default(),
        }
    }

    fn check_next_token(&mut self) {
        let mut current = self.base.next_token();
        let mut next = self.next_lookahead_token();
        current.text = self.fix_non_ascii_text(&current, &next).into();

        match current.token_type {
            // Insert indents or dedents depending on the next token's
            // indentation, and enqueues the newline at the correct place
            yarnspinnerlexer::NEWLINE => self.handle_newline_token(current.clone()),
            // Insert dedents before the end of the file, and then
            // enqueues the EOF.
            antlr_rust::token::TOKEN_EOF => self.handle_eof_token(current.clone()),
            yarnspinnerlexer::SHORTCUT_ARROW => {
                self.pending_tokens.enqueue(current.clone());
                self.line_contains_shortcut = true;
            }
            // we are at the end of the node
            // depth no longer matters
            // clear the stack
            yarnspinnerlexer::BODY_END => {
                self.line_contains_shortcut = false;
                self.last_indent = 0;
                self.unbalanced_indents.0.clear();
                self.last_seen_option_content = None;
                // [sic from the original!] TODO: this should be empty by now actually...
                self.pending_tokens.enqueue(current.clone());
            }
            // ## Implementation note
            // This is a massive hack because antlr4rust splits non-ascii VAR_IDs into one VAR_ID and multiple FUNC_IDs for some reason...
            yarnspinnerlexer::VAR_ID => {
                let mut cumulative_var_id_token = current.clone();
                loop {
                    if let Some(inner_next) = &next {
                        if inner_next.token_type == yarnspinnerlexer::FUNC_ID {
                            cumulative_var_id_token.stop = inner_next.stop;
                            next = self.next_lookahead_token();
                            self.base.next_token();
                            continue;
                        }
                    }
                    break;
                }

                cumulative_var_id_token.text = self
                    .fix_non_ascii_text(&cumulative_var_id_token, &next)
                    .into();
                self.pending_tokens.enqueue(cumulative_var_id_token.clone());
                current = cumulative_var_id_token;
            }
            _ => self.pending_tokens.enqueue(current.clone()),
        }

        // TODO: but... really?
        self.last_token = Some(current);
    }

    fn next_lookahead_token(&mut self) -> Option<Box<CommonToken<'input>>> {
        (self.lookahead_lexer.input().size() > 0).then(|| self.lookahead_lexer.next_token())
    }

    fn handle_newline_token(
        &mut self,
        current_token: Box<antlr_rust::token::GenericToken<std::borrow::Cow<'input, str>>>,
    ) {
        // We're about to go to a new line. Look ahead to see how indented it is.

        // insert the current NEWLINE token
        self.pending_tokens.enqueue(current_token.clone());

        if let Some(last_seen_option_content) = self.last_seen_option_content {
            // [sic!] we are a blank line
            if self
                .last_token
                .as_ref()
                .map(|last| current_token.token_type == last.token_type)
                .unwrap_or_default()
            {
                // is the option content directly above us?
                if self.base.get_line() - last_seen_option_content == 1 {
                    // [sic! (the whole thing)]
                    // so that we don't end up printing <ending option group> into the stream we set the text to be empty
                    // I dislike this and need to look into if you can set a debug text setting in ANTLR
                    // TODO: see above comment
                    // this.InsertToken("<ending option group>", YarnSpinnerLexer.BLANK_LINE_FOLLOWING_OPTION);
                    self.insert_token("", yarnspinnerlexer::BLANK_LINE_FOLLOWING_OPTION);
                }
                // disabling the option tracking
                self.last_seen_option_content = None;
            }
        }

        let current_indentation_length = self.get_length_of_newline_token(&current_token);

        // we need to actually see if there is a shortcut *somewhere* above us
        // if there isn't we just chug on without worrying
        if self.line_contains_shortcut {
            // we have a shortcut *somewhere* above us
            // that means we need to check our depth
            // and compare it to the shortcut depth

            // if the depth of the current line is greater than the previous one
            // we need to add this depth to the indents stack
            if current_indentation_length > self.last_indent {
                self.unbalanced_indents.push(current_indentation_length);
                // [sic!] so that we don't end up printing <indent to 8> into the stream we set the text to be empty
                // I dislike this and need to look into if you can set a debug text setting in ANTLR
                // TODO: see above comment
                // this.InsertToken($"<indent to {currentIndentationLength}>", YarnSpinnerLexer.INDENT);
                self.insert_token("", yarnspinnerlexer::INDENT);
            }

            // we've now started tracking the indentation, or ignored it, so can turn this off
            self.line_contains_shortcut = false;
            self.last_seen_option_content = Some(self.base.get_line());
        }

        // now we need to see if the current depth requires any indents or dedents
        // we do this by first checking to see if there are any unbalanced indents
        if let Some(&initial_top) = self.unbalanced_indents.peek() {
            // [sic!] later should make it check if indentation has changed inside the statement block and throw out a warning
            // this.warnings.Add(new Warning { Token = currentToken, Message = "Indentation inside of shortcut block has changed. This is generally a bad idea."});

            // while there are unbalanced indents
            // we need to check if the current line is shallower than the indent stack
            // if it is then we emit a dedent and continue checking

            let mut top = initial_top;

            while current_indentation_length < top {
                // so that we don't end up printing <indent from 8> into the stream we set the text to be empty
                // I dislike this and need to look into if you can set a debug text setting in ANTLR
                // TODO: see above comment
                // this.InsertToken($"<dedent from {top}>", YarnSpinnerLexer.DEDENT);
                self.insert_token("", yarnspinnerlexer::DEDENT);

                self.unbalanced_indents.pop();

                top = if let Some(&next) = self.unbalanced_indents.peek() {
                    next
                } else {
                    // we've dedented all the way out of the shortcut
                    // as such we are done with the option block
                    // previousLineWasOptionOrOptionBlock = false;
                    self.last_seen_option_content = Some(self.base.get_line());
                    0
                };
            }
        }

        // finally we update the last seen depth
        self.last_indent = current_indentation_length;
    }

    fn handle_eof_token(
        &mut self,
        current_token: Box<antlr_rust::token::GenericToken<std::borrow::Cow<'input, str>>>,
    ) {
        // We're at the end of the file. Emit as many dedents as we currently have on the stack.
        while let Some(_indent) = self.unbalanced_indents.pop() {
            // so that we don't end up printing <dedent from 8> into the stream we set the text to be empty
            // I dislike this and need to look into if you can set a debug text setting in ANTLR
            // TODO: see above comment
            // this.InsertToken($"<dedent: {indent}>", YarnSpinnerLexer.DEDENT);
            self.insert_token("", yarnspinnerlexer::DEDENT);
        }

        // Finally, enqueue the EOF token.
        self.pending_tokens.enqueue(current_token);
    }

    /// Given a NEWLINE token, return the length of the indentation
    /// following it by counting the spaces and tabs after it.
    fn get_length_of_newline_token(
        &mut self,
        current_token: &antlr_rust::token::GenericToken<std::borrow::Cow<'input, str>>,
    ) -> isize {
        if current_token.token_type != yarnspinnerlexer::NEWLINE {
            panic!("Current token must NOT be newline")
        }

        let mut length = 0;
        let mut saw_spaces = false;
        let mut saw_tabs = false;

        for c in current_token.get_text().chars() {
            match c {
                ' ' => {
                    length += 1;
                    saw_spaces = true;
                }
                '\t' => {
                    length += 8; // Ye, really (see reference implementation)
                    saw_tabs = true;
                }
                _ => {}
            }
        }

        if saw_spaces && saw_tabs {
            self.diagnostics.borrow_mut().push(
                Diagnostic::from_message("Indentation contains tabs and spaces")
                    .with_range(get_newline_indentation_range(current_token))
                    .with_context(get_newline_indentation_text(current_token))
                    .with_start_line(current_token.line as usize)
                    .with_file_name(self.file_name.clone())
                    .with_severity(DiagnosticSeverity::Warning),
            );
        }

        length
    }

    /// Inserts a new token with the given text and type, as though it
    /// had appeared in the input stream.
    fn insert_token(&mut self, text: impl Into<String>, token_type: isize) {
        // https://www.antlr.org/api/Java/org/antlr/v4/runtime/Lexer.html#_tokenStartCharIndex
        let start_index = self.base.token_start_char_index + self.base.get_text().len() as isize;

        let line = self.get_line();
        let char_position_in_line = self.get_char_position_in_line();

        let token = CommonTokenFactory.create(
            self.base.input.as_mut(),
            token_type,
            Some(text.into()),
            TOKEN_DEFAULT_CHANNEL,
            start_index,
            start_index - 1,
            line,
            char_position_in_line,
        );

        self.pending_tokens.enqueue(token);
    }

    /// antlr4rust does not parse non-ASCII characters properly, so we have to do a second pass
    fn fix_non_ascii_text(
        &self,
        current: &Box<CommonToken>,
        next: &Option<Box<CommonToken>>,
    ) -> String {
        let original_text = current.get_text();
        if original_text == "<EOF>" {
            return original_text.to_string();
        }

        let start_line = (current.line as usize).saturating_sub(1);
        let start_column = current.column as usize;

        let (end_line, end_column) = if let Some(next) = next {
            (next.line as usize - 1, next.column as usize)
        } else {
            (
                self.raw_input.lines().count(),
                self.raw_input.lines().last().unwrap().len(),
            )
        };

        let fixed = if start_line == end_line {
            self.raw_input
                .lines()
                .nth(start_line)
                .unwrap_or(self.raw_input)[start_column..end_column]
                .to_string()
        } else {
            self.raw_input
                .lines()
                .enumerate()
                .skip(start_line)
                .take(end_line - start_line + 1)
                .map(|(i, line)| {
                    if i == start_line {
                        &line[start_column..]
                    } else if i == end_line {
                        &line[..end_column]
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        let mut fixed = fixed.as_str();
        if original_text.trim_end() == original_text {
            fixed = fixed.trim_end();
        }
        if original_text.trim_start() == original_text {
            fixed = fixed.trim_start();
        }
        if !original_text.ends_with('\\') {
            fixed = fixed.trim_end_matches('\\');
        }

        return fixed.to_string();
    }
}

fn get_newline_indentation_range(token: &CommonToken<'_>) -> Range<Position> {
    // +1 compared to similar code because we don't want to start at the newline
    let line = token.get_line_as_usize();

    let start = Position { line, character: 0 };
    let stop = Position {
        line,
        character: token.get_text().len(),
    };

    start..stop
}

fn get_newline_indentation_text(token: &CommonToken<'_>) -> String {
    // Skip newline
    token.get_text().chars().skip(1).collect()
}

#[cfg(test)]
mod tests {
    //! To check for behaviour of the C# `IndentAwareLexer`, the following code can be added to a test file in the C# reference:
    //!
    //! Whitespace in the raw string literal is important, and not correctly rendered by markdown renderers!
    //!
    //! ```csharp
    //! using Antlr4.Runtime;
    //! using System.Linq;
    //! using Xunit;
    //! using Xunit.Abstractions;
    //! using Yarn.Compiler;
    //!
    //! namespace YarnSpinner.Tests;
    //!
    //! public class IndentAwareLexerTest
    //! {
    //!     private readonly ITestOutputHelper _testOutputHelper;
    //!
    //!     public IndentAwareLexerTest(ITestOutputHelper testOutputHelper)
    //!     {
    //!         _testOutputHelper = testOutputHelper;
    //!     }
    //!
    //!     [Fact]
    //!     public void NewOne()
    //!     {
    //!         const string input = """
    //! title: Start
    //! ---
    //! -> Option 1
    //!     Nice.
    //! -> Option 2
    //!     Nicer
    //!
    //! This is part of the previous option statement due to indentation on the empty line ahead
    //!
    //! And this doesn't, as the indentation is reset beforehand.
    //!
    //! This belongs to the previous statement, for the same reason.
    //!
    //! ===
    //! """;
    //!
    //!         // For the reference without indentation awareness copy the full lexer and change the base class to `Lexer` (of ANTLR)
    //!         var referenceLexer = new YarnSpinnerLexer(CharStreams.fromstring(input));
    //!         var referenceTokens = referenceLexer.GetAllTokens();
    //!         _testOutputHelper.WriteLine("[{0}]", string.Join(",\n", referenceTokens.Select(t => $"\"{YarnSpinnerLexer.DefaultVocabulary.GetSymbolicName(t.Type)}\"")));
    //!     }
    //! }
    //! ```
    use super::*;
    use crate::prelude::generated::yarnspinnerlexer::YarnSpinnerLexer as GeneratedYarnSpinnerLexer;
    use antlr_rust::{
        common_token_stream::CommonTokenStream, int_stream::IntStream, token::TOKEN_EOF,
        InputStream,
    };

    #[test]
    fn behaves_like_lexer_for_unindented_input() {
        const MINIMAL_INPUT: &str = "title: Minimal Yarn
---
This is the one and only line
===";

        let generated_lexer = GeneratedYarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT));
        let indent_aware_lexer = IndentAwareYarnSpinnerLexer::new(
            InputStream::new(MINIMAL_INPUT),
            InputStream::new(MINIMAL_INPUT),
            MINIMAL_INPUT,
            "input.yarn".to_owned(),
        );

        let mut reference_token_stream = CommonTokenStream::new(generated_lexer);
        let mut indent_aware_token_stream = CommonTokenStream::new(indent_aware_lexer);

        assert_eq!(
            reference_token_stream.size(),
            indent_aware_token_stream.size()
        );

        // Sanity check: Make sure at least one token is read: We do have input.
        assert_eq!(
            reference_token_stream.iter().next(),
            indent_aware_token_stream.iter().next()
        );

        // Can not do this, as trying to read EOF panics...
        // Iterator::eq(
        //     reference_token_stream.iter(),
        //     indent_aware_token_stream.iter(),
        // );

        while reference_token_stream.la(1) != TOKEN_EOF {
            assert_eq!(
                reference_token_stream.iter().next(),
                indent_aware_token_stream.iter().next()
            );
        }

        assert_eq!(TOKEN_EOF, reference_token_stream.la(1));
        assert_eq!(TOKEN_EOF, indent_aware_token_stream.la(1));
    }

    #[test]
    fn correctly_indents_and_dedents_with_token() {
        let option_indentation_relevant_input: &str = include_str!("significant_whitespace.yarn");

        let indent_aware_lexer = IndentAwareYarnSpinnerLexer::new(
            InputStream::new(option_indentation_relevant_input),
            InputStream::new(option_indentation_relevant_input),
            option_indentation_relevant_input,
            "input.yarn".to_owned(),
        );

        let mut indent_aware_token_stream = CommonTokenStream::new(indent_aware_lexer);

        let mut tokens = vec![indent_aware_token_stream.iter().next().unwrap()];

        while indent_aware_token_stream.la(1) != TOKEN_EOF {
            tokens.push(indent_aware_token_stream.iter().next().unwrap());
        }

        let symbols: Vec<_> = tokens
            .into_iter()
            .map(|t| yarnspinnerlexer::_SYMBOLIC_NAMES[t as usize].unwrap())
            .collect();

        // Tests the stability of the lexer, targeted at indents and dedents - might break due to internal changes!
        // See generated_lexer_output_is_same_as_reference for the commented out lines :)
        // TODO: investigate if we can do anything. Maybe fix the rust antlr generator?
        let expected = vec![
            "ID",
            "HEADER_DELIMITER",
            "REST_OF_LINE",
            // "NEWLINE",
            "BODY_START",
            // "NEWLINE",
            "SHORTCUT_ARROW",
            // "BODY_WS",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "INDENT",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "DEDENT",
            "SHORTCUT_ARROW",
            // "BODY_WS",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "INDENT",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "DEDENT",
            // "NEWLINE",
            "BLANK_LINE_FOLLOWING_OPTION",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "BODY_END",
        ];

        assert_eq!(expected, symbols);
    }

    #[test]
    fn generated_lexer_output_is_same_as_reference() {
        let option_indentation_relevant_input: &str = include_str!("significant_whitespace.yarn");

        let generated_lexer =
            GeneratedYarnSpinnerLexer::new(InputStream::new(option_indentation_relevant_input));
        let mut reference_token_stream = CommonTokenStream::new(generated_lexer);

        let mut tokens = vec![reference_token_stream.iter().next().unwrap()];

        while reference_token_stream.la(1) != TOKEN_EOF {
            tokens.push(reference_token_stream.iter().next().unwrap());
        }

        let symbols: Vec<_> = tokens
            .into_iter()
            .map(|t| yarnspinnerlexer::_SYMBOLIC_NAMES[t as usize].unwrap())
            .collect();

        // Tests the compatibility of the generated lexer with a manually generated output from the reference implementation.
        // The commented out lines are not correctly lexed by the generated lexer (in comparison with the C# generated lexer).
        let expected = vec![
            "ID",
            "HEADER_DELIMITER",
            "REST_OF_LINE",
            // "NEWLINE",
            "BODY_START",
            // "NEWLINE",
            "SHORTCUT_ARROW",
            // "BODY_WS",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "SHORTCUT_ARROW",
            // "BODY_WS",
            "TEXT",
            "TEXT",
            "NEWLINE",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "TEXT",
            "TEXT",
            // "NEWLINE",
            "NEWLINE",
            "BODY_END",
        ];

        assert_eq!(expected, symbols);
    }
}
