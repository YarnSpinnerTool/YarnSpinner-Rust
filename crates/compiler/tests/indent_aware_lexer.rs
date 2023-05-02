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
use antlr_rust::{
    common_token_stream::CommonTokenStream, int_stream::IntStream, token::TOKEN_EOF, InputStream,
};
use yarn_slinger_compiler::prelude::{
    generated::yarnspinnerlexer::{self, YarnSpinnerLexer as GeneratedYarnSpinnerLexer},
    YarnSpinnerLexer as IndentAwareYarnSpinnerLexer,
};

#[test]
fn behaves_like_lexer_for_unindented_input() {
    const MINIMAL_INPUT: &str = "title: Minimal Yarn
---
This is the one and only line
===";

    let generated_lexer = GeneratedYarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT));
    let indent_aware_lexer =
        IndentAwareYarnSpinnerLexer::new(InputStream::new(MINIMAL_INPUT), "input.yarn".to_owned());

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
    // Careful: IDE's love to break the following significant whitespace!
    let option_indentation_relevant_input: &str = "title: Start
---
-> Option 1
    Nice.
-> Option 2
    Nicer
    
    This is part of the previous option statement due to indentation on the empty line ahead

    And this doesn't, as the indentation is reset beforehand.
    
    This belongs to the previous statement, for the same reason.
    
===";

    let indent_aware_lexer = IndentAwareYarnSpinnerLexer::new(
        InputStream::new(option_indentation_relevant_input),
        "input.yarn".to_owned(),
    );

    let mut indent_aware_token_stream = CommonTokenStream::new(indent_aware_lexer);

    let mut tokens = vec![indent_aware_token_stream.iter().next().unwrap()];

    while indent_aware_token_stream.la(1) != TOKEN_EOF {
        tokens.push(indent_aware_token_stream.iter().next().unwrap());
    }

    let names: Vec<_> = tokens
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

    assert_eq!(expected, names);
}

#[test]
fn generated_lexer_output_is_same_as_reference() {
    let option_indentation_relevant_input: &str = "title: Start
---
-> Option 1
    Nice.
-> Option 2
    Nicer

    This is part of the previous option statement due to indentation on the empty line ahead

    And this doesn't, as the indentation is reset beforehand.

    This belongs to the previous statement, for the same reason.

===";

    let generated_lexer =
        GeneratedYarnSpinnerLexer::new(InputStream::new(option_indentation_relevant_input));
    let mut reference_token_stream = CommonTokenStream::new(generated_lexer);

    let mut tokens = vec![reference_token_stream.iter().next().unwrap()];

    while reference_token_stream.la(1) != TOKEN_EOF {
        tokens.push(reference_token_stream.iter().next().unwrap());
    }

    let names: Vec<_> = tokens
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

    assert_eq!(expected, names);
}
