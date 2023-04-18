use crate::parser::yarnspinnerlexer::YarnSpinnerLexer;
use crate::parser::yarnspinnerparser::{
    DialogueContext, DialogueContextAll, LocalTokenFactory, YarnSpinnerParser,
};
use antlr_rust::char_stream::CharStream;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::errors::ANTLRError;
use antlr_rust::token_stream::TokenStream;
use antlr_rust::{InputStream, TidAble};
use std::borrow::Cow;
use std::ops::Deref;
use std::rc::Rc;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, YarnCompilerError>;

pub fn compile<'input>(target: impl YarnCompilerTarget) -> Result<Rc<DialogueContextAll<'input>>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    let input_string = target.get_input_string().as_ref();
    let input_stream = YarnSpinnerLexer::new(InputStream::new(input_string.into()));
    let lexer = YarnSpinnerLexer::new(input_stream);
    let mut parser = YarnSpinnerParser::new(CommonTokenStream::new(lexer));
    let dialogue = parser.dialogue()?;
    Ok(dialogue)
}

pub trait YarnCompilerTarget {
    /// Not lazy because the types were getting out of hand
    fn get_input_string(&self) -> Cow<'static, str>;
}

impl YarnCompilerTarget for str {
    fn get_input_string(&self) -> Cow<'static, str> {
        self.into()
    }
}

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum YarnCompilerError {
    #[error(transparent)]
    Antlr(#[from] ANTLRError),
}
