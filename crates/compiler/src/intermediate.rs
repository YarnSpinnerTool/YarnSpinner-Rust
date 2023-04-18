//! Contains in-memory idiomatic representations of the Yarn Spinner AST.
//! Adapted from `YarnSpinnerParser.g4`

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub(crate) struct Dialogue {
    pub(crate) hashtags: Vec<FileHashtag>,
    pub(crate) nodes: Vec<Node>,
}

#[derive(Debug, Clone, Deref, DerefMut)]
pub(crate) struct FileHashtag(String);

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub(crate) enum Statement {
    LineStatement(LineStatement),
    IfStatement(String),
    SetStatement(String),
    ShortcutOptionStatement(String),
    CallStatement(String),
    CommandStatement(String),
    DeclareStatement(String),
    JumpStatement(String),
    Indent(Vec<Statement>),
}

#[derive(Debug, Clone)]
pub(crate) struct LineStatement {
    pub(crate) formatted_text: FormattedText,
    pub(crate) condition: Option<String>,
    pub(crate) hashtags: Vec<Hashtag>,
}

#[derive(Debug, Clone)]
pub(crate) struct FormattedText {
    pub(crate) text: String,
    pub(crate) formatting: Vec<Formatting>,
}
