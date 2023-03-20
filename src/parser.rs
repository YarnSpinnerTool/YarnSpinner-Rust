use std::fmt::Display;

use winnow::prelude::*;

// See https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.3.0/YarnSpinner.Compiler/YarnSpinnerParser.g4
/*
    dialogue
        : (file_hashtag*) node+
        ;
*/
pub fn parse(input: &str) -> Dialogue {
    parse_dialogue(input).finish().unwrap()
}

fn parse_dialogue(input: &str) -> IResult<&str, Dialogue> {
    todo!()
}

#[derive(Debug)]
pub struct Dialogue {}

impl Display for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
