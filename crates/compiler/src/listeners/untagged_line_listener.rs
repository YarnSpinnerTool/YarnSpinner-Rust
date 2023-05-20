//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Utils.cs>

use crate::parser::generated::yarnspinnerparser::Line_statementContext;
use crate::prelude::generated::yarnspinnerparser::{
    Line_statementContextAttrs, YarnSpinnerParserContextType,
};
use crate::prelude::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;
use crate::prelude::*;
use crate::visitors::get_hashtag_texts;
use antlr_rust::int_stream::IntStream;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::ParseTreeListener;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::AtomicBool;

pub(crate) struct UntaggedLineListener<'input> {
    existing_line_tags: Vec<LineId>,
    file: FileParseResult<'input>,
    pub(crate) rewritten_lines: Rc<RefCell<Vec<String>>>,
    pub(crate) rewrote_anything: Rc<AtomicBool>,
}

impl<'input> UntaggedLineListener<'input> {
    pub fn new(existing_line_tags: Vec<LineId>, file: FileParseResult<'input>) -> Self {
        let original_source = file
            .tokens()
            .get_all_text()
            .lines()
            .map(|s| s.to_owned())
            .collect();
        Self {
            existing_line_tags,
            file,
            rewritten_lines: Rc::new(RefCell::new(original_source)),
            rewrote_anything: Default::default(),
        }
    }

    /// Generates a new unique line tag that is not present in `existing_line_tags`.
    fn generate_string(&self) -> LineId {
        let mut rng = SmallRng::from_entropy();
        loop {
            let line: usize = rng.gen();
            let tag = LineId(format!("line:{}", line));
            if !self.existing_line_tags.contains(&tag) {
                return tag;
            }
        }
    }
}

impl<'input> ParseTreeListener<'input, YarnSpinnerParserContextType>
    for UntaggedLineListener<'input>
{
}

impl<'input> YarnSpinnerParserListener<'input> for UntaggedLineListener<'input> {
    fn exit_line_statement(&mut self, ctx: &Line_statementContext<'input>) {
        // We're looking at a complete line statement.

        // First, figure out if this line statement already has a line
        // tag. Start by taking the hashtags...
        let hashtags = ctx.hashtag_all();

        // Get the text for all of these hashtags...
        let texts = get_hashtag_texts(&hashtags);

        // And then look for a line ID hashtag.
        if texts.iter().any(|tag| tag.starts_with("line:")) {
            return;
        }

        // Find the index of the first token on the default channel to
        // the left of the newline.
        let index = ctx.NEWLINE().unwrap().symbol.get_token_index();

        let tokens = self.file.tokens();
        let previous_token_index = index_of_previous_token_on_channel(tokens, index);
        let line_index = ctx.start().get_line_as_usize().saturating_sub(1);

        // Did we find one?
        let previous_token_index = previous_token_index.unwrap_or_else(|| {
            // No token was found before this newline. This is an
            // internal error - there must be at least one symbol
            // besides the terminating newline.
            panic!("Internal error: failed to find any tokens before the newline in line statement on line {line_index}. \
                   This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new");
        });
        // Get the token at this index. We'll put our tag after it.
        let previous_token = tokens.get(previous_token_index);

        // Generate a new, unique line ID.
        let new_line_id = self.generate_string();
        // Record that we've used this new line ID, so that we don't
        // accidentally use it twice.
        self.existing_line_tags.push(new_line_id.clone());

        let mut lines = self.rewritten_lines.borrow_mut();
        let line = lines.get_mut(line_index).unwrap();

        let insertion_index = line
            .char_indices()
            .map(|(byte_pos, _char)| byte_pos)
            .nth(previous_token.get_column_as_usize())
            .unwrap_or_else(||
                panic!("Internal error: failed to convert char pos to byte pos for insertion index on line {line_index}. \
                        This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new"))
            + previous_token.get_text().len();
        line.insert_str(insertion_index, &format!(" #{new_line_id} "));
        self.rewrote_anything
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }
}

/// Gets the index of the first token to the left of the token at `index` that's on the default token channel.
///
/// ## Return value
///
/// Returns the index of the first token before the token at `index` that is on the channel `0`.
/// If none is found, returns [`None`]. If `index` is beyond the size of `token_stream`, returns the index of the last token in the stream.
fn index_of_previous_token_on_channel(
    token_stream: &ActualTokenStream,
    index: isize,
) -> Option<isize> {
    let default_token_channel = 0;
    // Are we beyond the list of tokens?
    if index >= token_stream.size() {
        // Return the final token in the channel, which will be an EOF.
        return Some(token_stream.size() - 1);
    }
    // 'index' is the token we want to start searching from. We want
    // to find items before it, so start looking from the token before it.

    // Walk backwards through the tokens list.
    (0..index)
        .rev()
        .find(|&i| token_stream.get(i).get_channel() == default_token_channel)
}
