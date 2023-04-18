use antlr_rust::{common_token_stream::CommonTokenStream, *};
use rusty_yarn_spinner::parser::{yarnspinnerlexer::*, yarnspinnerparser::*};

fn main() {
    let lexer = YarnSpinnerLexer::new(InputStream::new(
        "# hello
# nonono
title: Node_Title
---
Here are some lines!
Wow!
==="
        .into(),
    ));
    let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));

    let dialogue_context = parser.dialogue().unwrap();
    let hashtags = dialogue_context.file_hashtag_all();
    println!("{:?}", hashtags[0].HASHTAG_TEXT());
    println!("{:?}", hashtags[1].HASHTAG_TEXT());
}
