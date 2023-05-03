//! Contains functionality provided in the C# implementation of ANTLR but not (yet?) in antlr4rust.

use crate::parser::generated::yarnspinnerparser::YarnSpinnerParserContextType;
use crate::parser::ActualTokenStream;
use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserContext;
use crate::prelude::ContextRefExt;
use antlr_rust::int_stream::IntStream;
use antlr_rust::rule_context::RuleContext;
use antlr_rust::token::{CommonToken, Token, TOKEN_DEFAULT_CHANNEL};
use antlr_rust::token_factory::{CommonTokenFactory, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::ErrorNode;
use antlr_rust::InputStream;
use better_any::TidExt;
use std::rc::Rc;

pub(crate) trait CommonTokenStreamExt<'input> {
    /// Collect all tokens on specified channel to the left of
    /// the current token up until we see a token on
    /// [`Lexer::DefaultTokenChannel`].
    /// If `channel` is `-1`, find any non default channel token.
    fn get_hidden_tokens_to_left(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>>;

    /// Collect all tokens on specified channel to the right of
    /// the current token up until we see a token on
    /// [`Lexer::DefaultTokenChannel`] or EOF.
    /// If `channel` is `-1`, find any non default channel token.
    fn get_hidden_tokens_to_right(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>>;

    /// A collection of all tokens fetched from the token source.
    ///
    /// The list is considered a complete view of the input once
    /// `fetchedEOF` is set to `true`.
    fn get_tokens(&self) -> Vec<CommonToken<'input>>;
}

trait PrivateCommonTokenStreamExt<'input> {
    /// Given a starting index, return the index of the previous token on
    /// channel.
    ///
    /// Given a starting index, return the index of the previous token on channel.
    /// Return `token_index` if `tokens[token_index]` is on channel.
    /// Return -1 if there are no tokens on channel between
    /// `token_index` and 0.
    /// If `token_index` specifies an index at or after the EOF token, the EOF token
    /// index is returned. This is due to the fact that the EOF token is treated
    /// as though it were on every channel.
    fn previous_token_on_channel(&self, token_index: isize, channel: isize) -> isize;

    /// Given a starting index, return the index of the next token on channel.
    ///
    /// Given a starting index, return the index of the next token on channel.
    /// Return `token_index` if `tokens[token_index]` is on channel.
    ///
    /// Return the index of
    /// the EOF token if there are no tokens on channel between `token_index` and EOF
    fn next_token_on_channel(&self, token_index: isize, channel: isize) -> isize;

    fn filter_for_channel(
        &self,
        from: isize,
        to: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>>;
}

impl<'input> CommonTokenStreamExt<'input> for ActualTokenStream<'input> {
    fn get_hidden_tokens_to_left(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>> {
        // Adapted from <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/BufferedTokenStream.cs#L563>

        // This method is private, but it should be alright to leave it out.
        // this.setup();

        if token_index < 0 || token_index >= self.size() {
            panic!("{} not in 0..{}", token_index, self.size() - 1);
        }
        if token_index == 0 {
            return vec![];
        }
        let num = self.previous_token_on_channel(token_index - 1, 0);
        if num == token_index - 1 {
            vec![]
        } else {
            self.filter_for_channel(num + 1, token_index - 1, channel)
        }
    }

    fn get_hidden_tokens_to_right(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>> {
        // Adapted from <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/BufferedTokenStream.cs#L519>

        // This method is private, but it should be alright to leave it out.
        // this.setup();

        if token_index < 0 || token_index >= self.size() {
            panic!("{} not in 0..{}", token_index, self.size() - 1);
        }
        if token_index == 0 {
            return vec![];
        }
        let next_on_channel = self.next_token_on_channel(token_index + 1, 0);
        let from = token_index + 1;
        let to = if next_on_channel != -1 {
            next_on_channel
        } else {
            self.size() - 1
        };
        self.filter_for_channel(from, to, channel)
    }

    fn get_tokens(&self) -> Vec<CommonToken<'input>> {
        // Not adapted from anywhere, ANTLR for C# directly exposes this.
        // Unfortunately, we go and collect the data ourselves.
        (0..self.size()).map(|i| *self.get(i).clone()).collect()
    }
}

impl<'input> PrivateCommonTokenStreamExt<'input> for ActualTokenStream<'input> {
    fn previous_token_on_channel(&self, mut token_index: isize, channel: isize) -> isize {
        // Adapted from <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/BufferedTokenStream.cs#L488>

        // This method is private, but it should be alright to leave it out.
        // this.sync(token_index);

        if token_index >= self.size() {
            return self.size() - 1;
        }
        while token_index >= 0 {
            let token = self.get(token_index).clone();
            if token.get_token_type() == -1 || token.get_channel() == channel {
                return token_index;
            }
            token_index -= 1;
        }
        token_index
    }

    fn next_token_on_channel(&self, mut token_index: isize, channel: isize) -> isize {
        // Already in antlr4rust, but private
        // This method is private, but it should be alright to leave it out.
        // this.sync(token_index);

        if token_index >= self.size() {
            return self.size() - 1;
        }

        let tokens = self.get_tokens();
        let mut token = &tokens[token_index as usize];
        while token.get_channel() != channel {
            if token.get_token_type() == antlr_rust::int_stream::EOF || token_index < 0 {
                return token_index;
            }

            token_index += 1;
            // self.sync(i);
            token = &tokens[token_index as usize];
        }

        token_index
    }

    fn filter_for_channel(
        &self,
        from: isize,
        to: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>> {
        // Adapted from <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/BufferedTokenStream.cs#L597>
        let mut token_list = Vec::new();

        for index in from..=to {
            let token = *self.get(index).clone();
            if channel == -1 {
                if token.get_channel() != 0 {
                    token_list.push(token);
                }
            } else if token.get_channel() == channel {
                token_list.push(token);
            }
        }
        token_list
    }
}

pub(crate) trait YarnSpinnerParserContextExt<'input>:
    YarnSpinnerParserContext<'input>
{
    /// Same as [`YarnSpinnerParserContext::child_of_type`], but without the [`Sized`] requirement.
    fn child_of_type_unsized<T: YarnSpinnerParserContext<'input>>(
        &self,
        pos: usize,
    ) -> Option<Rc<T>> {
        self.get_children()
            .filter_map(|child| child.downcast_rc::<T>().ok())
            .nth(pos)
    }

    /// Adapted from <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/ParserRuleContext.cs#L220>
    fn add_error_node(
        &self,
        bad_token: CommonToken<'input>,
    ) -> Rc<ErrorNode<'input, YarnSpinnerParserContextType>> {
        let error_node = Rc::new(ErrorNode::new(Box::new(bad_token)));
        error_node.set_parent(&Some(self.ref_to_rc()));
        self.add_child(error_node.clone());
        error_node
    }
}
impl<'input, T: YarnSpinnerParserContext<'input> + ?Sized> YarnSpinnerParserContextExt<'input>
    for T
{
}

pub(crate) fn create_common_token<'a>(
    token_type: isize,
    text: impl Into<String>,
) -> Box<CommonToken<'a>> {
    // The default values can be found strewn across <https://github.com/antlr/antlr4/blob/8dcc6526cfb154d688497f31cf1e0904801c6df2/runtime/CSharp/src/CommonToken.cs>
    CommonTokenFactory.create::<InputStream<&'a str>>(
        None,
        token_type,
        Some(text.into()),
        TOKEN_DEFAULT_CHANNEL,
        0,
        0,
        0,
        -1,
    )
}
