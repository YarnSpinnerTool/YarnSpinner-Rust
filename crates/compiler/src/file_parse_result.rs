//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/FileParseResult.cs>

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::ParseTree;
use antlr_rust::TokenSource;

/// Contains the result of parsing a single file of source code.
///
/// This class provides only syntactic information about a parse - that is,
/// it provides access to the parse tree, and the stream of tokens used to
/// produce that parse tree.
pub struct FileParseResult<'input, P, T>
where
    P: ParseTree<'input>,
    T: TokenSource<'input>,
{
    pub name: String,

    pub tree: P,

    pub tokens: CommonTokenStream<'input, T>,
}
