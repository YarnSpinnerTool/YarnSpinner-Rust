//! Contains functionality provided in the C# implementation of ANTLR but not (yet?) in antlr4rust.

use crate::parser::ActualTokenStream;
use antlr_rust::int_stream::IntStream;
use antlr_rust::token::{CommonToken, Token, TOKEN_DEFAULT_CHANNEL};
use antlr_rust::token_factory::{CommonTokenFactory, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::InputStream;

pub(crate) trait CommonTokenStreamExt<'input> {
    fn get_hidden_tokens_to_left(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>>;

    fn previous_token_on_channel(&self, token_index: isize, channel: isize) -> isize;

    fn filter_for_channel(
        &self,
        from: isize,
        to: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>>;

    fn get_tokens(&self) -> Vec<CommonToken<'input>>;
}

impl<'input> CommonTokenStreamExt<'input> for ActualTokenStream<'input> {
    fn get_hidden_tokens_to_left(
        &self,
        token_index: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>> {
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

    fn previous_token_on_channel(&self, mut token_index: isize, channel: isize) -> isize {
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

    fn filter_for_channel(
        &self,
        from: isize,
        to: isize,
        channel: isize,
    ) -> Vec<CommonToken<'input>> {
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

    fn get_tokens(&self) -> Vec<CommonToken<'input>> {
        (0..self.size()).map(|i| *self.get(i).clone()).collect()
    }
}

pub(crate) fn create_common_token<'a>(
    token_type: isize,
    text: impl Into<String>,
) -> Box<CommonToken<'a>> {
    // Taken from C# implementation of `CommonToken`s constructor
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
