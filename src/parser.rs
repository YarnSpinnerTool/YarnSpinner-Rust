#![feature(associated_type_bounds)]

use std::fmt::Display;
use winnow::{
    bytes::{one_of, take_till1, take_until1, take_while1},
    multi::{many0, many1},
    prelude::*,
    sequence::preceded,
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
    preceded(parse_file_hashtags, many1(parse_node))
        .map(|nodes| Dialogue { nodes })
        .parse_next(input)
}

fn parse_file_hashtags(input: &str) -> IResult<&str, ()> {
    many0(parse_file_hashtag).parse_next(input)
}

/*
   file_hashtag
       : HASHTAG text=HASHTAG_TEXT
       ;
*/
fn parse_file_hashtag(input: &str) -> IResult<&str, ()> {
    ("#", hashtag_text).map(|_| ()).parse_next(input)
}

/* ~[ \t\r\n#$<]+ */
fn hashtag_text(input: &str) -> IResult<&str, &str> {
    take_till1(" \t\r\n#$<")(input)
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
