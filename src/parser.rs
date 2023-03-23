use std::fmt::Display;
use winnow::{
    branch::alt,
    bytes::{any, take_till0, take_till1, take_until0, take_while1},
    character::{alpha1, newline, space0},
    combinator::{eof, opt, rest, Or},
    multi::{many0, many1},
    prelude::*,
    sequence::{preceded, separated_pair, terminated},
    stream::AsChar,
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
    ((parse_file_hashtags, many1(parse_node)))
        .map(|(hashtags, nodes)| Dialogue { nodes })
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

// TODO: forbid those as in g4 of reference or not needed as we do lex/parse in one step?
/* ~[ \t\r\n#$<]+ */
fn hashtag_text(input: &str) -> IResult<&str, &str> {
    terminated(take_till1(AsChar::is_newline), newline).parse_next(input) // TODO: crlf?
}

/*
   node
       : header+  BODY_START  body BODY_END
       ;
*/
fn parse_node(input: &str) -> IResult<&str, Node> {
    // print!("{} kabo", parse_header.parse_next(input).unwrap().0);

    // TODO: parse many1 header
    terminated(parse_header, newline)
        .context("Parse node error") // TODO: allow eof
        .map(|(h, v)| Node {
            header_key: h,
            header_value: v,
        }) // TODO handle r
        .parse_next(input)
}

/*
   header
       : header_key=ID HEADER_DELIMITER  header_value=REST_OF_LINE?
       ;
*/
fn parse_header(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    separated_pair(
        parse_identifier,
        parse_header_delimiter,
        opt(take_till0(AsChar::is_newline)),
    )
    .parse_next(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1
        .verify(|id: &str| id.chars().nth(0).map_or(false, AsChar::is_alpha))
        .context("Could not parse identifer") // take_while1 guarantees 1 char
        .parse_next(input)
}

fn parse_header_delimiter(input: &str) -> IResult<&str, &str> {
    preceded(":", space0).parse_next(input)
}

#[derive(Debug)]
pub struct Dialogue<'a> {
    nodes: Vec<Node<'a>>,
}

#[derive(Debug)]
pub struct Node<'a> {
    header_key: &'a str,
    header_value: Option<&'a str>,
}

impl<'a> Display for Dialogue<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.nodes)
    }
}

impl<'a> Display for Node<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", "empty node")
    }
}
