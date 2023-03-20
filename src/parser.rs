use std::fmt::Display;
use winnow::{
    multi::{many0, many1},
    prelude::*,
};

// See https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.3.0/YarnSpinner.Compiler/YarnSpinnerParser.g4
pub fn parse(input: &str) -> Dialogue {
    parse_dialogue(input).finish().unwrap()
}

/*
    dialogue
        : (file_hashtag*) node+
        ;
*/
fn parse_dialogue(input: &str) -> IResult<&str, Dialogue> {
    let (remainder, ()) = many0(parse_file_hashtag).parse_next(input).unwrap();
    many1(parse_node)
        .map(|nodes| Dialogue { nodes })
        .parse_next(remainder)
}

fn parse_file_hashtag(input: &str) -> IResult<&str, ()> {
    todo!()
}

fn parse_node(input: &str) -> IResult<&str, Node> {
    todo!()
}

#[derive(Debug)]
pub struct Dialogue {
    nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {}

impl Display for Dialogue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
