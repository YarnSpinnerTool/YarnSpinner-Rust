// Generated from .\YarnSpinnerParser.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use super::yarnspinnerparserlistener::*;
use super::yarnspinnerparservisitor::*;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::error_strategy::{DefaultErrorStrategy, ErrorStrategy};
use antlr_rust::errors::*;
use antlr_rust::int_stream::EOF;
use antlr_rust::parser::{BaseParser, Parser, ParserNodeType, ParserRecog};
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::parser_rule_context::{cast, cast_mut, BaseParserRuleContext, ParserRuleContext};
use antlr_rust::recognizer::{Actions, Recognizer};
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::token::{CommonToken, OwningToken, Token, TOKEN_EOF};
use antlr_rust::token_factory::{CommonTokenFactory, TokenAware, TokenFactory};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::tree::*;
use antlr_rust::vocabulary::{Vocabulary, VocabularyImpl};
use antlr_rust::TokenSource;
use antlr_rust::{InputStream, PredictionContextCache};

use antlr_rust::lazy_static;
use antlr_rust::{TidAble, TidExt};

use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

pub const INDENT: isize = 1;
pub const DEDENT: isize = 2;
pub const BLANK_LINE_FOLLOWING_OPTION: isize = 3;
pub const WS: isize = 4;
pub const COMMENT: isize = 5;
pub const NEWLINE: isize = 6;
pub const ID: isize = 7;
pub const BODY_START: isize = 8;
pub const HEADER_DELIMITER: isize = 9;
pub const HASHTAG: isize = 10;
pub const REST_OF_LINE: isize = 11;
pub const BODY_WS: isize = 12;
pub const BODY_END: isize = 13;
pub const SHORTCUT_ARROW: isize = 14;
pub const COMMAND_START: isize = 15;
pub const EXPRESSION_START: isize = 16;
pub const ESCAPED_ANY: isize = 17;
pub const TEXT_ESCAPE: isize = 18;
pub const TEXT_COMMENT: isize = 19;
pub const TEXT: isize = 20;
pub const UNESCAPABLE_CHARACTER: isize = 21;
pub const TEXT_COMMANDHASHTAG_WS: isize = 22;
pub const TEXT_COMMANDHASHTAG_COMMENT: isize = 23;
pub const TEXT_COMMANDHASHTAG_ERROR: isize = 24;
pub const HASHTAG_WS: isize = 25;
pub const HASHTAG_TEXT: isize = 26;
pub const EXPR_WS: isize = 27;
pub const KEYWORD_TRUE: isize = 28;
pub const KEYWORD_FALSE: isize = 29;
pub const KEYWORD_NULL: isize = 30;
pub const OPERATOR_ASSIGNMENT: isize = 31;
pub const OPERATOR_LOGICAL_LESS_THAN_EQUALS: isize = 32;
pub const OPERATOR_LOGICAL_GREATER_THAN_EQUALS: isize = 33;
pub const OPERATOR_LOGICAL_EQUALS: isize = 34;
pub const OPERATOR_LOGICAL_LESS: isize = 35;
pub const OPERATOR_LOGICAL_GREATER: isize = 36;
pub const OPERATOR_LOGICAL_NOT_EQUALS: isize = 37;
pub const OPERATOR_LOGICAL_AND: isize = 38;
pub const OPERATOR_LOGICAL_OR: isize = 39;
pub const OPERATOR_LOGICAL_XOR: isize = 40;
pub const OPERATOR_LOGICAL_NOT: isize = 41;
pub const OPERATOR_MATHS_ADDITION_EQUALS: isize = 42;
pub const OPERATOR_MATHS_SUBTRACTION_EQUALS: isize = 43;
pub const OPERATOR_MATHS_MULTIPLICATION_EQUALS: isize = 44;
pub const OPERATOR_MATHS_MODULUS_EQUALS: isize = 45;
pub const OPERATOR_MATHS_DIVISION_EQUALS: isize = 46;
pub const OPERATOR_MATHS_ADDITION: isize = 47;
pub const OPERATOR_MATHS_SUBTRACTION: isize = 48;
pub const OPERATOR_MATHS_MULTIPLICATION: isize = 49;
pub const OPERATOR_MATHS_DIVISION: isize = 50;
pub const OPERATOR_MATHS_MODULUS: isize = 51;
pub const LPAREN: isize = 52;
pub const RPAREN: isize = 53;
pub const COMMA: isize = 54;
pub const EXPRESSION_AS: isize = 55;
pub const STRING: isize = 56;
pub const FUNC_ID: isize = 57;
pub const EXPRESSION_END: isize = 58;
pub const VAR_ID: isize = 59;
pub const DOT: isize = 60;
pub const NUMBER: isize = 61;
pub const COMMAND_WS: isize = 62;
pub const COMMAND_IF: isize = 63;
pub const COMMAND_ELSEIF: isize = 64;
pub const COMMAND_ELSE: isize = 65;
pub const COMMAND_SET: isize = 66;
pub const COMMAND_ENDIF: isize = 67;
pub const COMMAND_CALL: isize = 68;
pub const COMMAND_DECLARE: isize = 69;
pub const COMMAND_JUMP: isize = 70;
pub const COMMAND_ENUM: isize = 71;
pub const COMMAND_CASE: isize = 72;
pub const COMMAND_ENDENUM: isize = 73;
pub const COMMAND_LOCAL: isize = 74;
pub const COMMAND_END: isize = 75;
pub const COMMAND_TEXT_END: isize = 76;
pub const COMMAND_EXPRESSION_START: isize = 77;
pub const COMMAND_TEXT: isize = 78;
pub const TYPE_STRING: isize = 79;
pub const TYPE_NUMBER: isize = 80;
pub const TYPE_BOOL: isize = 81;
pub const RULE_dialogue: usize = 0;
pub const RULE_file_hashtag: usize = 1;
pub const RULE_node: usize = 2;
pub const RULE_header: usize = 3;
pub const RULE_body: usize = 4;
pub const RULE_statement: usize = 5;
pub const RULE_line_statement: usize = 6;
pub const RULE_line_formatted_text: usize = 7;
pub const RULE_hashtag: usize = 8;
pub const RULE_line_condition: usize = 9;
pub const RULE_expression: usize = 10;
pub const RULE_value: usize = 11;
pub const RULE_variable: usize = 12;
pub const RULE_function_call: usize = 13;
pub const RULE_if_statement: usize = 14;
pub const RULE_if_clause: usize = 15;
pub const RULE_else_if_clause: usize = 16;
pub const RULE_else_clause: usize = 17;
pub const RULE_set_statement: usize = 18;
pub const RULE_call_statement: usize = 19;
pub const RULE_command_statement: usize = 20;
pub const RULE_command_formatted_text: usize = 21;
pub const RULE_shortcut_option_statement: usize = 22;
pub const RULE_shortcut_option: usize = 23;
pub const RULE_declare_statement: usize = 24;
pub const RULE_jump_statement: usize = 25;
pub const ruleNames: [&'static str; 26] = [
    "dialogue",
    "file_hashtag",
    "node",
    "header",
    "body",
    "statement",
    "line_statement",
    "line_formatted_text",
    "hashtag",
    "line_condition",
    "expression",
    "value",
    "variable",
    "function_call",
    "if_statement",
    "if_clause",
    "else_if_clause",
    "else_clause",
    "set_statement",
    "call_statement",
    "command_statement",
    "command_formatted_text",
    "shortcut_option_statement",
    "shortcut_option",
    "declare_statement",
    "jump_statement",
];

pub const _LITERAL_NAMES: [Option<&'static str>; 82] = [
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'---'"),
    None,
    Some("'#'"),
    None,
    None,
    Some("'==='"),
    Some("'->'"),
    Some("'<<'"),
    None,
    None,
    Some("'\\'"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'true'"),
    Some("'false'"),
    Some("'null'"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'+='"),
    Some("'-='"),
    Some("'*='"),
    Some("'%='"),
    Some("'/='"),
    Some("'+'"),
    Some("'-'"),
    Some("'*'"),
    Some("'/'"),
    Some("'%'"),
    Some("'('"),
    Some("')'"),
    Some("','"),
    Some("'as'"),
    None,
    None,
    Some("'}'"),
    None,
    Some("'.'"),
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'endif'"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some("'{'"),
    None,
    Some("'string'"),
    Some("'number'"),
    Some("'bool'"),
];
pub const _SYMBOLIC_NAMES: [Option<&'static str>; 82] = [
    None,
    Some("INDENT"),
    Some("DEDENT"),
    Some("BLANK_LINE_FOLLOWING_OPTION"),
    Some("WS"),
    Some("COMMENT"),
    Some("NEWLINE"),
    Some("ID"),
    Some("BODY_START"),
    Some("HEADER_DELIMITER"),
    Some("HASHTAG"),
    Some("REST_OF_LINE"),
    Some("BODY_WS"),
    Some("BODY_END"),
    Some("SHORTCUT_ARROW"),
    Some("COMMAND_START"),
    Some("EXPRESSION_START"),
    Some("ESCAPED_ANY"),
    Some("TEXT_ESCAPE"),
    Some("TEXT_COMMENT"),
    Some("TEXT"),
    Some("UNESCAPABLE_CHARACTER"),
    Some("TEXT_COMMANDHASHTAG_WS"),
    Some("TEXT_COMMANDHASHTAG_COMMENT"),
    Some("TEXT_COMMANDHASHTAG_ERROR"),
    Some("HASHTAG_WS"),
    Some("HASHTAG_TEXT"),
    Some("EXPR_WS"),
    Some("KEYWORD_TRUE"),
    Some("KEYWORD_FALSE"),
    Some("KEYWORD_NULL"),
    Some("OPERATOR_ASSIGNMENT"),
    Some("OPERATOR_LOGICAL_LESS_THAN_EQUALS"),
    Some("OPERATOR_LOGICAL_GREATER_THAN_EQUALS"),
    Some("OPERATOR_LOGICAL_EQUALS"),
    Some("OPERATOR_LOGICAL_LESS"),
    Some("OPERATOR_LOGICAL_GREATER"),
    Some("OPERATOR_LOGICAL_NOT_EQUALS"),
    Some("OPERATOR_LOGICAL_AND"),
    Some("OPERATOR_LOGICAL_OR"),
    Some("OPERATOR_LOGICAL_XOR"),
    Some("OPERATOR_LOGICAL_NOT"),
    Some("OPERATOR_MATHS_ADDITION_EQUALS"),
    Some("OPERATOR_MATHS_SUBTRACTION_EQUALS"),
    Some("OPERATOR_MATHS_MULTIPLICATION_EQUALS"),
    Some("OPERATOR_MATHS_MODULUS_EQUALS"),
    Some("OPERATOR_MATHS_DIVISION_EQUALS"),
    Some("OPERATOR_MATHS_ADDITION"),
    Some("OPERATOR_MATHS_SUBTRACTION"),
    Some("OPERATOR_MATHS_MULTIPLICATION"),
    Some("OPERATOR_MATHS_DIVISION"),
    Some("OPERATOR_MATHS_MODULUS"),
    Some("LPAREN"),
    Some("RPAREN"),
    Some("COMMA"),
    Some("EXPRESSION_AS"),
    Some("STRING"),
    Some("FUNC_ID"),
    Some("EXPRESSION_END"),
    Some("VAR_ID"),
    Some("DOT"),
    Some("NUMBER"),
    Some("COMMAND_WS"),
    Some("COMMAND_IF"),
    Some("COMMAND_ELSEIF"),
    Some("COMMAND_ELSE"),
    Some("COMMAND_SET"),
    Some("COMMAND_ENDIF"),
    Some("COMMAND_CALL"),
    Some("COMMAND_DECLARE"),
    Some("COMMAND_JUMP"),
    Some("COMMAND_ENUM"),
    Some("COMMAND_CASE"),
    Some("COMMAND_ENDENUM"),
    Some("COMMAND_LOCAL"),
    Some("COMMAND_END"),
    Some("COMMAND_TEXT_END"),
    Some("COMMAND_EXPRESSION_START"),
    Some("COMMAND_TEXT"),
    Some("TYPE_STRING"),
    Some("TYPE_NUMBER"),
    Some("TYPE_BOOL"),
];
lazy_static! {
    static ref _shared_context_cache: Arc<PredictionContextCache> =
        Arc::new(PredictionContextCache::new());
    static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(
        _LITERAL_NAMES.iter(),
        _SYMBOLIC_NAMES.iter(),
        None
    ));
}

type BaseParserType<'input, I> = BaseParser<
    'input,
    YarnSpinnerParserExt<'input>,
    I,
    YarnSpinnerParserContextType,
    dyn YarnSpinnerParserListener<'input> + 'input,
>;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type YarnSpinnerParserTreeWalker<'input, 'a> = ParseTreeWalker<
    'input,
    'a,
    YarnSpinnerParserContextType,
    dyn YarnSpinnerParserListener<'input> + 'a,
>;

/// Parser for YarnSpinnerParser grammar
pub struct YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    base: BaseParserType<'input, I>,
    interpreter: Arc<ParserATNSimulator>,
    _shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn get_serialized_atn() -> &'static str {
        _serializedATN
    }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
        antlr_rust::recognizer::check_version("0", "3");
        let interpreter = Arc::new(ParserATNSimulator::new(
            _ATN.clone(),
            _decision_to_DFA.clone(),
            _shared_context_cache.clone(),
        ));
        Self {
            base: BaseParser::new_base_parser(
                input,
                Arc::clone(&interpreter),
                YarnSpinnerParserExt {
                    _pd: Default::default(),
                },
            ),
            interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }
}

type DynStrategy<'input, I> = Box<dyn ErrorStrategy<'input, BaseParserType<'input, I>> + 'input>;

impl<'input, I> YarnSpinnerParser<'input, I, DynStrategy<'input, I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self {
        Self::with_strategy(input, Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I>
    YarnSpinnerParser<'input, I, DefaultErrorStrategy<'input, YarnSpinnerParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    pub fn new(input: I) -> Self {
        Self::with_strategy(input, DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for YarnSpinnerParser
pub trait YarnSpinnerParserContext<'input>:
    for<'x> Listenable<dyn YarnSpinnerParserListener<'input> + 'x>
    + for<'x> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'x>
    + ParserRuleContext<'input, TF = LocalTokenFactory<'input>, Ctx = YarnSpinnerParserContextType>
{
}

antlr_rust::coerce_from! { 'input : YarnSpinnerParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn YarnSpinnerParserContext<'input> + 'input
where
    T: YarnSpinnerParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn YarnSpinnerParserVisitor<'input> + 'x))
    }
}

impl<'input> YarnSpinnerParserContext<'input>
    for TerminalNode<'input, YarnSpinnerParserContextType>
{
}
impl<'input> YarnSpinnerParserContext<'input> for ErrorNode<'input, YarnSpinnerParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn YarnSpinnerParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn YarnSpinnerParserListener<'input> + 'input }

pub struct YarnSpinnerParserContextType;
antlr_rust::tid! {YarnSpinnerParserContextType}

impl<'input> ParserNodeType<'input> for YarnSpinnerParserContextType {
    type TF = LocalTokenFactory<'input>;
    type Type = dyn YarnSpinnerParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    type Target = BaseParserType<'input, I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct YarnSpinnerParserExt<'input> {
    _pd: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserExt<'input> {}
antlr_rust::tid! { YarnSpinnerParserExt<'a> }

impl<'input> TokenAware<'input> for YarnSpinnerParserExt<'input> {
    type TF = LocalTokenFactory<'input>;
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    ParserRecog<'input, BaseParserType<'input, I>> for YarnSpinnerParserExt<'input>
{
}

impl<'input, I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>>
    Actions<'input, BaseParserType<'input, I>> for YarnSpinnerParserExt<'input>
{
    fn get_grammar_file_name(&self) -> &str {
        "YarnSpinnerParser.g4"
    }

    fn get_rule_names(&self) -> &[&str] {
        &ruleNames
    }

    fn get_vocabulary(&self) -> &dyn Vocabulary {
        &**VOCABULARY
    }
    fn sempred(
        _localctx: Option<&(dyn YarnSpinnerParserContext<'input> + 'input)>,
        rule_index: isize,
        pred_index: isize,
        recog: &mut BaseParserType<'input, I>,
    ) -> bool {
        match rule_index {
            10 => YarnSpinnerParser::<'input, I, _>::expression_sempred(
                _localctx.and_then(|x| x.downcast_ref()),
                pred_index,
                recog,
            ),
            _ => true,
        }
    }
}

impl<'input, I>
    YarnSpinnerParser<'input, I, DefaultErrorStrategy<'input, YarnSpinnerParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
{
    fn expression_sempred(
        _localctx: Option<&ExpressionContext<'input>>,
        pred_index: isize,
        recog: &mut <Self as Deref>::Target,
    ) -> bool {
        match pred_index {
            0 => recog.precpred(None, 6),
            1 => recog.precpred(None, 5),
            2 => recog.precpred(None, 4),
            3 => recog.precpred(None, 3),
            4 => recog.precpred(None, 2),
            _ => true,
        }
    }
}
//------------------- dialogue ----------------
pub type DialogueContextAll<'input> = DialogueContext<'input>;

pub type DialogueContext<'input> = BaseParserRuleContext<'input, DialogueContextExt<'input>>;

#[derive(Clone)]
pub struct DialogueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for DialogueContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for DialogueContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_dialogue(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_dialogue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for DialogueContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_dialogue(self);
    }
}

impl<'input> CustomRuleContext<'input> for DialogueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_dialogue
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_dialogue }
}
antlr_rust::tid! {DialogueContextExt<'a>}

impl<'input> DialogueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<DialogueContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            DialogueContextExt { ph: PhantomData },
        ))
    }
}

pub trait DialogueContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<DialogueContextExt<'input>>
{
    fn node_all(&self) -> Vec<Rc<NodeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn node(&self, i: usize) -> Option<Rc<NodeContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn file_hashtag_all(&self) -> Vec<Rc<File_hashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn file_hashtag(&self, i: usize) -> Option<Rc<File_hashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> DialogueContextAttrs<'input> for DialogueContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn dialogue(&mut self) -> Result<Rc<DialogueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = DialogueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_dialogue);
        let mut _localctx: Rc<DialogueContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                {
                    recog.base.set_state(55);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    while _la == HASHTAG {
                        {
                            {
                                /*InvokeRule file_hashtag*/
                                recog.base.set_state(52);
                                recog.file_hashtag()?;
                            }
                        }
                        recog.base.set_state(57);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                    }
                }
                recog.base.set_state(59);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule node*/
                            recog.base.set_state(58);
                            recog.node()?;
                        }
                    }
                    recog.base.set_state(61);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == ID) {
                        break;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- file_hashtag ----------------
pub type File_hashtagContextAll<'input> = File_hashtagContext<'input>;

pub type File_hashtagContext<'input> =
    BaseParserRuleContext<'input, File_hashtagContextExt<'input>>;

#[derive(Clone)]
pub struct File_hashtagContextExt<'input> {
    pub text: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for File_hashtagContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for File_hashtagContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_file_hashtag(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_file_hashtag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for File_hashtagContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_file_hashtag(self);
    }
}

impl<'input> CustomRuleContext<'input> for File_hashtagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_file_hashtag
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_file_hashtag }
}
antlr_rust::tid! {File_hashtagContextExt<'a>}

impl<'input> File_hashtagContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<File_hashtagContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            File_hashtagContextExt {
                text: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait File_hashtagContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<File_hashtagContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HASHTAG
    /// Returns `None` if there is no child corresponding to token HASHTAG
    fn HASHTAG(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HASHTAG, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HASHTAG_TEXT
    /// Returns `None` if there is no child corresponding to token HASHTAG_TEXT
    fn HASHTAG_TEXT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HASHTAG_TEXT, 0)
    }
}

impl<'input> File_hashtagContextAttrs<'input> for File_hashtagContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn file_hashtag(&mut self) -> Result<Rc<File_hashtagContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = File_hashtagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 2, RULE_file_hashtag);
        let mut _localctx: Rc<File_hashtagContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(63);
                recog.base.match_token(HASHTAG, &mut recog.err_handler)?;

                recog.base.set_state(64);
                let tmp = recog
                    .base
                    .match_token(HASHTAG_TEXT, &mut recog.err_handler)?;
                cast_mut::<_, File_hashtagContext>(&mut _localctx).text = Some(tmp.clone());
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- node ----------------
pub type NodeContextAll<'input> = NodeContext<'input>;

pub type NodeContext<'input> = BaseParserRuleContext<'input, NodeContextExt<'input>>;

#[derive(Clone)]
pub struct NodeContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for NodeContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for NodeContext<'input> {
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_node(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_node(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for NodeContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        YarnSpinnerParserVisitor::visit_node(visitor, self);
    }
}

impl<'input> CustomRuleContext<'input> for NodeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_node
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_node }
}
antlr_rust::tid! {NodeContextExt<'a>}

impl<'input> NodeContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<NodeContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            NodeContextExt { ph: PhantomData },
        ))
    }
}

pub trait NodeContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<NodeContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token BODY_START
    /// Returns `None` if there is no child corresponding to token BODY_START
    fn BODY_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BODY_START, 0)
    }
    fn body(&self) -> Option<Rc<BodyContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token BODY_END
    /// Returns `None` if there is no child corresponding to token BODY_END
    fn BODY_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BODY_END, 0)
    }
    fn header_all(&self) -> Vec<Rc<HeaderContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn header(&self, i: usize) -> Option<Rc<HeaderContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> NodeContextAttrs<'input> for NodeContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn node(&mut self) -> Result<Rc<NodeContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = NodeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_node);
        let mut _localctx: Rc<NodeContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(67);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        {
                            /*InvokeRule header*/
                            recog.base.set_state(66);
                            recog.header()?;
                        }
                    }
                    recog.base.set_state(69);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == ID) {
                        break;
                    }
                }
                recog.base.set_state(71);
                recog.base.match_token(BODY_START, &mut recog.err_handler)?;

                /*InvokeRule body*/
                recog.base.set_state(72);
                recog.body()?;

                recog.base.set_state(73);
                recog.base.match_token(BODY_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- header ----------------
pub type HeaderContextAll<'input> = HeaderContext<'input>;

pub type HeaderContext<'input> = BaseParserRuleContext<'input, HeaderContextExt<'input>>;

#[derive(Clone)]
pub struct HeaderContextExt<'input> {
    pub header_key: Option<TokenType<'input>>,
    pub header_value: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for HeaderContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for HeaderContext<'input> {
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_header(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_header(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for HeaderContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_header(self);
    }
}

impl<'input> CustomRuleContext<'input> for HeaderContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_header
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_header }
}
antlr_rust::tid! {HeaderContextExt<'a>}

impl<'input> HeaderContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<HeaderContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            HeaderContextExt {
                header_key: None,
                header_value: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait HeaderContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<HeaderContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HEADER_DELIMITER
    /// Returns `None` if there is no child corresponding to token HEADER_DELIMITER
    fn HEADER_DELIMITER(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HEADER_DELIMITER, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token REST_OF_LINE
    /// Returns `None` if there is no child corresponding to token REST_OF_LINE
    fn REST_OF_LINE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(REST_OF_LINE, 0)
    }
}

impl<'input> HeaderContextAttrs<'input> for HeaderContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn header(&mut self) -> Result<Rc<HeaderContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = HeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_header);
        let mut _localctx: Rc<HeaderContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(75);
                let tmp = recog.base.match_token(ID, &mut recog.err_handler)?;
                cast_mut::<_, HeaderContext>(&mut _localctx).header_key = Some(tmp.clone());

                recog.base.set_state(76);
                recog
                    .base
                    .match_token(HEADER_DELIMITER, &mut recog.err_handler)?;

                recog.base.set_state(78);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == REST_OF_LINE {
                    {
                        recog.base.set_state(77);
                        let tmp = recog
                            .base
                            .match_token(REST_OF_LINE, &mut recog.err_handler)?;
                        cast_mut::<_, HeaderContext>(&mut _localctx).header_value =
                            Some(tmp.clone());
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- body ----------------
pub type BodyContextAll<'input> = BodyContext<'input>;

pub type BodyContext<'input> = BaseParserRuleContext<'input, BodyContextExt<'input>>;

#[derive(Clone)]
pub struct BodyContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for BodyContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for BodyContext<'input> {
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_body(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_body(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for BodyContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_body(self);
    }
}

impl<'input> CustomRuleContext<'input> for BodyContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_body
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_body }
}
antlr_rust::tid! {BodyContextExt<'a>}

impl<'input> BodyContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<BodyContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            BodyContextExt { ph: PhantomData },
        ))
    }
}

pub trait BodyContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<BodyContextExt<'input>>
{
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> BodyContextAttrs<'input> for BodyContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn body(&mut self) -> Result<Rc<BodyContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = BodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_body);
        let mut _localctx: Rc<BodyContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(83);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << INDENT)
                            | (1usize << SHORTCUT_ARROW)
                            | (1usize << COMMAND_START)
                            | (1usize << EXPRESSION_START)
                            | (1usize << TEXT)))
                        != 0)
                {
                    {
                        {
                            /*InvokeRule statement*/
                            recog.base.set_state(80);
                            recog.statement()?;
                        }
                    }
                    recog.base.set_state(85);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;

pub type StatementContext<'input> = BaseParserRuleContext<'input, StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for StatementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for StatementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for StatementContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr_rust::tid! {StatementContextExt<'a>}

impl<'input> StatementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<StatementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            StatementContextExt { ph: PhantomData },
        ))
    }
}

pub trait StatementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<StatementContextExt<'input>>
{
    fn line_statement(&self) -> Option<Rc<Line_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn set_statement(&self) -> Option<Rc<Set_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn shortcut_option_statement(&self) -> Option<Rc<Shortcut_option_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn call_statement(&self) -> Option<Rc<Call_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn command_statement(&self) -> Option<Rc<Command_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn declare_statement(&self) -> Option<Rc<Declare_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn jump_statement(&self) -> Option<Rc<Jump_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INDENT
    /// Returns `None` if there is no child corresponding to token INDENT
    fn INDENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DEDENT
    /// Returns `None` if there is no child corresponding to token DEDENT
    fn DEDENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEDENT, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn statement(&mut self) -> Result<Rc<StatementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(102);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(6, &mut recog.base)? {
                1 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 1);
                    recog.base.enter_outer_alt(None, 1);
                    {
                        /*InvokeRule line_statement*/
                        recog.base.set_state(86);
                        recog.line_statement()?;
                    }
                }
                2 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 2);
                    recog.base.enter_outer_alt(None, 2);
                    {
                        /*InvokeRule if_statement*/
                        recog.base.set_state(87);
                        recog.if_statement()?;
                    }
                }
                3 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 3);
                    recog.base.enter_outer_alt(None, 3);
                    {
                        /*InvokeRule set_statement*/
                        recog.base.set_state(88);
                        recog.set_statement()?;
                    }
                }
                4 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 4);
                    recog.base.enter_outer_alt(None, 4);
                    {
                        /*InvokeRule shortcut_option_statement*/
                        recog.base.set_state(89);
                        recog.shortcut_option_statement()?;
                    }
                }
                5 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 5);
                    recog.base.enter_outer_alt(None, 5);
                    {
                        /*InvokeRule call_statement*/
                        recog.base.set_state(90);
                        recog.call_statement()?;
                    }
                }
                6 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 6);
                    recog.base.enter_outer_alt(None, 6);
                    {
                        /*InvokeRule command_statement*/
                        recog.base.set_state(91);
                        recog.command_statement()?;
                    }
                }
                7 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 7);
                    recog.base.enter_outer_alt(None, 7);
                    {
                        /*InvokeRule declare_statement*/
                        recog.base.set_state(92);
                        recog.declare_statement()?;
                    }
                }
                8 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 8);
                    recog.base.enter_outer_alt(None, 8);
                    {
                        /*InvokeRule jump_statement*/
                        recog.base.set_state(93);
                        recog.jump_statement()?;
                    }
                }
                9 => {
                    //recog.base.enter_outer_alt(_localctx.clone(), 9);
                    recog.base.enter_outer_alt(None, 9);
                    {
                        recog.base.set_state(94);
                        recog.base.match_token(INDENT, &mut recog.err_handler)?;

                        recog.base.set_state(98);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                        while (((_la) & !0x3f) == 0
                            && ((1usize << _la)
                                & ((1usize << INDENT)
                                    | (1usize << SHORTCUT_ARROW)
                                    | (1usize << COMMAND_START)
                                    | (1usize << EXPRESSION_START)
                                    | (1usize << TEXT)))
                                != 0)
                        {
                            {
                                {
                                    /*InvokeRule statement*/
                                    recog.base.set_state(95);
                                    recog.statement()?;
                                }
                            }
                            recog.base.set_state(100);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                        }
                        recog.base.set_state(101);
                        recog.base.match_token(DEDENT, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- line_statement ----------------
pub type Line_statementContextAll<'input> = Line_statementContext<'input>;

pub type Line_statementContext<'input> =
    BaseParserRuleContext<'input, Line_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Line_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Line_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Line_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_line_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_line_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Line_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_line_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Line_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_line_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_line_statement }
}
antlr_rust::tid! {Line_statementContextExt<'a>}

impl<'input> Line_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Line_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Line_statementContextExt { ph: PhantomData },
        ))
    }
}

pub trait Line_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Line_statementContextExt<'input>>
{
    fn line_formatted_text(&self) -> Option<Rc<Line_formatted_textContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token NEWLINE
    /// Returns `None` if there is no child corresponding to token NEWLINE
    fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NEWLINE, 0)
    }
    fn line_condition(&self) -> Option<Rc<Line_conditionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn hashtag_all(&self) -> Vec<Rc<HashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn hashtag(&self, i: usize) -> Option<Rc<HashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Line_statementContextAttrs<'input> for Line_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn line_statement(&mut self) -> Result<Rc<Line_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Line_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 12, RULE_line_statement);
        let mut _localctx: Rc<Line_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule line_formatted_text*/
                recog.base.set_state(104);
                recog.line_formatted_text()?;

                recog.base.set_state(106);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == COMMAND_START {
                    {
                        /*InvokeRule line_condition*/
                        recog.base.set_state(105);
                        recog.line_condition()?;
                    }
                }

                recog.base.set_state(111);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == HASHTAG {
                    {
                        {
                            /*InvokeRule hashtag*/
                            recog.base.set_state(108);
                            recog.hashtag()?;
                        }
                    }
                    recog.base.set_state(113);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(114);
                recog.base.match_token(NEWLINE, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- line_formatted_text ----------------
pub type Line_formatted_textContextAll<'input> = Line_formatted_textContext<'input>;

pub type Line_formatted_textContext<'input> =
    BaseParserRuleContext<'input, Line_formatted_textContextExt<'input>>;

#[derive(Clone)]
pub struct Line_formatted_textContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Line_formatted_textContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Line_formatted_textContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_line_formatted_text(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_line_formatted_text(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Line_formatted_textContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_line_formatted_text(self);
    }
}

impl<'input> CustomRuleContext<'input> for Line_formatted_textContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_line_formatted_text
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_line_formatted_text }
}
antlr_rust::tid! {Line_formatted_textContextExt<'a>}

impl<'input> Line_formatted_textContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Line_formatted_textContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Line_formatted_textContextExt { ph: PhantomData },
        ))
    }
}

pub trait Line_formatted_textContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Line_formatted_textContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_START in current rule
    fn EXPRESSION_START_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_START, starting from 0.
    /// Returns `None` if number of children corresponding to token EXPRESSION_START is less or equal than `i`.
    fn EXPRESSION_START(
        &self,
        i: usize,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_START, i)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_END in current rule
    fn EXPRESSION_END_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_END, starting from 0.
    /// Returns `None` if number of children corresponding to token EXPRESSION_END is less or equal than `i`.
    fn EXPRESSION_END(
        &self,
        i: usize,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_END, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token TEXT in current rule
    fn TEXT_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token TEXT, starting from 0.
    /// Returns `None` if number of children corresponding to token TEXT is less or equal than `i`.
    fn TEXT(&self, i: usize) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(TEXT, i)
    }
}

impl<'input> Line_formatted_textContextAttrs<'input> for Line_formatted_textContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn line_formatted_text(
        &mut self,
    ) -> Result<Rc<Line_formatted_textContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Line_formatted_textContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 14, RULE_line_formatted_text);
        let mut _localctx: Rc<Line_formatted_textContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(125);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                loop {
                    {
                        recog.base.set_state(125);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            TEXT => {
                                recog.base.set_state(117);
                                recog.err_handler.sync(&mut recog.base)?;
                                _alt = 1;
                                loop {
                                    match _alt {
                                        x if x == 1 => {
                                            recog.base.set_state(116);
                                            recog.base.match_token(TEXT, &mut recog.err_handler)?;
                                        }

                                        _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                            &mut recog.base,
                                        )))?,
                                    }
                                    recog.base.set_state(119);
                                    recog.err_handler.sync(&mut recog.base)?;
                                    _alt =
                                        recog.interpreter.adaptive_predict(9, &mut recog.base)?;
                                    if _alt == 2 || _alt == INVALID_ALT {
                                        break;
                                    }
                                }
                            }

                            EXPRESSION_START => {
                                {
                                    recog.base.set_state(121);
                                    recog
                                        .base
                                        .match_token(EXPRESSION_START, &mut recog.err_handler)?;

                                    /*InvokeRule expression*/
                                    recog.base.set_state(122);
                                    recog.expression_rec(0)?;

                                    recog.base.set_state(123);
                                    recog
                                        .base
                                        .match_token(EXPRESSION_END, &mut recog.err_handler)?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                    recog.base.set_state(127);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if !(_la == EXPRESSION_START || _la == TEXT) {
                        break;
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- hashtag ----------------
pub type HashtagContextAll<'input> = HashtagContext<'input>;

pub type HashtagContext<'input> = BaseParserRuleContext<'input, HashtagContextExt<'input>>;

#[derive(Clone)]
pub struct HashtagContextExt<'input> {
    pub text: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for HashtagContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for HashtagContext<'input> {
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_hashtag(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_hashtag(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for HashtagContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_hashtag(self);
    }
}

impl<'input> CustomRuleContext<'input> for HashtagContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_hashtag
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_hashtag }
}
antlr_rust::tid! {HashtagContextExt<'a>}

impl<'input> HashtagContextExt<'input> {
    pub fn new_with_text(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
        text: impl Into<Option<TokenType<'input>>>,
    ) -> Rc<HashtagContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            HashtagContextExt {
                text: text.into(),
                ph: PhantomData,
            },
        ))
    }
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<HashtagContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            HashtagContextExt {
                text: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait HashtagContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<HashtagContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token HASHTAG
    /// Returns `None` if there is no child corresponding to token HASHTAG
    fn HASHTAG(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HASHTAG, 0)
    }
    /// Retrieves first TerminalNode corresponding to token HASHTAG_TEXT
    /// Returns `None` if there is no child corresponding to token HASHTAG_TEXT
    fn HASHTAG_TEXT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(HASHTAG_TEXT, 0)
    }
}

impl<'input> HashtagContextAttrs<'input> for HashtagContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn hashtag(&mut self) -> Result<Rc<HashtagContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = HashtagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_hashtag);
        let mut _localctx: Rc<HashtagContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(129);
                recog.base.match_token(HASHTAG, &mut recog.err_handler)?;

                recog.base.set_state(130);
                let tmp = recog
                    .base
                    .match_token(HASHTAG_TEXT, &mut recog.err_handler)?;
                cast_mut::<_, HashtagContext>(&mut _localctx).text = Some(tmp.clone());
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- line_condition ----------------
pub type Line_conditionContextAll<'input> = Line_conditionContext<'input>;

pub type Line_conditionContext<'input> =
    BaseParserRuleContext<'input, Line_conditionContextExt<'input>>;

#[derive(Clone)]
pub struct Line_conditionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Line_conditionContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Line_conditionContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_line_condition(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_line_condition(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Line_conditionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_line_condition(self);
    }
}

impl<'input> CustomRuleContext<'input> for Line_conditionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_line_condition
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_line_condition }
}
antlr_rust::tid! {Line_conditionContextExt<'a>}

impl<'input> Line_conditionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Line_conditionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Line_conditionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Line_conditionContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Line_conditionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_IF
    /// Returns `None` if there is no child corresponding to token COMMAND_IF
    fn COMMAND_IF(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_IF, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
}

impl<'input> Line_conditionContextAttrs<'input> for Line_conditionContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn line_condition(&mut self) -> Result<Rc<Line_conditionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Line_conditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 18, RULE_line_condition);
        let mut _localctx: Rc<Line_conditionContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(132);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(133);
                recog.base.match_token(COMMAND_IF, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(134);
                recog.expression_rec(0)?;

                recog.base.set_state(135);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input> {
    ExpParensContext(ExpParensContext<'input>),
    ExpMultDivModContext(ExpMultDivModContext<'input>),
    ExpComparisonContext(ExpComparisonContext<'input>),
    ExpNegativeContext(ExpNegativeContext<'input>),
    ExpAndOrXorContext(ExpAndOrXorContext<'input>),
    ExpAddSubContext(ExpAddSubContext<'input>),
    ExpNotContext(ExpNotContext<'input>),
    ExpValueContext(ExpValueContext<'input>),
    ExpEqualityContext(ExpEqualityContext<'input>),
    Error(ExpressionContext<'input>),
}
antlr_rust::tid! {ExpressionContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input> {}

impl<'input> YarnSpinnerParserContext<'input> for ExpressionContextAll<'input> {}

impl<'input> Deref for ExpressionContextAll<'input> {
    type Target = dyn ExpressionContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ExpressionContextAll::*;
        match self {
            ExpParensContext(inner) => inner,
            ExpMultDivModContext(inner) => inner,
            ExpComparisonContext(inner) => inner,
            ExpNegativeContext(inner) => inner,
            ExpAndOrXorContext(inner) => inner,
            ExpAddSubContext(inner) => inner,
            ExpNotContext(inner) => inner,
            ExpValueContext(inner) => inner,
            ExpEqualityContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpressionContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpressionContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ExpressionContext<'input> = BaseParserRuleContext<'input, ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for ExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpressionContext<'input>
{
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpressionContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr_rust::tid! {ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ExpressionContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ExpressionContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>
{
}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input> {}

pub type ExpParensContext<'input> = BaseParserRuleContext<'input, ExpParensContextExt<'input>>;

pub trait ExpParensContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
}

impl<'input> ExpParensContextAttrs<'input> for ExpParensContext<'input> {}

pub struct ExpParensContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpParensContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpParensContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpParensContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expParens(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expParens(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpParensContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expParens(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpParensContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpParensContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpParensContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpParensContext<'input> {}

impl<'input> ExpParensContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpParensContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpParensContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpMultDivModContext<'input> =
    BaseParserRuleContext<'input, ExpMultDivModContextExt<'input>>;

pub trait ExpMultDivModContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MULTIPLICATION
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MULTIPLICATION
    fn OPERATOR_MATHS_MULTIPLICATION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_MULTIPLICATION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_DIVISION
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_DIVISION
    fn OPERATOR_MATHS_DIVISION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_DIVISION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MODULUS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MODULUS
    fn OPERATOR_MATHS_MODULUS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_MODULUS, 0)
    }
}

impl<'input> ExpMultDivModContextAttrs<'input> for ExpMultDivModContext<'input> {}

pub struct ExpMultDivModContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpMultDivModContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpMultDivModContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpMultDivModContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expMultDivMod(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expMultDivMod(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpMultDivModContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expMultDivMod(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpMultDivModContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpMultDivModContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpMultDivModContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpMultDivModContext<'input> {}

impl<'input> ExpMultDivModContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpMultDivModContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpMultDivModContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpComparisonContext<'input> =
    BaseParserRuleContext<'input, ExpComparisonContextExt<'input>>;

pub trait ExpComparisonContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_LESS_THAN_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_LESS_THAN_EQUALS
    fn OPERATOR_LOGICAL_LESS_THAN_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_LESS_THAN_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_GREATER_THAN_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_GREATER_THAN_EQUALS
    fn OPERATOR_LOGICAL_GREATER_THAN_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_GREATER_THAN_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_LESS
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_LESS
    fn OPERATOR_LOGICAL_LESS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_LESS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_GREATER
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_GREATER
    fn OPERATOR_LOGICAL_GREATER(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_GREATER, 0)
    }
}

impl<'input> ExpComparisonContextAttrs<'input> for ExpComparisonContext<'input> {}

pub struct ExpComparisonContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpComparisonContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpComparisonContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpComparisonContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expComparison(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expComparison(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpComparisonContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expComparison(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpComparisonContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpComparisonContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpComparisonContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpComparisonContext<'input> {}

impl<'input> ExpComparisonContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpComparisonContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpComparisonContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpNegativeContext<'input> = BaseParserRuleContext<'input, ExpNegativeContextExt<'input>>;

pub trait ExpNegativeContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION
    fn OPERATOR_MATHS_SUBTRACTION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_SUBTRACTION, 0)
    }
}

impl<'input> ExpNegativeContextAttrs<'input> for ExpNegativeContext<'input> {}

pub struct ExpNegativeContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpNegativeContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpNegativeContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpNegativeContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expNegative(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expNegative(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpNegativeContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expNegative(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpNegativeContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpNegativeContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpNegativeContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpNegativeContext<'input> {}

impl<'input> ExpNegativeContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpNegativeContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpNegativeContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpAndOrXorContext<'input> = BaseParserRuleContext<'input, ExpAndOrXorContextExt<'input>>;

pub trait ExpAndOrXorContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_AND
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_AND
    fn OPERATOR_LOGICAL_AND(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_AND, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_OR
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_OR
    fn OPERATOR_LOGICAL_OR(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_OR, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_XOR
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_XOR
    fn OPERATOR_LOGICAL_XOR(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_XOR, 0)
    }
}

impl<'input> ExpAndOrXorContextAttrs<'input> for ExpAndOrXorContext<'input> {}

pub struct ExpAndOrXorContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpAndOrXorContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpAndOrXorContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpAndOrXorContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expAndOrXor(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expAndOrXor(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpAndOrXorContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expAndOrXor(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpAndOrXorContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpAndOrXorContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpAndOrXorContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpAndOrXorContext<'input> {}

impl<'input> ExpAndOrXorContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpAndOrXorContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpAndOrXorContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpAddSubContext<'input> = BaseParserRuleContext<'input, ExpAddSubContextExt<'input>>;

pub trait ExpAddSubContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_ADDITION
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_ADDITION
    fn OPERATOR_MATHS_ADDITION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_ADDITION, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION
    fn OPERATOR_MATHS_SUBTRACTION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_SUBTRACTION, 0)
    }
}

impl<'input> ExpAddSubContextAttrs<'input> for ExpAddSubContext<'input> {}

pub struct ExpAddSubContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpAddSubContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpAddSubContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpAddSubContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expAddSub(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expAddSub(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpAddSubContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expAddSub(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpAddSubContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpAddSubContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpAddSubContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpAddSubContext<'input> {}

impl<'input> ExpAddSubContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpAddSubContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpAddSubContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpNotContext<'input> = BaseParserRuleContext<'input, ExpNotContextExt<'input>>;

pub trait ExpNotContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_NOT
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_NOT
    fn OPERATOR_LOGICAL_NOT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_NOT, 0)
    }
}

impl<'input> ExpNotContextAttrs<'input> for ExpNotContext<'input> {}

pub struct ExpNotContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpNotContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpNotContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpNotContext<'input> {
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expNot(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expNot(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpNotContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expNot(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpNotContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpNotContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpNotContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpNotContext<'input> {}

impl<'input> ExpNotContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpNotContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpNotContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpValueContext<'input> = BaseParserRuleContext<'input, ExpValueContextExt<'input>>;

pub trait ExpValueContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn value(&self) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ExpValueContextAttrs<'input> for ExpValueContext<'input> {}

pub struct ExpValueContextExt<'input> {
    base: ExpressionContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpValueContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpValueContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpValueContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expValue(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expValue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpValueContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expValue(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpValueContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpValueContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpValueContext<'input> {}

impl<'input> ExpValueContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpValueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpValueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ExpEqualityContext<'input> = BaseParserRuleContext<'input, ExpEqualityContextExt<'input>>;

pub trait ExpEqualityContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_EQUALS
    fn OPERATOR_LOGICAL_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_NOT_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_NOT_EQUALS
    fn OPERATOR_LOGICAL_NOT_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_LOGICAL_NOT_EQUALS, 0)
    }
}

impl<'input> ExpEqualityContextAttrs<'input> for ExpEqualityContext<'input> {}

pub struct ExpEqualityContextExt<'input> {
    base: ExpressionContextExt<'input>,
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ExpEqualityContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpEqualityContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ExpEqualityContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_expEquality(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_expEquality(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ExpEqualityContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_expEquality(self);
    }
}

impl<'input> CustomRuleContext<'input> for ExpEqualityContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_expression
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpEqualityContext<'input> {
    fn borrow(&self) -> &ExpressionContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpEqualityContext<'input> {
    fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ExpressionContextAttrs<'input> for ExpEqualityContext<'input> {}

impl<'input> ExpEqualityContextExt<'input> {
    fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>> {
        Rc::new(ExpressionContextAll::ExpEqualityContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ExpEqualityContextExt {
                    op: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn expression(&mut self) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        self.expression_rec(0)
    }

    fn expression_rec(
        &mut self,
        _p: isize,
    ) -> Result<Rc<ExpressionContextAll<'input>>, ANTLRError> {
        let recog = self;
        let _parentctx = recog.ctx.take();
        let _parentState = recog.base.get_state();
        let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_recursion_rule(_localctx.clone(), 20, RULE_expression, _p);
        let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
        let _startState = 20;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(147);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.base.input.la(1) {
                    LPAREN => {
                        {
                            let mut tmp = ExpParensContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();

                            recog.base.set_state(138);
                            recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(139);
                            recog.expression_rec(0)?;

                            recog.base.set_state(140);
                            recog.base.match_token(RPAREN, &mut recog.err_handler)?;
                        }
                    }

                    OPERATOR_MATHS_SUBTRACTION => {
                        {
                            let mut tmp = ExpNegativeContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(142);
                            let tmp = recog
                                .base
                                .match_token(OPERATOR_MATHS_SUBTRACTION, &mut recog.err_handler)?;
                            if let ExpressionContextAll::ExpNegativeContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.op = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            /*InvokeRule expression*/
                            recog.base.set_state(143);
                            recog.expression_rec(8)?;
                        }
                    }

                    OPERATOR_LOGICAL_NOT => {
                        {
                            let mut tmp = ExpNotContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            recog.base.set_state(144);
                            let tmp = recog
                                .base
                                .match_token(OPERATOR_LOGICAL_NOT, &mut recog.err_handler)?;
                            if let ExpressionContextAll::ExpNotContext(ctx) =
                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                            {
                                ctx.op = Some(tmp.clone());
                            } else {
                                unreachable!("cant cast");
                            }

                            /*InvokeRule expression*/
                            recog.base.set_state(145);
                            recog.expression_rec(7)?;
                        }
                    }

                    KEYWORD_TRUE | KEYWORD_FALSE | KEYWORD_NULL | STRING | FUNC_ID | VAR_ID
                    | NUMBER => {
                        {
                            let mut tmp = ExpValueContextExt::new(&**_localctx);
                            recog.ctx = Some(tmp.clone());
                            _localctx = tmp;
                            _prevctx = _localctx.clone();
                            /*InvokeRule value*/
                            recog.base.set_state(146);
                            recog.value()?;
                        }
                    }

                    _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                        &mut recog.base,
                    )))?,
                }

                let tmp = recog.input.lt(-1).cloned();
                recog.ctx.as_ref().unwrap().set_stop(tmp);
                recog.base.set_state(166);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        recog.trigger_exit_rule_event();
                        _prevctx = _localctx.clone();
                        {
                            recog.base.set_state(164);
                            recog.err_handler.sync(&mut recog.base)?;
                            match recog.interpreter.adaptive_predict(13, &mut recog.base)? {
                                1 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ExpMultDivModContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(149);
                                        if !({ recog.precpred(None, 6) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 6)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(150);
                                        if let ExpressionContextAll::ExpMultDivModContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 49) & !0x3f) == 0
                                                && ((1usize << (_la - 49))
                                                    & ((1usize
                                                        << (OPERATOR_MATHS_MULTIPLICATION - 49))
                                                        | (1usize
                                                            << (OPERATOR_MATHS_DIVISION - 49))
                                                        | (1usize
                                                            << (OPERATOR_MATHS_MODULUS - 49))))
                                                    != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::ExpMultDivModContext(ctx) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(151);
                                        recog.expression_rec(7)?;
                                    }
                                }
                                2 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp =
                                            ExpAddSubContextExt::new(&**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ));
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(152);
                                        if !({ recog.precpred(None, 5) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 5)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(153);
                                        if let ExpressionContextAll::ExpAddSubContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(_la == OPERATOR_MATHS_ADDITION
                                                || _la == OPERATOR_MATHS_SUBTRACTION)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::ExpAddSubContext(ctx) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(154);
                                        recog.expression_rec(6)?;
                                    }
                                }
                                3 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ExpComparisonContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(155);
                                        if !({ recog.precpred(None, 4) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 4)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(156);
                                        if let ExpressionContextAll::ExpComparisonContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 32) & !0x3f) == 0
                                                && ((1usize << (_la - 32))
                                                    & ((1usize
                                                        << (OPERATOR_LOGICAL_LESS_THAN_EQUALS
                                                            - 32))
                                                        | (1usize
                                                            << (OPERATOR_LOGICAL_GREATER_THAN_EQUALS
                                                                - 32))
                                                        | (1usize
                                                            << (OPERATOR_LOGICAL_LESS - 32))
                                                        | (1usize
                                                            << (OPERATOR_LOGICAL_GREATER - 32))))
                                                    != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::ExpComparisonContext(ctx) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(157);
                                        recog.expression_rec(5)?;
                                    }
                                }
                                4 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ExpEqualityContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(158);
                                        if !({ recog.precpred(None, 3) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 3)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(159);
                                        if let ExpressionContextAll::ExpEqualityContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(_la == OPERATOR_LOGICAL_EQUALS
                                                || _la == OPERATOR_LOGICAL_NOT_EQUALS)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::ExpEqualityContext(ctx) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(160);
                                        recog.expression_rec(4)?;
                                    }
                                }
                                5 => {
                                    {
                                        /*recRuleLabeledAltStartAction*/
                                        let mut tmp = ExpAndOrXorContextExt::new(
                                            &**ExpressionContextExt::new(
                                                _parentctx.clone(),
                                                _parentState,
                                            ),
                                        );
                                        recog.push_new_recursion_context(
                                            tmp.clone(),
                                            _startState,
                                            RULE_expression,
                                        );
                                        _localctx = tmp;
                                        recog.base.set_state(161);
                                        if !({ recog.precpred(None, 2) }) {
                                            Err(FailedPredicateError::new(
                                                &mut recog.base,
                                                Some("recog.precpred(None, 2)".to_owned()),
                                                None,
                                            ))?;
                                        }
                                        recog.base.set_state(162);
                                        if let ExpressionContextAll::ExpAndOrXorContext(ctx) =
                                            cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                        {
                                            ctx.op = recog.base.input.lt(1).cloned();
                                        } else {
                                            unreachable!("cant cast");
                                        }
                                        _la = recog.base.input.la(1);
                                        if {
                                            !(((_la - 38) & !0x3f) == 0
                                                && ((1usize << (_la - 38))
                                                    & ((1usize << (OPERATOR_LOGICAL_AND - 38))
                                                        | (1usize << (OPERATOR_LOGICAL_OR - 38))
                                                        | (1usize << (OPERATOR_LOGICAL_XOR - 38))))
                                                    != 0)
                                        } {
                                            let tmp = recog
                                                .err_handler
                                                .recover_inline(&mut recog.base)?;
                                            if let ExpressionContextAll::ExpAndOrXorContext(ctx) =
                                                cast_mut::<_, ExpressionContextAll>(&mut _localctx)
                                            {
                                                ctx.op = Some(tmp.clone());
                                            } else {
                                                unreachable!("cant cast");
                                            }
                                        } else {
                                            if recog.base.input.la(1) == TOKEN_EOF {
                                                recog.base.matched_eof = true
                                            };
                                            recog.err_handler.report_match(&mut recog.base);
                                            recog.base.consume(&mut recog.err_handler);
                                        }
                                        /*InvokeRule expression*/
                                        recog.base.set_state(163);
                                        recog.expression_rec(3)?;
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    recog.base.set_state(168);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(14, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.unroll_recursion_context(_parentctx);

        Ok(_localctx)
    }
}
//------------------- value ----------------
#[derive(Debug)]
pub enum ValueContextAll<'input> {
    ValueNullContext(ValueNullContext<'input>),
    ValueNumberContext(ValueNumberContext<'input>),
    ValueTrueContext(ValueTrueContext<'input>),
    ValueFalseContext(ValueFalseContext<'input>),
    ValueFuncContext(ValueFuncContext<'input>),
    ValueVarContext(ValueVarContext<'input>),
    ValueStringContext(ValueStringContext<'input>),
    Error(ValueContext<'input>),
}
antlr_rust::tid! {ValueContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ValueContextAll<'input> {}

impl<'input> YarnSpinnerParserContext<'input> for ValueContextAll<'input> {}

impl<'input> Deref for ValueContextAll<'input> {
    type Target = dyn ValueContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use ValueContextAll::*;
        match self {
            ValueNullContext(inner) => inner,
            ValueNumberContext(inner) => inner,
            ValueTrueContext(inner) => inner,
            ValueFalseContext(inner) => inner,
            ValueFuncContext(inner) => inner,
            ValueVarContext(inner) => inner,
            ValueStringContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueContextAll<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type ValueContext<'input> = BaseParserRuleContext<'input, ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for ValueContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueContext<'input> {}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueContext<'input> {}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr_rust::tid! {ValueContextExt<'a>}

impl<'input> ValueContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                ValueContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait ValueContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<ValueContextExt<'input>>
{
}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input> {}

pub type ValueNullContext<'input> = BaseParserRuleContext<'input, ValueNullContextExt<'input>>;

pub trait ValueNullContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token KEYWORD_NULL
    /// Returns `None` if there is no child corresponding to token KEYWORD_NULL
    fn KEYWORD_NULL(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KEYWORD_NULL, 0)
    }
}

impl<'input> ValueNullContextAttrs<'input> for ValueNullContext<'input> {}

pub struct ValueNullContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueNullContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueNullContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueNullContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueNull(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueNull(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueNullContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueNull(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueNullContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueNullContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueNullContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueNullContext<'input> {}

impl<'input> ValueNullContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueNullContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueNullContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueNumberContext<'input> = BaseParserRuleContext<'input, ValueNumberContextExt<'input>>;

pub trait ValueNumberContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token NUMBER
    /// Returns `None` if there is no child corresponding to token NUMBER
    fn NUMBER(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(NUMBER, 0)
    }
}

impl<'input> ValueNumberContextAttrs<'input> for ValueNumberContext<'input> {}

pub struct ValueNumberContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueNumberContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueNumberContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueNumberContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueNumber(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueNumber(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ValueNumberContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueNumber(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueNumberContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueNumberContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueNumberContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueNumberContext<'input> {}

impl<'input> ValueNumberContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueNumberContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueNumberContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueTrueContext<'input> = BaseParserRuleContext<'input, ValueTrueContextExt<'input>>;

pub trait ValueTrueContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
    /// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
    fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KEYWORD_TRUE, 0)
    }
}

impl<'input> ValueTrueContextAttrs<'input> for ValueTrueContext<'input> {}

pub struct ValueTrueContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueTrueContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueTrueContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueTrueContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueTrue(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueTrue(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueTrueContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueTrue(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueTrueContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueTrueContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueTrueContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueTrueContext<'input> {}

impl<'input> ValueTrueContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueTrueContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueTrueContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueFalseContext<'input> = BaseParserRuleContext<'input, ValueFalseContextExt<'input>>;

pub trait ValueFalseContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
    /// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
    fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(KEYWORD_FALSE, 0)
    }
}

impl<'input> ValueFalseContextAttrs<'input> for ValueFalseContext<'input> {}

pub struct ValueFalseContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueFalseContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueFalseContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueFalseContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueFalse(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueFalse(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ValueFalseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueFalse(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueFalseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueFalseContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueFalseContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueFalseContext<'input> {}

impl<'input> ValueFalseContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueFalseContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueFalseContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueFuncContext<'input> = BaseParserRuleContext<'input, ValueFuncContextExt<'input>>;

pub trait ValueFuncContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ValueFuncContextAttrs<'input> for ValueFuncContext<'input> {}

pub struct ValueFuncContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueFuncContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueFuncContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueFuncContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueFunc(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueFunc(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueFuncContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueFunc(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueFuncContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueFuncContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueFuncContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueFuncContext<'input> {}

impl<'input> ValueFuncContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueFuncContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueFuncContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueVarContext<'input> = BaseParserRuleContext<'input, ValueVarContextExt<'input>>;

pub trait ValueVarContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    fn variable(&self) -> Option<Rc<VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> ValueVarContextAttrs<'input> for ValueVarContext<'input> {}

pub struct ValueVarContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueVarContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueVarContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueVarContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueVar(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueVar(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueVarContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueVar(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueVarContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueVarContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueVarContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueVarContext<'input> {}

impl<'input> ValueVarContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueVarContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueVarContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type ValueStringContext<'input> = BaseParserRuleContext<'input, ValueStringContextExt<'input>>;

pub trait ValueStringContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token STRING
    /// Returns `None` if there is no child corresponding to token STRING
    fn STRING(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(STRING, 0)
    }
}

impl<'input> ValueStringContextAttrs<'input> for ValueStringContext<'input> {}

pub struct ValueStringContextExt<'input> {
    base: ValueContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {ValueStringContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueStringContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for ValueStringContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_valueString(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_valueString(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for ValueStringContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_valueString(self);
    }
}

impl<'input> CustomRuleContext<'input> for ValueStringContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_value
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueStringContext<'input> {
    fn borrow(&self) -> &ValueContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueStringContext<'input> {
    fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> {
        &mut self.base
    }
}

impl<'input> ValueContextAttrs<'input> for ValueStringContext<'input> {}

impl<'input> ValueStringContextExt<'input> {
    fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>> {
        Rc::new(ValueContextAll::ValueStringContext(
            BaseParserRuleContext::copy_from(
                ctx,
                ValueStringContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn value(&mut self) -> Result<Rc<ValueContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(176);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.base.input.la(1) {
                NUMBER => {
                    let tmp = ValueNumberContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(169);
                        recog.base.match_token(NUMBER, &mut recog.err_handler)?;
                    }
                }

                KEYWORD_TRUE => {
                    let tmp = ValueTrueContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(170);
                        recog
                            .base
                            .match_token(KEYWORD_TRUE, &mut recog.err_handler)?;
                    }
                }

                KEYWORD_FALSE => {
                    let tmp = ValueFalseContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 3);
                    _localctx = tmp;
                    {
                        recog.base.set_state(171);
                        recog
                            .base
                            .match_token(KEYWORD_FALSE, &mut recog.err_handler)?;
                    }
                }

                VAR_ID => {
                    let tmp = ValueVarContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 4);
                    _localctx = tmp;
                    {
                        /*InvokeRule variable*/
                        recog.base.set_state(172);
                        recog.variable()?;
                    }
                }

                STRING => {
                    let tmp = ValueStringContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 5);
                    _localctx = tmp;
                    {
                        recog.base.set_state(173);
                        recog.base.match_token(STRING, &mut recog.err_handler)?;
                    }
                }

                KEYWORD_NULL => {
                    let tmp = ValueNullContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 6);
                    _localctx = tmp;
                    {
                        recog.base.set_state(174);
                        recog
                            .base
                            .match_token(KEYWORD_NULL, &mut recog.err_handler)?;
                    }
                }

                FUNC_ID => {
                    let tmp = ValueFuncContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 7);
                    _localctx = tmp;
                    {
                        /*InvokeRule function_call*/
                        recog.base.set_state(175);
                        recog.function_call()?;
                    }
                }

                _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                    &mut recog.base,
                )))?,
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;

pub type VariableContext<'input> = BaseParserRuleContext<'input, VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for VariableContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for VariableContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_variable(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_variable(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for VariableContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_variable(self);
    }
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_variable
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr_rust::tid! {VariableContextExt<'a>}

impl<'input> VariableContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<VariableContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            VariableContextExt { ph: PhantomData },
        ))
    }
}

pub trait VariableContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<VariableContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token VAR_ID
    /// Returns `None` if there is no child corresponding to token VAR_ID
    fn VAR_ID(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(VAR_ID, 0)
    }
}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn variable(&mut self) -> Result<Rc<VariableContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(178);
                recog.base.match_token(VAR_ID, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- function_call ----------------
pub type Function_callContextAll<'input> = Function_callContext<'input>;

pub type Function_callContext<'input> =
    BaseParserRuleContext<'input, Function_callContextExt<'input>>;

#[derive(Clone)]
pub struct Function_callContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Function_callContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Function_callContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_function_call(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_function_call(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Function_callContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_function_call(self);
    }
}

impl<'input> CustomRuleContext<'input> for Function_callContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_function_call
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_function_call }
}
antlr_rust::tid! {Function_callContextExt<'a>}

impl<'input> Function_callContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Function_callContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Function_callContextExt { ph: PhantomData },
        ))
    }
}

pub trait Function_callContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Function_callContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token FUNC_ID
    /// Returns `None` if there is no child corresponding to token FUNC_ID
    fn FUNC_ID(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FUNC_ID, 0)
    }
    /// Retrieves first TerminalNode corresponding to token LPAREN
    /// Returns `None` if there is no child corresponding to token LPAREN
    fn LPAREN(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(LPAREN, 0)
    }
    /// Retrieves first TerminalNode corresponding to token RPAREN
    /// Returns `None` if there is no child corresponding to token RPAREN
    fn RPAREN(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(RPAREN, 0)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
    fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
    fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMA, i)
    }
}

impl<'input> Function_callContextAttrs<'input> for Function_callContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn function_call(&mut self) -> Result<Rc<Function_callContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Function_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 26, RULE_function_call);
        let mut _localctx: Rc<Function_callContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(180);
                recog.base.match_token(FUNC_ID, &mut recog.err_handler)?;

                recog.base.set_state(181);
                recog.base.match_token(LPAREN, &mut recog.err_handler)?;

                recog.base.set_state(183);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if (((_la) & !0x3f) == 0
                    && ((1usize << _la)
                        & ((1usize << KEYWORD_TRUE)
                            | (1usize << KEYWORD_FALSE)
                            | (1usize << KEYWORD_NULL)))
                        != 0)
                    || (((_la - 41) & !0x3f) == 0
                        && ((1usize << (_la - 41))
                            & ((1usize << (OPERATOR_LOGICAL_NOT - 41))
                                | (1usize << (OPERATOR_MATHS_SUBTRACTION - 41))
                                | (1usize << (LPAREN - 41))
                                | (1usize << (STRING - 41))
                                | (1usize << (FUNC_ID - 41))
                                | (1usize << (VAR_ID - 41))
                                | (1usize << (NUMBER - 41))))
                            != 0)
                {
                    {
                        /*InvokeRule expression*/
                        recog.base.set_state(182);
                        recog.expression_rec(0)?;
                    }
                }

                recog.base.set_state(189);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMA {
                    {
                        {
                            recog.base.set_state(185);
                            recog.base.match_token(COMMA, &mut recog.err_handler)?;

                            /*InvokeRule expression*/
                            recog.base.set_state(186);
                            recog.expression_rec(0)?;
                        }
                    }
                    recog.base.set_state(191);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
                recog.base.set_state(192);
                recog.base.match_token(RPAREN, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- if_statement ----------------
pub type If_statementContextAll<'input> = If_statementContext<'input>;

pub type If_statementContext<'input> =
    BaseParserRuleContext<'input, If_statementContextExt<'input>>;

#[derive(Clone)]
pub struct If_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for If_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for If_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_if_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_if_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for If_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_if_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_if_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr_rust::tid! {If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<If_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            If_statementContextExt { ph: PhantomData },
        ))
    }
}

pub trait If_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<If_statementContextExt<'input>>
{
    fn if_clause(&self) -> Option<Rc<If_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_ENDIF
    /// Returns `None` if there is no child corresponding to token COMMAND_ENDIF
    fn COMMAND_ENDIF(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_ENDIF, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    fn else_if_clause_all(&self) -> Vec<Rc<Else_if_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn else_if_clause(&self, i: usize) -> Option<Rc<Else_if_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    fn else_clause(&self) -> Option<Rc<Else_clauseContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn if_statement(&mut self) -> Result<Rc<If_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 28, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                /*InvokeRule if_clause*/
                recog.base.set_state(194);
                recog.if_clause()?;

                recog.base.set_state(198);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(18, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule else_if_clause*/
                                recog.base.set_state(195);
                                recog.else_if_clause()?;
                            }
                        }
                    }
                    recog.base.set_state(200);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(18, &mut recog.base)?;
                }
                recog.base.set_state(202);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(19, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            /*InvokeRule else_clause*/
                            recog.base.set_state(201);
                            recog.else_clause()?;
                        }
                    }

                    _ => {}
                }
                recog.base.set_state(204);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(205);
                recog
                    .base
                    .match_token(COMMAND_ENDIF, &mut recog.err_handler)?;

                recog.base.set_state(206);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- if_clause ----------------
pub type If_clauseContextAll<'input> = If_clauseContext<'input>;

pub type If_clauseContext<'input> = BaseParserRuleContext<'input, If_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct If_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for If_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for If_clauseContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_if_clause(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_if_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for If_clauseContext<'input> {
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_if_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for If_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_if_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_if_clause }
}
antlr_rust::tid! {If_clauseContextExt<'a>}

impl<'input> If_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<If_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            If_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait If_clauseContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<If_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_IF
    /// Returns `None` if there is no child corresponding to token COMMAND_IF
    fn COMMAND_IF(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_IF, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> If_clauseContextAttrs<'input> for If_clauseContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn if_clause(&mut self) -> Result<Rc<If_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = If_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_if_clause);
        let mut _localctx: Rc<If_clauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(208);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(209);
                recog.base.match_token(COMMAND_IF, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(210);
                recog.expression_rec(0)?;

                recog.base.set_state(211);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;

                recog.base.set_state(215);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(20, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule statement*/
                                recog.base.set_state(212);
                                recog.statement()?;
                            }
                        }
                    }
                    recog.base.set_state(217);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(20, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- else_if_clause ----------------
pub type Else_if_clauseContextAll<'input> = Else_if_clauseContext<'input>;

pub type Else_if_clauseContext<'input> =
    BaseParserRuleContext<'input, Else_if_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Else_if_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Else_if_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Else_if_clauseContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_else_if_clause(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_else_if_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Else_if_clauseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_else_if_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for Else_if_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_else_if_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_else_if_clause }
}
antlr_rust::tid! {Else_if_clauseContextExt<'a>}

impl<'input> Else_if_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Else_if_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Else_if_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Else_if_clauseContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Else_if_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_ELSEIF
    /// Returns `None` if there is no child corresponding to token COMMAND_ELSEIF
    fn COMMAND_ELSEIF(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_ELSEIF, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Else_if_clauseContextAttrs<'input> for Else_if_clauseContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn else_if_clause(&mut self) -> Result<Rc<Else_if_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Else_if_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 32, RULE_else_if_clause);
        let mut _localctx: Rc<Else_if_clauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(218);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(219);
                recog
                    .base
                    .match_token(COMMAND_ELSEIF, &mut recog.err_handler)?;

                /*InvokeRule expression*/
                recog.base.set_state(220);
                recog.expression_rec(0)?;

                recog.base.set_state(221);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;

                recog.base.set_state(225);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(21, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule statement*/
                                recog.base.set_state(222);
                                recog.statement()?;
                            }
                        }
                    }
                    recog.base.set_state(227);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(21, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- else_clause ----------------
pub type Else_clauseContextAll<'input> = Else_clauseContext<'input>;

pub type Else_clauseContext<'input> = BaseParserRuleContext<'input, Else_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Else_clauseContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Else_clauseContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Else_clauseContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_else_clause(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_else_clause(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Else_clauseContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_else_clause(self);
    }
}

impl<'input> CustomRuleContext<'input> for Else_clauseContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_else_clause
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_else_clause }
}
antlr_rust::tid! {Else_clauseContextExt<'a>}

impl<'input> Else_clauseContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Else_clauseContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Else_clauseContextExt { ph: PhantomData },
        ))
    }
}

pub trait Else_clauseContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Else_clauseContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_ELSE
    /// Returns `None` if there is no child corresponding to token COMMAND_ELSE
    fn COMMAND_ELSE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_ELSE, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Else_clauseContextAttrs<'input> for Else_clauseContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn else_clause(&mut self) -> Result<Rc<Else_clauseContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx = Else_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 34, RULE_else_clause);
        let mut _localctx: Rc<Else_clauseContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(228);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(229);
                recog
                    .base
                    .match_token(COMMAND_ELSE, &mut recog.err_handler)?;

                recog.base.set_state(230);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;

                recog.base.set_state(234);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(22, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule statement*/
                                recog.base.set_state(231);
                                recog.statement()?;
                            }
                        }
                    }
                    recog.base.set_state(236);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(22, &mut recog.base)?;
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- set_statement ----------------
pub type Set_statementContextAll<'input> = Set_statementContext<'input>;

pub type Set_statementContext<'input> =
    BaseParserRuleContext<'input, Set_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Set_statementContextExt<'input> {
    pub op: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Set_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Set_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_set_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_set_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Set_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_set_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Set_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_set_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_set_statement }
}
antlr_rust::tid! {Set_statementContextExt<'a>}

impl<'input> Set_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Set_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Set_statementContextExt {
                op: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait Set_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Set_statementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_SET
    /// Returns `None` if there is no child corresponding to token COMMAND_SET
    fn COMMAND_SET(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_SET, 0)
    }
    fn variable(&self) -> Option<Rc<VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_ASSIGNMENT
    /// Returns `None` if there is no child corresponding to token OPERATOR_ASSIGNMENT
    fn OPERATOR_ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_ASSIGNMENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MULTIPLICATION_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MULTIPLICATION_EQUALS
    fn OPERATOR_MATHS_MULTIPLICATION_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_MULTIPLICATION_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_DIVISION_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_DIVISION_EQUALS
    fn OPERATOR_MATHS_DIVISION_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_DIVISION_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MODULUS_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MODULUS_EQUALS
    fn OPERATOR_MATHS_MODULUS_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_MODULUS_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_ADDITION_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_ADDITION_EQUALS
    fn OPERATOR_MATHS_ADDITION_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_ADDITION_EQUALS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION_EQUALS
    /// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION_EQUALS
    fn OPERATOR_MATHS_SUBTRACTION_EQUALS(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_MATHS_SUBTRACTION_EQUALS, 0)
    }
}

impl<'input> Set_statementContextAttrs<'input> for Set_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn set_statement(&mut self) -> Result<Rc<Set_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Set_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 36, RULE_set_statement);
        let mut _localctx: Rc<Set_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(237);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(238);
                recog
                    .base
                    .match_token(COMMAND_SET, &mut recog.err_handler)?;

                /*InvokeRule variable*/
                recog.base.set_state(239);
                recog.variable()?;

                recog.base.set_state(240);
                cast_mut::<_, Set_statementContext>(&mut _localctx).op =
                    recog.base.input.lt(1).cloned();

                _la = recog.base.input.la(1);
                if {
                    !(((_la - 31) & !0x3f) == 0
                        && ((1usize << (_la - 31))
                            & ((1usize << (OPERATOR_ASSIGNMENT - 31))
                                | (1usize << (OPERATOR_MATHS_ADDITION_EQUALS - 31))
                                | (1usize << (OPERATOR_MATHS_SUBTRACTION_EQUALS - 31))
                                | (1usize << (OPERATOR_MATHS_MULTIPLICATION_EQUALS - 31))
                                | (1usize << (OPERATOR_MATHS_MODULUS_EQUALS - 31))
                                | (1usize << (OPERATOR_MATHS_DIVISION_EQUALS - 31))))
                            != 0)
                } {
                    let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
                    cast_mut::<_, Set_statementContext>(&mut _localctx).op = Some(tmp.clone());
                } else {
                    if recog.base.input.la(1) == TOKEN_EOF {
                        recog.base.matched_eof = true
                    };
                    recog.err_handler.report_match(&mut recog.base);
                    recog.base.consume(&mut recog.err_handler);
                }
                /*InvokeRule expression*/
                recog.base.set_state(241);
                recog.expression_rec(0)?;

                recog.base.set_state(242);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- call_statement ----------------
pub type Call_statementContextAll<'input> = Call_statementContext<'input>;

pub type Call_statementContext<'input> =
    BaseParserRuleContext<'input, Call_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Call_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Call_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Call_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_call_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_call_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Call_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_call_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Call_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_call_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_call_statement }
}
antlr_rust::tid! {Call_statementContextExt<'a>}

impl<'input> Call_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Call_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Call_statementContextExt { ph: PhantomData },
        ))
    }
}

pub trait Call_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Call_statementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_CALL
    /// Returns `None` if there is no child corresponding to token COMMAND_CALL
    fn COMMAND_CALL(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_CALL, 0)
    }
    fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
}

impl<'input> Call_statementContextAttrs<'input> for Call_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn call_statement(&mut self) -> Result<Rc<Call_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Call_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 38, RULE_call_statement);
        let mut _localctx: Rc<Call_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(244);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(245);
                recog
                    .base
                    .match_token(COMMAND_CALL, &mut recog.err_handler)?;

                /*InvokeRule function_call*/
                recog.base.set_state(246);
                recog.function_call()?;

                recog.base.set_state(247);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- command_statement ----------------
pub type Command_statementContextAll<'input> = Command_statementContext<'input>;

pub type Command_statementContext<'input> =
    BaseParserRuleContext<'input, Command_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Command_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Command_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Command_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_command_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_command_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Command_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_command_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Command_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_command_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_command_statement }
}
antlr_rust::tid! {Command_statementContextExt<'a>}

impl<'input> Command_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Command_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Command_statementContextExt { ph: PhantomData },
        ))
    }
}

pub trait Command_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Command_statementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    fn command_formatted_text(&self) -> Option<Rc<Command_formatted_textContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_TEXT_END
    /// Returns `None` if there is no child corresponding to token COMMAND_TEXT_END
    fn COMMAND_TEXT_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_TEXT_END, 0)
    }
    fn hashtag_all(&self) -> Vec<Rc<HashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn hashtag(&self, i: usize) -> Option<Rc<HashtagContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Command_statementContextAttrs<'input> for Command_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn command_statement(
        &mut self,
    ) -> Result<Rc<Command_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Command_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 40, RULE_command_statement);
        let mut _localctx: Rc<Command_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(249);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                /*InvokeRule command_formatted_text*/
                recog.base.set_state(250);
                recog.command_formatted_text()?;

                recog.base.set_state(251);
                recog
                    .base
                    .match_token(COMMAND_TEXT_END, &mut recog.err_handler)?;

                {
                    recog.base.set_state(255);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    while _la == HASHTAG {
                        {
                            {
                                /*InvokeRule hashtag*/
                                recog.base.set_state(252);
                                recog.hashtag()?;
                            }
                        }
                        recog.base.set_state(257);
                        recog.err_handler.sync(&mut recog.base)?;
                        _la = recog.base.input.la(1);
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- command_formatted_text ----------------
pub type Command_formatted_textContextAll<'input> = Command_formatted_textContext<'input>;

pub type Command_formatted_textContext<'input> =
    BaseParserRuleContext<'input, Command_formatted_textContextExt<'input>>;

#[derive(Clone)]
pub struct Command_formatted_textContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Command_formatted_textContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Command_formatted_textContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_command_formatted_text(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_command_formatted_text(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Command_formatted_textContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_command_formatted_text(self);
    }
}

impl<'input> CustomRuleContext<'input> for Command_formatted_textContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_command_formatted_text
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_command_formatted_text }
}
antlr_rust::tid! {Command_formatted_textContextExt<'a>}

impl<'input> Command_formatted_textContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Command_formatted_textContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Command_formatted_textContextExt { ph: PhantomData },
        ))
    }
}

pub trait Command_formatted_textContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Command_formatted_textContextExt<'input>>
{
    /// Retrieves all `TerminalNode`s corresponding to token COMMAND_TEXT in current rule
    fn COMMAND_TEXT_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMAND_TEXT, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMAND_TEXT is less or equal than `i`.
    fn COMMAND_TEXT(
        &self,
        i: usize,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_TEXT, i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token COMMAND_EXPRESSION_START in current rule
    fn COMMAND_EXPRESSION_START_all(
        &self,
    ) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token COMMAND_EXPRESSION_START, starting from 0.
    /// Returns `None` if number of children corresponding to token COMMAND_EXPRESSION_START is less or equal than `i`.
    fn COMMAND_EXPRESSION_START(
        &self,
        i: usize,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_EXPRESSION_START, i)
    }
    fn expression_all(&self) -> Vec<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_END in current rule
    fn EXPRESSION_END_all(&self) -> Vec<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    /// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_END, starting from 0.
    /// Returns `None` if number of children corresponding to token EXPRESSION_END is less or equal than `i`.
    fn EXPRESSION_END(
        &self,
        i: usize,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_END, i)
    }
}

impl<'input> Command_formatted_textContextAttrs<'input> for Command_formatted_textContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn command_formatted_text(
        &mut self,
    ) -> Result<Rc<Command_formatted_textContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Command_formatted_textContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 42, RULE_command_formatted_text);
        let mut _localctx: Rc<Command_formatted_textContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(265);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                while _la == COMMAND_EXPRESSION_START || _la == COMMAND_TEXT {
                    {
                        recog.base.set_state(263);
                        recog.err_handler.sync(&mut recog.base)?;
                        match recog.base.input.la(1) {
                            COMMAND_TEXT => {
                                recog.base.set_state(258);
                                recog
                                    .base
                                    .match_token(COMMAND_TEXT, &mut recog.err_handler)?;
                            }

                            COMMAND_EXPRESSION_START => {
                                {
                                    recog.base.set_state(259);
                                    recog.base.match_token(
                                        COMMAND_EXPRESSION_START,
                                        &mut recog.err_handler,
                                    )?;

                                    /*InvokeRule expression*/
                                    recog.base.set_state(260);
                                    recog.expression_rec(0)?;

                                    recog.base.set_state(261);
                                    recog
                                        .base
                                        .match_token(EXPRESSION_END, &mut recog.err_handler)?;
                                }
                            }

                            _ => Err(ANTLRError::NoAltError(NoViableAltError::new(
                                &mut recog.base,
                            )))?,
                        }
                    }
                    recog.base.set_state(267);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- shortcut_option_statement ----------------
pub type Shortcut_option_statementContextAll<'input> = Shortcut_option_statementContext<'input>;

pub type Shortcut_option_statementContext<'input> =
    BaseParserRuleContext<'input, Shortcut_option_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Shortcut_option_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Shortcut_option_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Shortcut_option_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_shortcut_option_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_shortcut_option_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Shortcut_option_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_shortcut_option_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Shortcut_option_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_shortcut_option_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_shortcut_option_statement }
}
antlr_rust::tid! {Shortcut_option_statementContextExt<'a>}

impl<'input> Shortcut_option_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Shortcut_option_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Shortcut_option_statementContextExt { ph: PhantomData },
        ))
    }
}

pub trait Shortcut_option_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Shortcut_option_statementContextExt<'input>>
{
    fn shortcut_option_all(&self) -> Vec<Rc<Shortcut_optionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn shortcut_option(&self, i: usize) -> Option<Rc<Shortcut_optionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
    /// Retrieves first TerminalNode corresponding to token BLANK_LINE_FOLLOWING_OPTION
    /// Returns `None` if there is no child corresponding to token BLANK_LINE_FOLLOWING_OPTION
    fn BLANK_LINE_FOLLOWING_OPTION(
        &self,
    ) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(BLANK_LINE_FOLLOWING_OPTION, 0)
    }
}

impl<'input> Shortcut_option_statementContextAttrs<'input>
    for Shortcut_option_statementContext<'input>
{
}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn shortcut_option_statement(
        &mut self,
    ) -> Result<Rc<Shortcut_option_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Shortcut_option_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 44, RULE_shortcut_option_statement);
        let mut _localctx: Rc<Shortcut_option_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            let mut _alt: isize;
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(271);
                recog.err_handler.sync(&mut recog.base)?;
                _alt = recog.interpreter.adaptive_predict(26, &mut recog.base)?;
                while { _alt != 2 && _alt != INVALID_ALT } {
                    if _alt == 1 {
                        {
                            {
                                /*InvokeRule shortcut_option*/
                                recog.base.set_state(268);
                                recog.shortcut_option()?;
                            }
                        }
                    }
                    recog.base.set_state(273);
                    recog.err_handler.sync(&mut recog.base)?;
                    _alt = recog.interpreter.adaptive_predict(26, &mut recog.base)?;
                }
                {
                    /*InvokeRule shortcut_option*/
                    recog.base.set_state(274);
                    recog.shortcut_option()?;

                    recog.base.set_state(276);
                    recog.err_handler.sync(&mut recog.base)?;
                    _la = recog.base.input.la(1);
                    if _la == BLANK_LINE_FOLLOWING_OPTION {
                        {
                            recog.base.set_state(275);
                            recog
                                .base
                                .match_token(BLANK_LINE_FOLLOWING_OPTION, &mut recog.err_handler)?;
                        }
                    }
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- shortcut_option ----------------
pub type Shortcut_optionContextAll<'input> = Shortcut_optionContext<'input>;

pub type Shortcut_optionContext<'input> =
    BaseParserRuleContext<'input, Shortcut_optionContextExt<'input>>;

#[derive(Clone)]
pub struct Shortcut_optionContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Shortcut_optionContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Shortcut_optionContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_shortcut_option(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_shortcut_option(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Shortcut_optionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_shortcut_option(self);
    }
}

impl<'input> CustomRuleContext<'input> for Shortcut_optionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_shortcut_option
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_shortcut_option }
}
antlr_rust::tid! {Shortcut_optionContextExt<'a>}

impl<'input> Shortcut_optionContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Shortcut_optionContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Shortcut_optionContextExt { ph: PhantomData },
        ))
    }
}

pub trait Shortcut_optionContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Shortcut_optionContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token SHORTCUT_ARROW
    /// Returns `None` if there is no child corresponding to token SHORTCUT_ARROW
    fn SHORTCUT_ARROW(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(SHORTCUT_ARROW, 0)
    }
    fn line_statement(&self) -> Option<Rc<Line_statementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token INDENT
    /// Returns `None` if there is no child corresponding to token INDENT
    fn INDENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(INDENT, 0)
    }
    /// Retrieves first TerminalNode corresponding to token DEDENT
    /// Returns `None` if there is no child corresponding to token DEDENT
    fn DEDENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(DEDENT, 0)
    }
    fn statement_all(&self) -> Vec<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.children_of_type()
    }
    fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(i)
    }
}

impl<'input> Shortcut_optionContextAttrs<'input> for Shortcut_optionContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn shortcut_option(&mut self) -> Result<Rc<Shortcut_optionContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Shortcut_optionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 46, RULE_shortcut_option);
        let mut _localctx: Rc<Shortcut_optionContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(278);
                recog
                    .base
                    .match_token(SHORTCUT_ARROW, &mut recog.err_handler)?;

                /*InvokeRule line_statement*/
                recog.base.set_state(279);
                recog.line_statement()?;

                recog.base.set_state(288);
                recog.err_handler.sync(&mut recog.base)?;
                match recog.interpreter.adaptive_predict(29, &mut recog.base)? {
                    x if x == 1 => {
                        {
                            recog.base.set_state(280);
                            recog.base.match_token(INDENT, &mut recog.err_handler)?;

                            recog.base.set_state(284);
                            recog.err_handler.sync(&mut recog.base)?;
                            _la = recog.base.input.la(1);
                            while (((_la) & !0x3f) == 0
                                && ((1usize << _la)
                                    & ((1usize << INDENT)
                                        | (1usize << SHORTCUT_ARROW)
                                        | (1usize << COMMAND_START)
                                        | (1usize << EXPRESSION_START)
                                        | (1usize << TEXT)))
                                    != 0)
                            {
                                {
                                    {
                                        /*InvokeRule statement*/
                                        recog.base.set_state(281);
                                        recog.statement()?;
                                    }
                                }
                                recog.base.set_state(286);
                                recog.err_handler.sync(&mut recog.base)?;
                                _la = recog.base.input.la(1);
                            }
                            recog.base.set_state(287);
                            recog.base.match_token(DEDENT, &mut recog.err_handler)?;
                        }
                    }

                    _ => {}
                }
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- declare_statement ----------------
pub type Declare_statementContextAll<'input> = Declare_statementContext<'input>;

pub type Declare_statementContext<'input> =
    BaseParserRuleContext<'input, Declare_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Declare_statementContextExt<'input> {
    pub declaration_type: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Declare_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Declare_statementContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_declare_statement(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_declare_statement(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Declare_statementContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_declare_statement(self);
    }
}

impl<'input> CustomRuleContext<'input> for Declare_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_declare_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_declare_statement }
}
antlr_rust::tid! {Declare_statementContextExt<'a>}

impl<'input> Declare_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Declare_statementContextAll<'input>> {
        Rc::new(BaseParserRuleContext::new_parser_ctx(
            parent,
            invoking_state,
            Declare_statementContextExt {
                declaration_type: None,
                ph: PhantomData,
            },
        ))
    }
}

pub trait Declare_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Declare_statementContextExt<'input>>
{
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_DECLARE
    /// Returns `None` if there is no child corresponding to token COMMAND_DECLARE
    fn COMMAND_DECLARE(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_DECLARE, 0)
    }
    fn variable(&self) -> Option<Rc<VariableContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token OPERATOR_ASSIGNMENT
    /// Returns `None` if there is no child corresponding to token OPERATOR_ASSIGNMENT
    fn OPERATOR_ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(OPERATOR_ASSIGNMENT, 0)
    }
    fn value(&self) -> Option<Rc<ValueContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EXPRESSION_AS
    /// Returns `None` if there is no child corresponding to token EXPRESSION_AS
    fn EXPRESSION_AS(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_AS, 0)
    }
    /// Retrieves first TerminalNode corresponding to token FUNC_ID
    /// Returns `None` if there is no child corresponding to token FUNC_ID
    fn FUNC_ID(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(FUNC_ID, 0)
    }
}

impl<'input> Declare_statementContextAttrs<'input> for Declare_statementContext<'input> {}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn declare_statement(
        &mut self,
    ) -> Result<Rc<Declare_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Declare_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 48, RULE_declare_statement);
        let mut _localctx: Rc<Declare_statementContextAll> = _localctx;
        let mut _la: isize = -1;
        let result: Result<(), ANTLRError> = (|| {
            //recog.base.enter_outer_alt(_localctx.clone(), 1);
            recog.base.enter_outer_alt(None, 1);
            {
                recog.base.set_state(290);
                recog
                    .base
                    .match_token(COMMAND_START, &mut recog.err_handler)?;

                recog.base.set_state(291);
                recog
                    .base
                    .match_token(COMMAND_DECLARE, &mut recog.err_handler)?;

                /*InvokeRule variable*/
                recog.base.set_state(292);
                recog.variable()?;

                recog.base.set_state(293);
                recog
                    .base
                    .match_token(OPERATOR_ASSIGNMENT, &mut recog.err_handler)?;

                /*InvokeRule value*/
                recog.base.set_state(294);
                recog.value()?;

                recog.base.set_state(297);
                recog.err_handler.sync(&mut recog.base)?;
                _la = recog.base.input.la(1);
                if _la == EXPRESSION_AS {
                    {
                        recog.base.set_state(295);
                        recog
                            .base
                            .match_token(EXPRESSION_AS, &mut recog.err_handler)?;

                        recog.base.set_state(296);
                        let tmp = recog.base.match_token(FUNC_ID, &mut recog.err_handler)?;
                        cast_mut::<_, Declare_statementContext>(&mut _localctx).declaration_type =
                            Some(tmp.clone());
                    }
                }

                recog.base.set_state(299);
                recog
                    .base
                    .match_token(COMMAND_END, &mut recog.err_handler)?;
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}
//------------------- jump_statement ----------------
#[derive(Debug)]
pub enum Jump_statementContextAll<'input> {
    JumpToNodeNameContext(JumpToNodeNameContext<'input>),
    JumpToExpressionContext(JumpToExpressionContext<'input>),
    Error(Jump_statementContext<'input>),
}
antlr_rust::tid! {Jump_statementContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for Jump_statementContextAll<'input> {}

impl<'input> YarnSpinnerParserContext<'input> for Jump_statementContextAll<'input> {}

impl<'input> Deref for Jump_statementContextAll<'input> {
    type Target = dyn Jump_statementContextAttrs<'input> + 'input;
    fn deref(&self) -> &Self::Target {
        use Jump_statementContextAll::*;
        match self {
            JumpToNodeNameContext(inner) => inner,
            JumpToExpressionContext(inner) => inner,
            Error(inner) => inner,
        }
    }
}
impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Jump_statementContextAll<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        self.deref().accept(visitor)
    }
}
impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Jump_statementContextAll<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().enter(listener)
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        self.deref().exit(listener)
    }
}

pub type Jump_statementContext<'input> =
    BaseParserRuleContext<'input, Jump_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Jump_statementContextExt<'input> {
    ph: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserContext<'input> for Jump_statementContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for Jump_statementContext<'input>
{
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for Jump_statementContext<'input>
{
}

impl<'input> CustomRuleContext<'input> for Jump_statementContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_jump_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}
antlr_rust::tid! {Jump_statementContextExt<'a>}

impl<'input> Jump_statementContextExt<'input> {
    fn new(
        parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
        invoking_state: isize,
    ) -> Rc<Jump_statementContextAll<'input>> {
        Rc::new(Jump_statementContextAll::Error(
            BaseParserRuleContext::new_parser_ctx(
                parent,
                invoking_state,
                Jump_statementContextExt { ph: PhantomData },
            ),
        ))
    }
}

pub trait Jump_statementContextAttrs<'input>:
    YarnSpinnerParserContext<'input> + BorrowMut<Jump_statementContextExt<'input>>
{
}

impl<'input> Jump_statementContextAttrs<'input> for Jump_statementContext<'input> {}

pub type JumpToNodeNameContext<'input> =
    BaseParserRuleContext<'input, JumpToNodeNameContextExt<'input>>;

pub trait JumpToNodeNameContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_JUMP
    /// Returns `None` if there is no child corresponding to token COMMAND_JUMP
    fn COMMAND_JUMP(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_JUMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token ID
    /// Returns `None` if there is no child corresponding to token ID
    fn ID(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(ID, 0)
    }
}

impl<'input> JumpToNodeNameContextAttrs<'input> for JumpToNodeNameContext<'input> {}

pub struct JumpToNodeNameContextExt<'input> {
    base: Jump_statementContextExt<'input>,
    pub destination: Option<TokenType<'input>>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {JumpToNodeNameContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for JumpToNodeNameContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for JumpToNodeNameContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_jumpToNodeName(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_jumpToNodeName(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for JumpToNodeNameContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_jumpToNodeName(self);
    }
}

impl<'input> CustomRuleContext<'input> for JumpToNodeNameContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_jump_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}

impl<'input> Borrow<Jump_statementContextExt<'input>> for JumpToNodeNameContext<'input> {
    fn borrow(&self) -> &Jump_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Jump_statementContextExt<'input>> for JumpToNodeNameContext<'input> {
    fn borrow_mut(&mut self) -> &mut Jump_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Jump_statementContextAttrs<'input> for JumpToNodeNameContext<'input> {}

impl<'input> JumpToNodeNameContextExt<'input> {
    fn new(ctx: &dyn Jump_statementContextAttrs<'input>) -> Rc<Jump_statementContextAll<'input>> {
        Rc::new(Jump_statementContextAll::JumpToNodeNameContext(
            BaseParserRuleContext::copy_from(
                ctx,
                JumpToNodeNameContextExt {
                    destination: None,
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

pub type JumpToExpressionContext<'input> =
    BaseParserRuleContext<'input, JumpToExpressionContextExt<'input>>;

pub trait JumpToExpressionContextAttrs<'input>: YarnSpinnerParserContext<'input> {
    /// Retrieves first TerminalNode corresponding to token COMMAND_START
    /// Returns `None` if there is no child corresponding to token COMMAND_START
    fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_START, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_JUMP
    /// Returns `None` if there is no child corresponding to token COMMAND_JUMP
    fn COMMAND_JUMP(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_JUMP, 0)
    }
    /// Retrieves first TerminalNode corresponding to token EXPRESSION_START
    /// Returns `None` if there is no child corresponding to token EXPRESSION_START
    fn EXPRESSION_START(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_START, 0)
    }
    fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>>
    where
        Self: Sized,
    {
        self.child_of_type(0)
    }
    /// Retrieves first TerminalNode corresponding to token EXPRESSION_END
    /// Returns `None` if there is no child corresponding to token EXPRESSION_END
    fn EXPRESSION_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(EXPRESSION_END, 0)
    }
    /// Retrieves first TerminalNode corresponding to token COMMAND_END
    /// Returns `None` if there is no child corresponding to token COMMAND_END
    fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input, YarnSpinnerParserContextType>>>
    where
        Self: Sized,
    {
        self.get_token(COMMAND_END, 0)
    }
}

impl<'input> JumpToExpressionContextAttrs<'input> for JumpToExpressionContext<'input> {}

pub struct JumpToExpressionContextExt<'input> {
    base: Jump_statementContextExt<'input>,
    ph: PhantomData<&'input str>,
}

antlr_rust::tid! {JumpToExpressionContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for JumpToExpressionContext<'input> {}

impl<'input, 'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a>
    for JumpToExpressionContext<'input>
{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.enter_every_rule(self);
        listener.enter_jumpToExpression(self);
    }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) {
        listener.exit_jumpToExpression(self);
        listener.exit_every_rule(self);
    }
}

impl<'input, 'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a>
    for JumpToExpressionContext<'input>
{
    fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
        visitor.visit_jumpToExpression(self);
    }
}

impl<'input> CustomRuleContext<'input> for JumpToExpressionContextExt<'input> {
    type TF = LocalTokenFactory<'input>;
    type Ctx = YarnSpinnerParserContextType;
    fn get_rule_index(&self) -> usize {
        RULE_jump_statement
    }
    //fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}

impl<'input> Borrow<Jump_statementContextExt<'input>> for JumpToExpressionContext<'input> {
    fn borrow(&self) -> &Jump_statementContextExt<'input> {
        &self.base
    }
}
impl<'input> BorrowMut<Jump_statementContextExt<'input>> for JumpToExpressionContext<'input> {
    fn borrow_mut(&mut self) -> &mut Jump_statementContextExt<'input> {
        &mut self.base
    }
}

impl<'input> Jump_statementContextAttrs<'input> for JumpToExpressionContext<'input> {}

impl<'input> JumpToExpressionContextExt<'input> {
    fn new(ctx: &dyn Jump_statementContextAttrs<'input>) -> Rc<Jump_statementContextAll<'input>> {
        Rc::new(Jump_statementContextAll::JumpToExpressionContext(
            BaseParserRuleContext::copy_from(
                ctx,
                JumpToExpressionContextExt {
                    base: ctx.borrow().clone(),
                    ph: PhantomData,
                },
            ),
        ))
    }
}

impl<'input, I, H> YarnSpinnerParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input>> + TidAble<'input>,
    H: ErrorStrategy<'input, BaseParserType<'input, I>>,
{
    pub fn jump_statement(&mut self) -> Result<Rc<Jump_statementContextAll<'input>>, ANTLRError> {
        let mut recog = self;
        let _parentctx = recog.ctx.take();
        let mut _localctx =
            Jump_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog
            .base
            .enter_rule(_localctx.clone(), 50, RULE_jump_statement);
        let mut _localctx: Rc<Jump_statementContextAll> = _localctx;
        let result: Result<(), ANTLRError> = (|| {
            recog.base.set_state(312);
            recog.err_handler.sync(&mut recog.base)?;
            match recog.interpreter.adaptive_predict(31, &mut recog.base)? {
                1 => {
                    let tmp = JumpToNodeNameContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 1);
                    _localctx = tmp;
                    {
                        recog.base.set_state(301);
                        recog
                            .base
                            .match_token(COMMAND_START, &mut recog.err_handler)?;

                        recog.base.set_state(302);
                        recog
                            .base
                            .match_token(COMMAND_JUMP, &mut recog.err_handler)?;

                        recog.base.set_state(303);
                        let tmp = recog.base.match_token(ID, &mut recog.err_handler)?;
                        if let Jump_statementContextAll::JumpToNodeNameContext(ctx) =
                            cast_mut::<_, Jump_statementContextAll>(&mut _localctx)
                        {
                            ctx.destination = Some(tmp.clone());
                        } else {
                            unreachable!("cant cast");
                        }

                        recog.base.set_state(304);
                        recog
                            .base
                            .match_token(COMMAND_END, &mut recog.err_handler)?;
                    }
                }
                2 => {
                    let tmp = JumpToExpressionContextExt::new(&**_localctx);
                    recog.base.enter_outer_alt(Some(tmp.clone()), 2);
                    _localctx = tmp;
                    {
                        recog.base.set_state(305);
                        recog
                            .base
                            .match_token(COMMAND_START, &mut recog.err_handler)?;

                        recog.base.set_state(306);
                        recog
                            .base
                            .match_token(COMMAND_JUMP, &mut recog.err_handler)?;

                        recog.base.set_state(307);
                        recog
                            .base
                            .match_token(EXPRESSION_START, &mut recog.err_handler)?;

                        /*InvokeRule expression*/
                        recog.base.set_state(308);
                        recog.expression_rec(0)?;

                        recog.base.set_state(309);
                        recog
                            .base
                            .match_token(EXPRESSION_END, &mut recog.err_handler)?;

                        recog.base.set_state(310);
                        recog
                            .base
                            .match_token(COMMAND_END, &mut recog.err_handler)?;
                    }
                }

                _ => {}
            }
            Ok(())
        })();
        match result {
            Ok(_) => {}
            Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
            Err(ref re) => {
                //_localctx.exception = re;
                recog.err_handler.report_error(&mut recog.base, re);
                recog.err_handler.recover(&mut recog.base, re)?;
            }
        }
        recog.base.exit_rule();

        Ok(_localctx)
    }
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(_ATN.clone(), _ATN.get_decision_state(i), i as isize).into())
        }
        Arc::new(dfa)
    };
}

const _serializedATN: &'static str =
    "\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x53\u{13d}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x03\
	\x02\x07\x02\x38\x0a\x02\x0c\x02\x0e\x02\x3b\x0b\x02\x03\x02\x06\x02\x3e\
	\x0a\x02\x0d\x02\x0e\x02\x3f\x03\x03\x03\x03\x03\x03\x03\x04\x06\x04\x46\
	\x0a\x04\x0d\x04\x0e\x04\x47\x03\x04\x03\x04\x03\x04\x03\x04\x03\x05\x03\
	\x05\x03\x05\x05\x05\x51\x0a\x05\x03\x06\x07\x06\x54\x0a\x06\x0c\x06\x0e\
	\x06\x57\x0b\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x07\x07\x63\x0a\x07\x0c\x07\x0e\x07\x66\x0b\x07\
	\x03\x07\x05\x07\x69\x0a\x07\x03\x08\x03\x08\x05\x08\x6d\x0a\x08\x03\x08\
	\x07\x08\x70\x0a\x08\x0c\x08\x0e\x08\x73\x0b\x08\x03\x08\x03\x08\x03\x09\
	\x06\x09\x78\x0a\x09\x0d\x09\x0e\x09\x79\x03\x09\x03\x09\x03\x09\x03\x09\
	\x06\x09\u{80}\x0a\x09\x0d\x09\x0e\x09\u{81}\x03\x0a\x03\x0a\x03\x0a\x03\
	\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\
	\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{96}\x0a\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x07\x0c\u{a7}\x0a\x0c\x0c\x0c\x0e\
	\x0c\u{aa}\x0b\x0c\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\x03\x0d\
	\x05\x0d\u{b3}\x0a\x0d\x03\x0e\x03\x0e\x03\x0f\x03\x0f\x03\x0f\x05\x0f\u{ba}\
	\x0a\x0f\x03\x0f\x03\x0f\x07\x0f\u{be}\x0a\x0f\x0c\x0f\x0e\x0f\u{c1}\x0b\
	\x0f\x03\x0f\x03\x0f\x03\x10\x03\x10\x07\x10\u{c7}\x0a\x10\x0c\x10\x0e\x10\
	\u{ca}\x0b\x10\x03\x10\x05\x10\u{cd}\x0a\x10\x03\x10\x03\x10\x03\x10\x03\
	\x10\x03\x11\x03\x11\x03\x11\x03\x11\x03\x11\x07\x11\u{d8}\x0a\x11\x0c\x11\
	\x0e\x11\u{db}\x0b\x11\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{e2}\
	\x0a\x12\x0c\x12\x0e\x12\u{e5}\x0b\x12\x03\x13\x03\x13\x03\x13\x03\x13\x07\
	\x13\u{eb}\x0a\x13\x0c\x13\x0e\x13\u{ee}\x0b\x13\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x03\x14\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\
	\x03\x16\x03\x16\x03\x16\x03\x16\x07\x16\u{100}\x0a\x16\x0c\x16\x0e\x16\
	\u{103}\x0b\x16\x03\x17\x03\x17\x03\x17\x03\x17\x03\x17\x07\x17\u{10a}\x0a\
	\x17\x0c\x17\x0e\x17\u{10d}\x0b\x17\x03\x18\x07\x18\u{110}\x0a\x18\x0c\x18\
	\x0e\x18\u{113}\x0b\x18\x03\x18\x03\x18\x05\x18\u{117}\x0a\x18\x03\x19\x03\
	\x19\x03\x19\x03\x19\x07\x19\u{11d}\x0a\x19\x0c\x19\x0e\x19\u{120}\x0b\x19\
	\x03\x19\x05\x19\u{123}\x0a\x19\x03\x1a\x03\x1a\x03\x1a\x03\x1a\x03\x1a\
	\x03\x1a\x03\x1a\x05\x1a\u{12c}\x0a\x1a\x03\x1a\x03\x1a\x03\x1b\x03\x1b\
	\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\x03\x1b\
	\x05\x1b\u{13b}\x0a\x1b\x03\x1b\x02\x03\x16\x1c\x02\x04\x06\x08\x0a\x0c\
	\x0e\x10\x12\x14\x16\x18\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\
	\x32\x34\x02\x08\x03\x02\x33\x35\x03\x02\x31\x32\x04\x02\x22\x23\x25\x26\
	\x04\x02\x24\x24\x27\x27\x03\x02\x28\x2a\x04\x02\x21\x21\x2c\x30\x02\u{153}\
	\x02\x39\x03\x02\x02\x02\x04\x41\x03\x02\x02\x02\x06\x45\x03\x02\x02\x02\
	\x08\x4d\x03\x02\x02\x02\x0a\x55\x03\x02\x02\x02\x0c\x68\x03\x02\x02\x02\
	\x0e\x6a\x03\x02\x02\x02\x10\x7f\x03\x02\x02\x02\x12\u{83}\x03\x02\x02\x02\
	\x14\u{86}\x03\x02\x02\x02\x16\u{95}\x03\x02\x02\x02\x18\u{b2}\x03\x02\x02\
	\x02\x1a\u{b4}\x03\x02\x02\x02\x1c\u{b6}\x03\x02\x02\x02\x1e\u{c4}\x03\x02\
	\x02\x02\x20\u{d2}\x03\x02\x02\x02\x22\u{dc}\x03\x02\x02\x02\x24\u{e6}\x03\
	\x02\x02\x02\x26\u{ef}\x03\x02\x02\x02\x28\u{f6}\x03\x02\x02\x02\x2a\u{fb}\
	\x03\x02\x02\x02\x2c\u{10b}\x03\x02\x02\x02\x2e\u{111}\x03\x02\x02\x02\x30\
	\u{118}\x03\x02\x02\x02\x32\u{124}\x03\x02\x02\x02\x34\u{13a}\x03\x02\x02\
	\x02\x36\x38\x05\x04\x03\x02\x37\x36\x03\x02\x02\x02\x38\x3b\x03\x02\x02\
	\x02\x39\x37\x03\x02\x02\x02\x39\x3a\x03\x02\x02\x02\x3a\x3d\x03\x02\x02\
	\x02\x3b\x39\x03\x02\x02\x02\x3c\x3e\x05\x06\x04\x02\x3d\x3c\x03\x02\x02\
	\x02\x3e\x3f\x03\x02\x02\x02\x3f\x3d\x03\x02\x02\x02\x3f\x40\x03\x02\x02\
	\x02\x40\x03\x03\x02\x02\x02\x41\x42\x07\x0c\x02\x02\x42\x43\x07\x1c\x02\
	\x02\x43\x05\x03\x02\x02\x02\x44\x46\x05\x08\x05\x02\x45\x44\x03\x02\x02\
	\x02\x46\x47\x03\x02\x02\x02\x47\x45\x03\x02\x02\x02\x47\x48\x03\x02\x02\
	\x02\x48\x49\x03\x02\x02\x02\x49\x4a\x07\x0a\x02\x02\x4a\x4b\x05\x0a\x06\
	\x02\x4b\x4c\x07\x0f\x02\x02\x4c\x07\x03\x02\x02\x02\x4d\x4e\x07\x09\x02\
	\x02\x4e\x50\x07\x0b\x02\x02\x4f\x51\x07\x0d\x02\x02\x50\x4f\x03\x02\x02\
	\x02\x50\x51\x03\x02\x02\x02\x51\x09\x03\x02\x02\x02\x52\x54\x05\x0c\x07\
	\x02\x53\x52\x03\x02\x02\x02\x54\x57\x03\x02\x02\x02\x55\x53\x03\x02\x02\
	\x02\x55\x56\x03\x02\x02\x02\x56\x0b\x03\x02\x02\x02\x57\x55\x03\x02\x02\
	\x02\x58\x69\x05\x0e\x08\x02\x59\x69\x05\x1e\x10\x02\x5a\x69\x05\x26\x14\
	\x02\x5b\x69\x05\x2e\x18\x02\x5c\x69\x05\x28\x15\x02\x5d\x69\x05\x2a\x16\
	\x02\x5e\x69\x05\x32\x1a\x02\x5f\x69\x05\x34\x1b\x02\x60\x64\x07\x03\x02\
	\x02\x61\x63\x05\x0c\x07\x02\x62\x61\x03\x02\x02\x02\x63\x66\x03\x02\x02\
	\x02\x64\x62\x03\x02\x02\x02\x64\x65\x03\x02\x02\x02\x65\x67\x03\x02\x02\
	\x02\x66\x64\x03\x02\x02\x02\x67\x69\x07\x04\x02\x02\x68\x58\x03\x02\x02\
	\x02\x68\x59\x03\x02\x02\x02\x68\x5a\x03\x02\x02\x02\x68\x5b\x03\x02\x02\
	\x02\x68\x5c\x03\x02\x02\x02\x68\x5d\x03\x02\x02\x02\x68\x5e\x03\x02\x02\
	\x02\x68\x5f\x03\x02\x02\x02\x68\x60\x03\x02\x02\x02\x69\x0d\x03\x02\x02\
	\x02\x6a\x6c\x05\x10\x09\x02\x6b\x6d\x05\x14\x0b\x02\x6c\x6b\x03\x02\x02\
	\x02\x6c\x6d\x03\x02\x02\x02\x6d\x71\x03\x02\x02\x02\x6e\x70\x05\x12\x0a\
	\x02\x6f\x6e\x03\x02\x02\x02\x70\x73\x03\x02\x02\x02\x71\x6f\x03\x02\x02\
	\x02\x71\x72\x03\x02\x02\x02\x72\x74\x03\x02\x02\x02\x73\x71\x03\x02\x02\
	\x02\x74\x75\x07\x08\x02\x02\x75\x0f\x03\x02\x02\x02\x76\x78\x07\x16\x02\
	\x02\x77\x76\x03\x02\x02\x02\x78\x79\x03\x02\x02\x02\x79\x77\x03\x02\x02\
	\x02\x79\x7a\x03\x02\x02\x02\x7a\u{80}\x03\x02\x02\x02\x7b\x7c\x07\x12\x02\
	\x02\x7c\x7d\x05\x16\x0c\x02\x7d\x7e\x07\x3c\x02\x02\x7e\u{80}\x03\x02\x02\
	\x02\x7f\x77\x03\x02\x02\x02\x7f\x7b\x03\x02\x02\x02\u{80}\u{81}\x03\x02\
	\x02\x02\u{81}\x7f\x03\x02\x02\x02\u{81}\u{82}\x03\x02\x02\x02\u{82}\x11\
	\x03\x02\x02\x02\u{83}\u{84}\x07\x0c\x02\x02\u{84}\u{85}\x07\x1c\x02\x02\
	\u{85}\x13\x03\x02\x02\x02\u{86}\u{87}\x07\x11\x02\x02\u{87}\u{88}\x07\x41\
	\x02\x02\u{88}\u{89}\x05\x16\x0c\x02\u{89}\u{8a}\x07\x4d\x02\x02\u{8a}\x15\
	\x03\x02\x02\x02\u{8b}\u{8c}\x08\x0c\x01\x02\u{8c}\u{8d}\x07\x36\x02\x02\
	\u{8d}\u{8e}\x05\x16\x0c\x02\u{8e}\u{8f}\x07\x37\x02\x02\u{8f}\u{96}\x03\
	\x02\x02\x02\u{90}\u{91}\x07\x32\x02\x02\u{91}\u{96}\x05\x16\x0c\x0a\u{92}\
	\u{93}\x07\x2b\x02\x02\u{93}\u{96}\x05\x16\x0c\x09\u{94}\u{96}\x05\x18\x0d\
	\x02\u{95}\u{8b}\x03\x02\x02\x02\u{95}\u{90}\x03\x02\x02\x02\u{95}\u{92}\
	\x03\x02\x02\x02\u{95}\u{94}\x03\x02\x02\x02\u{96}\u{a8}\x03\x02\x02\x02\
	\u{97}\u{98}\x0c\x08\x02\x02\u{98}\u{99}\x09\x02\x02\x02\u{99}\u{a7}\x05\
	\x16\x0c\x09\u{9a}\u{9b}\x0c\x07\x02\x02\u{9b}\u{9c}\x09\x03\x02\x02\u{9c}\
	\u{a7}\x05\x16\x0c\x08\u{9d}\u{9e}\x0c\x06\x02\x02\u{9e}\u{9f}\x09\x04\x02\
	\x02\u{9f}\u{a7}\x05\x16\x0c\x07\u{a0}\u{a1}\x0c\x05\x02\x02\u{a1}\u{a2}\
	\x09\x05\x02\x02\u{a2}\u{a7}\x05\x16\x0c\x06\u{a3}\u{a4}\x0c\x04\x02\x02\
	\u{a4}\u{a5}\x09\x06\x02\x02\u{a5}\u{a7}\x05\x16\x0c\x05\u{a6}\u{97}\x03\
	\x02\x02\x02\u{a6}\u{9a}\x03\x02\x02\x02\u{a6}\u{9d}\x03\x02\x02\x02\u{a6}\
	\u{a0}\x03\x02\x02\x02\u{a6}\u{a3}\x03\x02\x02\x02\u{a7}\u{aa}\x03\x02\x02\
	\x02\u{a8}\u{a6}\x03\x02\x02\x02\u{a8}\u{a9}\x03\x02\x02\x02\u{a9}\x17\x03\
	\x02\x02\x02\u{aa}\u{a8}\x03\x02\x02\x02\u{ab}\u{b3}\x07\x3f\x02\x02\u{ac}\
	\u{b3}\x07\x1e\x02\x02\u{ad}\u{b3}\x07\x1f\x02\x02\u{ae}\u{b3}\x05\x1a\x0e\
	\x02\u{af}\u{b3}\x07\x3a\x02\x02\u{b0}\u{b3}\x07\x20\x02\x02\u{b1}\u{b3}\
	\x05\x1c\x0f\x02\u{b2}\u{ab}\x03\x02\x02\x02\u{b2}\u{ac}\x03\x02\x02\x02\
	\u{b2}\u{ad}\x03\x02\x02\x02\u{b2}\u{ae}\x03\x02\x02\x02\u{b2}\u{af}\x03\
	\x02\x02\x02\u{b2}\u{b0}\x03\x02\x02\x02\u{b2}\u{b1}\x03\x02\x02\x02\u{b3}\
	\x19\x03\x02\x02\x02\u{b4}\u{b5}\x07\x3d\x02\x02\u{b5}\x1b\x03\x02\x02\x02\
	\u{b6}\u{b7}\x07\x3b\x02\x02\u{b7}\u{b9}\x07\x36\x02\x02\u{b8}\u{ba}\x05\
	\x16\x0c\x02\u{b9}\u{b8}\x03\x02\x02\x02\u{b9}\u{ba}\x03\x02\x02\x02\u{ba}\
	\u{bf}\x03\x02\x02\x02\u{bb}\u{bc}\x07\x38\x02\x02\u{bc}\u{be}\x05\x16\x0c\
	\x02\u{bd}\u{bb}\x03\x02\x02\x02\u{be}\u{c1}\x03\x02\x02\x02\u{bf}\u{bd}\
	\x03\x02\x02\x02\u{bf}\u{c0}\x03\x02\x02\x02\u{c0}\u{c2}\x03\x02\x02\x02\
	\u{c1}\u{bf}\x03\x02\x02\x02\u{c2}\u{c3}\x07\x37\x02\x02\u{c3}\x1d\x03\x02\
	\x02\x02\u{c4}\u{c8}\x05\x20\x11\x02\u{c5}\u{c7}\x05\x22\x12\x02\u{c6}\u{c5}\
	\x03\x02\x02\x02\u{c7}\u{ca}\x03\x02\x02\x02\u{c8}\u{c6}\x03\x02\x02\x02\
	\u{c8}\u{c9}\x03\x02\x02\x02\u{c9}\u{cc}\x03\x02\x02\x02\u{ca}\u{c8}\x03\
	\x02\x02\x02\u{cb}\u{cd}\x05\x24\x13\x02\u{cc}\u{cb}\x03\x02\x02\x02\u{cc}\
	\u{cd}\x03\x02\x02\x02\u{cd}\u{ce}\x03\x02\x02\x02\u{ce}\u{cf}\x07\x11\x02\
	\x02\u{cf}\u{d0}\x07\x45\x02\x02\u{d0}\u{d1}\x07\x4d\x02\x02\u{d1}\x1f\x03\
	\x02\x02\x02\u{d2}\u{d3}\x07\x11\x02\x02\u{d3}\u{d4}\x07\x41\x02\x02\u{d4}\
	\u{d5}\x05\x16\x0c\x02\u{d5}\u{d9}\x07\x4d\x02\x02\u{d6}\u{d8}\x05\x0c\x07\
	\x02\u{d7}\u{d6}\x03\x02\x02\x02\u{d8}\u{db}\x03\x02\x02\x02\u{d9}\u{d7}\
	\x03\x02\x02\x02\u{d9}\u{da}\x03\x02\x02\x02\u{da}\x21\x03\x02\x02\x02\u{db}\
	\u{d9}\x03\x02\x02\x02\u{dc}\u{dd}\x07\x11\x02\x02\u{dd}\u{de}\x07\x42\x02\
	\x02\u{de}\u{df}\x05\x16\x0c\x02\u{df}\u{e3}\x07\x4d\x02\x02\u{e0}\u{e2}\
	\x05\x0c\x07\x02\u{e1}\u{e0}\x03\x02\x02\x02\u{e2}\u{e5}\x03\x02\x02\x02\
	\u{e3}\u{e1}\x03\x02\x02\x02\u{e3}\u{e4}\x03\x02\x02\x02\u{e4}\x23\x03\x02\
	\x02\x02\u{e5}\u{e3}\x03\x02\x02\x02\u{e6}\u{e7}\x07\x11\x02\x02\u{e7}\u{e8}\
	\x07\x43\x02\x02\u{e8}\u{ec}\x07\x4d\x02\x02\u{e9}\u{eb}\x05\x0c\x07\x02\
	\u{ea}\u{e9}\x03\x02\x02\x02\u{eb}\u{ee}\x03\x02\x02\x02\u{ec}\u{ea}\x03\
	\x02\x02\x02\u{ec}\u{ed}\x03\x02\x02\x02\u{ed}\x25\x03\x02\x02\x02\u{ee}\
	\u{ec}\x03\x02\x02\x02\u{ef}\u{f0}\x07\x11\x02\x02\u{f0}\u{f1}\x07\x44\x02\
	\x02\u{f1}\u{f2}\x05\x1a\x0e\x02\u{f2}\u{f3}\x09\x07\x02\x02\u{f3}\u{f4}\
	\x05\x16\x0c\x02\u{f4}\u{f5}\x07\x4d\x02\x02\u{f5}\x27\x03\x02\x02\x02\u{f6}\
	\u{f7}\x07\x11\x02\x02\u{f7}\u{f8}\x07\x46\x02\x02\u{f8}\u{f9}\x05\x1c\x0f\
	\x02\u{f9}\u{fa}\x07\x4d\x02\x02\u{fa}\x29\x03\x02\x02\x02\u{fb}\u{fc}\x07\
	\x11\x02\x02\u{fc}\u{fd}\x05\x2c\x17\x02\u{fd}\u{101}\x07\x4e\x02\x02\u{fe}\
	\u{100}\x05\x12\x0a\x02\u{ff}\u{fe}\x03\x02\x02\x02\u{100}\u{103}\x03\x02\
	\x02\x02\u{101}\u{ff}\x03\x02\x02\x02\u{101}\u{102}\x03\x02\x02\x02\u{102}\
	\x2b\x03\x02\x02\x02\u{103}\u{101}\x03\x02\x02\x02\u{104}\u{10a}\x07\x50\
	\x02\x02\u{105}\u{106}\x07\x4f\x02\x02\u{106}\u{107}\x05\x16\x0c\x02\u{107}\
	\u{108}\x07\x3c\x02\x02\u{108}\u{10a}\x03\x02\x02\x02\u{109}\u{104}\x03\
	\x02\x02\x02\u{109}\u{105}\x03\x02\x02\x02\u{10a}\u{10d}\x03\x02\x02\x02\
	\u{10b}\u{109}\x03\x02\x02\x02\u{10b}\u{10c}\x03\x02\x02\x02\u{10c}\x2d\
	\x03\x02\x02\x02\u{10d}\u{10b}\x03\x02\x02\x02\u{10e}\u{110}\x05\x30\x19\
	\x02\u{10f}\u{10e}\x03\x02\x02\x02\u{110}\u{113}\x03\x02\x02\x02\u{111}\
	\u{10f}\x03\x02\x02\x02\u{111}\u{112}\x03\x02\x02\x02\u{112}\u{114}\x03\
	\x02\x02\x02\u{113}\u{111}\x03\x02\x02\x02\u{114}\u{116}\x05\x30\x19\x02\
	\u{115}\u{117}\x07\x05\x02\x02\u{116}\u{115}\x03\x02\x02\x02\u{116}\u{117}\
	\x03\x02\x02\x02\u{117}\x2f\x03\x02\x02\x02\u{118}\u{119}\x07\x10\x02\x02\
	\u{119}\u{122}\x05\x0e\x08\x02\u{11a}\u{11e}\x07\x03\x02\x02\u{11b}\u{11d}\
	\x05\x0c\x07\x02\u{11c}\u{11b}\x03\x02\x02\x02\u{11d}\u{120}\x03\x02\x02\
	\x02\u{11e}\u{11c}\x03\x02\x02\x02\u{11e}\u{11f}\x03\x02\x02\x02\u{11f}\
	\u{121}\x03\x02\x02\x02\u{120}\u{11e}\x03\x02\x02\x02\u{121}\u{123}\x07\
	\x04\x02\x02\u{122}\u{11a}\x03\x02\x02\x02\u{122}\u{123}\x03\x02\x02\x02\
	\u{123}\x31\x03\x02\x02\x02\u{124}\u{125}\x07\x11\x02\x02\u{125}\u{126}\
	\x07\x47\x02\x02\u{126}\u{127}\x05\x1a\x0e\x02\u{127}\u{128}\x07\x21\x02\
	\x02\u{128}\u{12b}\x05\x18\x0d\x02\u{129}\u{12a}\x07\x39\x02\x02\u{12a}\
	\u{12c}\x07\x3b\x02\x02\u{12b}\u{129}\x03\x02\x02\x02\u{12b}\u{12c}\x03\
	\x02\x02\x02\u{12c}\u{12d}\x03\x02\x02\x02\u{12d}\u{12e}\x07\x4d\x02\x02\
	\u{12e}\x33\x03\x02\x02\x02\u{12f}\u{130}\x07\x11\x02\x02\u{130}\u{131}\
	\x07\x48\x02\x02\u{131}\u{132}\x07\x09\x02\x02\u{132}\u{13b}\x07\x4d\x02\
	\x02\u{133}\u{134}\x07\x11\x02\x02\u{134}\u{135}\x07\x48\x02\x02\u{135}\
	\u{136}\x07\x12\x02\x02\u{136}\u{137}\x05\x16\x0c\x02\u{137}\u{138}\x07\
	\x3c\x02\x02\u{138}\u{139}\x07\x4d\x02\x02\u{139}\u{13b}\x03\x02\x02\x02\
	\u{13a}\u{12f}\x03\x02\x02\x02\u{13a}\u{133}\x03\x02\x02\x02\u{13b}\x35\
	\x03\x02\x02\x02\x22\x39\x3f\x47\x50\x55\x64\x68\x6c\x71\x79\x7f\u{81}\u{95}\
	\u{a6}\u{a8}\u{b2}\u{b9}\u{bf}\u{c8}\u{cc}\u{d9}\u{e3}\u{ec}\u{101}\u{109}\
	\u{10b}\u{111}\u{116}\u{11e}\u{122}\u{12b}\u{13a}";
