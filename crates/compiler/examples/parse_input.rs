use antlr_rust::{common_token_stream::CommonTokenStream, *};
use rusty_yarn_spinner_compiler::prelude::{yarnspinnerlexer::*, yarnspinnerparser::*};

fn main() {
    let lexer = YarnSpinnerLexer::new(InputStream::new(
        "# hello
# nonono
title: Node_Title
---
Here are some lines!
That's weird?
Wow!
==="
        .into(),
    ));
    let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));

    let dialogue_context = parser.dialogue().unwrap();
    let hashtags = dialogue_context.file_hashtag_all();

    println!("{:?}", hashtags[0].HASHTAG_TEXT());
    println!("{:?}", hashtags[1].HASHTAG_TEXT());

    let nodes = dialogue_context.node_all();
    let first_node: &std::rc::Rc<parser_rule_context::BaseParserRuleContext<NodeContextExt>> =
        &nodes[0];
    let statements = first_node.body().unwrap().statement_all();
    println!(
        "{:?}",
        statements[0]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .TEXT_all()
    );
    println!(
        "{:?}",
        statements[1]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .TEXT_all()
    );
    println!(
        "{:?}",
        statements[2]
            .line_statement()
            .unwrap()
            .line_formatted_text()
            .unwrap()
            .TEXT_all()
    );

    let title = first_node.header(0);
    println!("{:?}", title.unwrap().REST_OF_LINE().unwrap())
}
