// Generated from third-party/YarnSpinner/YarnSpinner.Compiler/YarnSpinnerParser.g4 by ANTLR 4.13.2
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]
use antlr4rust::PredictionContextCache;
use antlr4rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr4rust::token_stream::TokenStream;
use antlr4rust::TokenSource;
use antlr4rust::parser_atn_simulator::ParserATNSimulator;
use antlr4rust::errors::*;
use antlr4rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr4rust::recognizer::{Recognizer,Actions};
use antlr4rust::atn_deserializer::ATNDeserializer;
use antlr4rust::dfa::DFA;
use antlr4rust::atn::{ATN, INVALID_ALT};
use antlr4rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr4rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr4rust::tree::*;
use antlr4rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr4rust::int_stream::EOF;
use antlr4rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr4rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::yarnspinnerparserlistener::*;
use super::yarnspinnerparservisitor::*;

use antlr4rust::lazy_static;
use antlr4rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const YarnSpinnerParser_INDENT:i32=1; 
		pub const YarnSpinnerParser_DEDENT:i32=2; 
		pub const YarnSpinnerParser_BLANK_LINE_FOLLOWING_OPTION:i32=3; 
		pub const YarnSpinnerParser_WS:i32=4; 
		pub const YarnSpinnerParser_COMMENT:i32=5; 
		pub const YarnSpinnerParser_NEWLINE:i32=6; 
		pub const YarnSpinnerParser_ID:i32=7; 
		pub const YarnSpinnerParser_BODY_START:i32=8; 
		pub const YarnSpinnerParser_HEADER_DELIMITER:i32=9; 
		pub const YarnSpinnerParser_HASHTAG:i32=10; 
		pub const YarnSpinnerParser_REST_OF_LINE:i32=11; 
		pub const YarnSpinnerParser_BODY_WS:i32=12; 
		pub const YarnSpinnerParser_BODY_END:i32=13; 
		pub const YarnSpinnerParser_SHORTCUT_ARROW:i32=14; 
		pub const YarnSpinnerParser_COMMAND_START:i32=15; 
		pub const YarnSpinnerParser_EXPRESSION_START:i32=16; 
		pub const YarnSpinnerParser_ESCAPED_ANY:i32=17; 
		pub const YarnSpinnerParser_TEXT_ESCAPE:i32=18; 
		pub const YarnSpinnerParser_TEXT_COMMENT:i32=19; 
		pub const YarnSpinnerParser_TEXT:i32=20; 
		pub const YarnSpinnerParser_UNESCAPABLE_CHARACTER:i32=21; 
		pub const YarnSpinnerParser_TEXT_COMMANDHASHTAG_WS:i32=22; 
		pub const YarnSpinnerParser_TEXT_COMMANDHASHTAG_COMMENT:i32=23; 
		pub const YarnSpinnerParser_TEXT_COMMANDHASHTAG_ERROR:i32=24; 
		pub const YarnSpinnerParser_HASHTAG_WS:i32=25; 
		pub const YarnSpinnerParser_HASHTAG_TEXT:i32=26; 
		pub const YarnSpinnerParser_EXPR_WS:i32=27; 
		pub const YarnSpinnerParser_KEYWORD_TRUE:i32=28; 
		pub const YarnSpinnerParser_KEYWORD_FALSE:i32=29; 
		pub const YarnSpinnerParser_KEYWORD_NULL:i32=30; 
		pub const YarnSpinnerParser_OPERATOR_ASSIGNMENT:i32=31; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_LESS_THAN_EQUALS:i32=32; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_GREATER_THAN_EQUALS:i32=33; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_EQUALS:i32=34; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_LESS:i32=35; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_GREATER:i32=36; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_NOT_EQUALS:i32=37; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_AND:i32=38; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_OR:i32=39; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_XOR:i32=40; 
		pub const YarnSpinnerParser_OPERATOR_LOGICAL_NOT:i32=41; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_ADDITION_EQUALS:i32=42; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION_EQUALS:i32=43; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_MULTIPLICATION_EQUALS:i32=44; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_MODULUS_EQUALS:i32=45; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_DIVISION_EQUALS:i32=46; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_ADDITION:i32=47; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION:i32=48; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_MULTIPLICATION:i32=49; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_DIVISION:i32=50; 
		pub const YarnSpinnerParser_OPERATOR_MATHS_MODULUS:i32=51; 
		pub const YarnSpinnerParser_LPAREN:i32=52; 
		pub const YarnSpinnerParser_RPAREN:i32=53; 
		pub const YarnSpinnerParser_COMMA:i32=54; 
		pub const YarnSpinnerParser_EXPRESSION_AS:i32=55; 
		pub const YarnSpinnerParser_STRING:i32=56; 
		pub const YarnSpinnerParser_FUNC_ID:i32=57; 
		pub const YarnSpinnerParser_EXPRESSION_END:i32=58; 
		pub const YarnSpinnerParser_VAR_ID:i32=59; 
		pub const YarnSpinnerParser_DOT:i32=60; 
		pub const YarnSpinnerParser_NUMBER:i32=61; 
		pub const YarnSpinnerParser_COMMAND_NEWLINE:i32=62; 
		pub const YarnSpinnerParser_COMMAND_WS:i32=63; 
		pub const YarnSpinnerParser_COMMAND_IF:i32=64; 
		pub const YarnSpinnerParser_COMMAND_ELSEIF:i32=65; 
		pub const YarnSpinnerParser_COMMAND_ELSE:i32=66; 
		pub const YarnSpinnerParser_COMMAND_SET:i32=67; 
		pub const YarnSpinnerParser_COMMAND_ENDIF:i32=68; 
		pub const YarnSpinnerParser_COMMAND_CALL:i32=69; 
		pub const YarnSpinnerParser_COMMAND_DECLARE:i32=70; 
		pub const YarnSpinnerParser_COMMAND_JUMP:i32=71; 
		pub const YarnSpinnerParser_COMMAND_ENUM:i32=72; 
		pub const YarnSpinnerParser_COMMAND_CASE:i32=73; 
		pub const YarnSpinnerParser_COMMAND_ENDENUM:i32=74; 
		pub const YarnSpinnerParser_COMMAND_LOCAL:i32=75; 
		pub const YarnSpinnerParser_COMMAND_END:i32=76; 
		pub const YarnSpinnerParser_COMMAND_TEXT_NEWLINE:i32=77; 
		pub const YarnSpinnerParser_COMMAND_TEXT_END:i32=78; 
		pub const YarnSpinnerParser_COMMAND_EXPRESSION_START:i32=79; 
		pub const YarnSpinnerParser_COMMAND_TEXT:i32=80; 
		pub const YarnSpinnerParser_COMMAND_ID_NEWLINE:i32=81; 
		pub const YarnSpinnerParser_TYPE_STRING:i32=82; 
		pub const YarnSpinnerParser_TYPE_NUMBER:i32=83; 
		pub const YarnSpinnerParser_TYPE_BOOL:i32=84;
	pub const YarnSpinnerParser_EOF:i32=EOF;
	pub const RULE_dialogue:usize = 0; 
	pub const RULE_file_hashtag:usize = 1; 
	pub const RULE_node:usize = 2; 
	pub const RULE_header:usize = 3; 
	pub const RULE_body:usize = 4; 
	pub const RULE_statement:usize = 5; 
	pub const RULE_line_statement:usize = 6; 
	pub const RULE_line_formatted_text:usize = 7; 
	pub const RULE_hashtag:usize = 8; 
	pub const RULE_line_condition:usize = 9; 
	pub const RULE_expression:usize = 10; 
	pub const RULE_value:usize = 11; 
	pub const RULE_variable:usize = 12; 
	pub const RULE_function_call:usize = 13; 
	pub const RULE_if_statement:usize = 14; 
	pub const RULE_if_clause:usize = 15; 
	pub const RULE_else_if_clause:usize = 16; 
	pub const RULE_else_clause:usize = 17; 
	pub const RULE_set_statement:usize = 18; 
	pub const RULE_call_statement:usize = 19; 
	pub const RULE_command_statement:usize = 20; 
	pub const RULE_command_formatted_text:usize = 21; 
	pub const RULE_shortcut_option_statement:usize = 22; 
	pub const RULE_shortcut_option:usize = 23; 
	pub const RULE_declare_statement:usize = 24; 
	pub const RULE_jump_statement:usize = 25;
	pub const ruleNames: [&'static str; 26] =  [
		"dialogue", "file_hashtag", "node", "header", "body", "statement", "line_statement", 
		"line_formatted_text", "hashtag", "line_condition", "expression", "value", 
		"variable", "function_call", "if_statement", "if_clause", "else_if_clause", 
		"else_clause", "set_statement", "call_statement", "command_statement", 
		"command_formatted_text", "shortcut_option_statement", "shortcut_option", 
		"declare_statement", "jump_statement"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;85] = [
		None, None, None, None, None, None, None, None, Some("'---'"), None, Some("'#'"), 
		None, None, Some("'==='"), Some("'->'"), Some("'<<'"), None, None, Some("'\\'"), 
		None, None, None, None, None, None, None, None, None, Some("'true'"), 
		Some("'false'"), Some("'null'"), None, None, None, None, None, None, None, 
		None, None, None, None, Some("'+='"), Some("'-='"), Some("'*='"), Some("'%='"), 
		Some("'/='"), Some("'+'"), Some("'-'"), Some("'*'"), Some("'/'"), Some("'%'"), 
		Some("'('"), Some("')'"), Some("','"), Some("'as'"), None, None, Some("'}'"), 
		None, Some("'.'"), None, None, None, None, None, None, None, Some("'endif'"), 
		None, None, None, None, None, None, None, None, None, None, Some("'{'"), 
		None, None, Some("'string'"), Some("'number'"), Some("'bool'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;85]  = [
		None, Some("INDENT"), Some("DEDENT"), Some("BLANK_LINE_FOLLOWING_OPTION"), 
		Some("WS"), Some("COMMENT"), Some("NEWLINE"), Some("ID"), Some("BODY_START"), 
		Some("HEADER_DELIMITER"), Some("HASHTAG"), Some("REST_OF_LINE"), Some("BODY_WS"), 
		Some("BODY_END"), Some("SHORTCUT_ARROW"), Some("COMMAND_START"), Some("EXPRESSION_START"), 
		Some("ESCAPED_ANY"), Some("TEXT_ESCAPE"), Some("TEXT_COMMENT"), Some("TEXT"), 
		Some("UNESCAPABLE_CHARACTER"), Some("TEXT_COMMANDHASHTAG_WS"), Some("TEXT_COMMANDHASHTAG_COMMENT"), 
		Some("TEXT_COMMANDHASHTAG_ERROR"), Some("HASHTAG_WS"), Some("HASHTAG_TEXT"), 
		Some("EXPR_WS"), Some("KEYWORD_TRUE"), Some("KEYWORD_FALSE"), Some("KEYWORD_NULL"), 
		Some("OPERATOR_ASSIGNMENT"), Some("OPERATOR_LOGICAL_LESS_THAN_EQUALS"), 
		Some("OPERATOR_LOGICAL_GREATER_THAN_EQUALS"), Some("OPERATOR_LOGICAL_EQUALS"), 
		Some("OPERATOR_LOGICAL_LESS"), Some("OPERATOR_LOGICAL_GREATER"), Some("OPERATOR_LOGICAL_NOT_EQUALS"), 
		Some("OPERATOR_LOGICAL_AND"), Some("OPERATOR_LOGICAL_OR"), Some("OPERATOR_LOGICAL_XOR"), 
		Some("OPERATOR_LOGICAL_NOT"), Some("OPERATOR_MATHS_ADDITION_EQUALS"), 
		Some("OPERATOR_MATHS_SUBTRACTION_EQUALS"), Some("OPERATOR_MATHS_MULTIPLICATION_EQUALS"), 
		Some("OPERATOR_MATHS_MODULUS_EQUALS"), Some("OPERATOR_MATHS_DIVISION_EQUALS"), 
		Some("OPERATOR_MATHS_ADDITION"), Some("OPERATOR_MATHS_SUBTRACTION"), Some("OPERATOR_MATHS_MULTIPLICATION"), 
		Some("OPERATOR_MATHS_DIVISION"), Some("OPERATOR_MATHS_MODULUS"), Some("LPAREN"), 
		Some("RPAREN"), Some("COMMA"), Some("EXPRESSION_AS"), Some("STRING"), 
		Some("FUNC_ID"), Some("EXPRESSION_END"), Some("VAR_ID"), Some("DOT"), 
		Some("NUMBER"), Some("COMMAND_NEWLINE"), Some("COMMAND_WS"), Some("COMMAND_IF"), 
		Some("COMMAND_ELSEIF"), Some("COMMAND_ELSE"), Some("COMMAND_SET"), Some("COMMAND_ENDIF"), 
		Some("COMMAND_CALL"), Some("COMMAND_DECLARE"), Some("COMMAND_JUMP"), Some("COMMAND_ENUM"), 
		Some("COMMAND_CASE"), Some("COMMAND_ENDENUM"), Some("COMMAND_LOCAL"), 
		Some("COMMAND_END"), Some("COMMAND_TEXT_NEWLINE"), Some("COMMAND_TEXT_END"), 
		Some("COMMAND_EXPRESSION_START"), Some("COMMAND_TEXT"), Some("COMMAND_ID_NEWLINE"), 
		Some("TYPE_STRING"), Some("TYPE_NUMBER"), Some("TYPE_BOOL")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,YarnSpinnerParserExt<'input>, I, YarnSpinnerParserContextType , dyn YarnSpinnerParserListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type YarnSpinnerParserTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, YarnSpinnerParserContextType , dyn YarnSpinnerParserListener<'input> + 'a>;

/// Parser for YarnSpinnerParser grammar
pub struct YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >,
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn set_error_strategy(&mut self, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: Box<dyn ErrorStrategy<'input,BaseParserType<'input,I> > >) -> Self {
		antlr4rust::recognizer::check_version("0","5");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				YarnSpinnerParserExt{
					_pd: Default::default(),
				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for YarnSpinnerParser
pub trait YarnSpinnerParserContext<'input>:
	for<'x> Listenable<dyn YarnSpinnerParserListener<'input> + 'x > + 
	for<'x> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=YarnSpinnerParserContextType>
{}

antlr4rust::coerce_from!{ 'input : YarnSpinnerParserContext<'input> }

impl<'input, 'x, T> VisitableDyn<T> for dyn YarnSpinnerParserContext<'input> + 'input
where
    T: YarnSpinnerParserVisitor<'input> + 'x,
{
    fn accept_dyn(&self, visitor: &mut T) {
        self.accept(visitor as &mut (dyn YarnSpinnerParserVisitor<'input> + 'x))
    }
}

impl<'input> YarnSpinnerParserContext<'input> for TerminalNode<'input,YarnSpinnerParserContextType> {}
impl<'input> YarnSpinnerParserContext<'input> for ErrorNode<'input,YarnSpinnerParserContextType> {}

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn YarnSpinnerParserContext<'input> + 'input }

antlr4rust::tid! { impl<'input> TidAble<'input> for dyn YarnSpinnerParserListener<'input> + 'input }

pub struct YarnSpinnerParserContextType;
antlr4rust::tid!{YarnSpinnerParserContextType}

impl<'input> ParserNodeType<'input> for YarnSpinnerParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn YarnSpinnerParserContext<'input> + 'input;
}

impl<'input, I> Deref for YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I> DerefMut for YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct YarnSpinnerParserExt<'input>{
	_pd: PhantomData<&'input str>,
}

impl<'input> YarnSpinnerParserExt<'input>{
}
antlr4rust::tid! { YarnSpinnerParserExt<'a> }

impl<'input> TokenAware<'input> for YarnSpinnerParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for YarnSpinnerParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for YarnSpinnerParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "YarnSpinnerParser.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn YarnSpinnerParserContext<'input> + 'input)>, rule_index: i32, pred_index: i32,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					10 => YarnSpinnerParser::<'input,I>::expression_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expression_sempred(_localctx: Option<&ExpressionContext<'input>>, pred_index:i32,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 6)
				}
				1=>{
					recog.precpred(None, 5)
				}
				2=>{
					recog.precpred(None, 4)
				}
				3=>{
					recog.precpred(None, 3)
				}
				4=>{
					recog.precpred(None, 2)
				}
			_ => true
		}
	}
}
//------------------- dialogue ----------------
pub type DialogueContextAll<'input> = DialogueContext<'input>;


pub type DialogueContext<'input> = BaseParserRuleContext<'input,DialogueContextExt<'input>>;

#[derive(Clone)]
pub struct DialogueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for DialogueContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for DialogueContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_dialogue(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_dialogue(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for DialogueContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_dialogue(self);
	}
}

impl<'input> CustomRuleContext<'input> for DialogueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_dialogue }
	//fn type_rule_index() -> usize where Self: Sized { RULE_dialogue }
}
antlr4rust::tid!{DialogueContextExt<'a>}

impl<'input> DialogueContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<DialogueContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DialogueContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait DialogueContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<DialogueContextExt<'input>>{

fn node_all(&self) ->  Vec<Rc<NodeContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn node(&self, i: usize) -> Option<Rc<NodeContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn file_hashtag_all(&self) ->  Vec<Rc<File_hashtagContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn file_hashtag(&self, i: usize) -> Option<Rc<File_hashtagContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> DialogueContextAttrs<'input> for DialogueContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn dialogue(&mut self,)
	-> Result<Rc<DialogueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DialogueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_dialogue);
        let mut _localctx: Rc<DialogueContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			{
			recog.base.set_state(55);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==YarnSpinnerParser_HASHTAG {
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
				if !(_la==YarnSpinnerParser_ID) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- file_hashtag ----------------
pub type File_hashtagContextAll<'input> = File_hashtagContext<'input>;


pub type File_hashtagContext<'input> = BaseParserRuleContext<'input,File_hashtagContextExt<'input>>;

#[derive(Clone)]
pub struct File_hashtagContextExt<'input>{
	pub text: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for File_hashtagContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for File_hashtagContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_file_hashtag(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_file_hashtag(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for File_hashtagContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_file_hashtag(self);
	}
}

impl<'input> CustomRuleContext<'input> for File_hashtagContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_file_hashtag }
	//fn type_rule_index() -> usize where Self: Sized { RULE_file_hashtag }
}
antlr4rust::tid!{File_hashtagContextExt<'a>}

impl<'input> File_hashtagContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<File_hashtagContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,File_hashtagContextExt{
				text: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait File_hashtagContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<File_hashtagContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HASHTAG
/// Returns `None` if there is no child corresponding to token HASHTAG
fn HASHTAG(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_HASHTAG, 0)
}
/// Retrieves first TerminalNode corresponding to token HASHTAG_TEXT
/// Returns `None` if there is no child corresponding to token HASHTAG_TEXT
fn HASHTAG_TEXT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_HASHTAG_TEXT, 0)
}

}

impl<'input> File_hashtagContextAttrs<'input> for File_hashtagContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn file_hashtag(&mut self,)
	-> Result<Rc<File_hashtagContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = File_hashtagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_file_hashtag);
        let mut _localctx: Rc<File_hashtagContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(63);
			recog.base.match_token(YarnSpinnerParser_HASHTAG,&mut recog.err_handler)?;

			recog.base.set_state(64);
			let tmp = recog.base.match_token(YarnSpinnerParser_HASHTAG_TEXT,&mut recog.err_handler)?;
			 cast_mut::<_,File_hashtagContext >(&mut _localctx).text = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- node ----------------
pub type NodeContextAll<'input> = NodeContext<'input>;


pub type NodeContext<'input> = BaseParserRuleContext<'input,NodeContextExt<'input>>;

#[derive(Clone)]
pub struct NodeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for NodeContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for NodeContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_node(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_node(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for NodeContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		YarnSpinnerParserVisitor::visit_node(visitor, self);
	}
}

impl<'input> CustomRuleContext<'input> for NodeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_node }
	//fn type_rule_index() -> usize where Self: Sized { RULE_node }
}
antlr4rust::tid!{NodeContextExt<'a>}

impl<'input> NodeContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<NodeContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,NodeContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait NodeContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<NodeContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token BODY_START
/// Returns `None` if there is no child corresponding to token BODY_START
fn BODY_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_BODY_START, 0)
}
fn body(&self) -> Option<Rc<BodyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token BODY_END
/// Returns `None` if there is no child corresponding to token BODY_END
fn BODY_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_BODY_END, 0)
}
fn header_all(&self) ->  Vec<Rc<HeaderContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn header(&self, i: usize) -> Option<Rc<HeaderContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> NodeContextAttrs<'input> for NodeContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn node(&mut self,)
	-> Result<Rc<NodeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = NodeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_node);
        let mut _localctx: Rc<NodeContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
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
				if !(_la==YarnSpinnerParser_ID) {break}
			}
			recog.base.set_state(71);
			recog.base.match_token(YarnSpinnerParser_BODY_START,&mut recog.err_handler)?;

			/*InvokeRule body*/
			recog.base.set_state(72);
			recog.body()?;

			recog.base.set_state(73);
			recog.base.match_token(YarnSpinnerParser_BODY_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- header ----------------
pub type HeaderContextAll<'input> = HeaderContext<'input>;


pub type HeaderContext<'input> = BaseParserRuleContext<'input,HeaderContextExt<'input>>;

#[derive(Clone)]
pub struct HeaderContextExt<'input>{
	pub header_key: Option<TokenType<'input>>,
	pub header_value: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for HeaderContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for HeaderContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_header(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_header(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for HeaderContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_header(self);
	}
}

impl<'input> CustomRuleContext<'input> for HeaderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_header }
	//fn type_rule_index() -> usize where Self: Sized { RULE_header }
}
antlr4rust::tid!{HeaderContextExt<'a>}

impl<'input> HeaderContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HeaderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HeaderContextExt{
				header_key: None, header_value: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait HeaderContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<HeaderContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HEADER_DELIMITER
/// Returns `None` if there is no child corresponding to token HEADER_DELIMITER
fn HEADER_DELIMITER(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_HEADER_DELIMITER, 0)
}
/// Retrieves first TerminalNode corresponding to token ID
/// Returns `None` if there is no child corresponding to token ID
fn ID(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_ID, 0)
}
/// Retrieves first TerminalNode corresponding to token REST_OF_LINE
/// Returns `None` if there is no child corresponding to token REST_OF_LINE
fn REST_OF_LINE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_REST_OF_LINE, 0)
}

}

impl<'input> HeaderContextAttrs<'input> for HeaderContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn header(&mut self,)
	-> Result<Rc<HeaderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HeaderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_header);
        let mut _localctx: Rc<HeaderContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(75);
			let tmp = recog.base.match_token(YarnSpinnerParser_ID,&mut recog.err_handler)?;
			 cast_mut::<_,HeaderContext >(&mut _localctx).header_key = Some(tmp.clone());
			  

			recog.base.set_state(76);
			recog.base.match_token(YarnSpinnerParser_HEADER_DELIMITER,&mut recog.err_handler)?;

			recog.base.set_state(78);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==YarnSpinnerParser_REST_OF_LINE {
				{
				recog.base.set_state(77);
				let tmp = recog.base.match_token(YarnSpinnerParser_REST_OF_LINE,&mut recog.err_handler)?;
				 cast_mut::<_,HeaderContext >(&mut _localctx).header_value = Some(tmp.clone());
				  

				}
			}

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- body ----------------
pub type BodyContextAll<'input> = BodyContext<'input>;


pub type BodyContext<'input> = BaseParserRuleContext<'input,BodyContextExt<'input>>;

#[derive(Clone)]
pub struct BodyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for BodyContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for BodyContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_body(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_body(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for BodyContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_body(self);
	}
}

impl<'input> CustomRuleContext<'input> for BodyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_body }
	//fn type_rule_index() -> usize where Self: Sized { RULE_body }
}
antlr4rust::tid!{BodyContextExt<'a>}

impl<'input> BodyContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<BodyContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BodyContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait BodyContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<BodyContextExt<'input>>{

fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> BodyContextAttrs<'input> for BodyContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn body(&mut self,)
	-> Result<Rc<BodyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BodyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_body);
        let mut _localctx: Rc<BodyContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(83);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & 1163266) != 0) {
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
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- statement ----------------
pub type StatementContextAll<'input> = StatementContext<'input>;


pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

#[derive(Clone)]
pub struct StatementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for StatementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for StatementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_statement }
}
antlr4rust::tid!{StatementContextExt<'a>}

impl<'input> StatementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<StatementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StatementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait StatementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<StatementContextExt<'input>>{

fn line_statement(&self) -> Option<Rc<Line_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn if_statement(&self) -> Option<Rc<If_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn set_statement(&self) -> Option<Rc<Set_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn shortcut_option_statement(&self) -> Option<Rc<Shortcut_option_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn call_statement(&self) -> Option<Rc<Call_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn command_statement(&self) -> Option<Rc<Command_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn declare_statement(&self) -> Option<Rc<Declare_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn jump_statement(&self) -> Option<Rc<Jump_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INDENT
/// Returns `None` if there is no child corresponding to token INDENT
fn INDENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_INDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEDENT
/// Returns `None` if there is no child corresponding to token DEDENT
fn DEDENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_DEDENT, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn statement(&mut self,)
	-> Result<Rc<StatementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StatementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_statement);
        let mut _localctx: Rc<StatementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(102);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(6,&mut recog.base)? {
				1 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
					recog.base.enter_outer_alt(None, 1)?;
					{
					/*InvokeRule line_statement*/
					recog.base.set_state(86);
					recog.line_statement()?;

					}
				}
			,
				2 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 2)?;
					recog.base.enter_outer_alt(None, 2)?;
					{
					/*InvokeRule if_statement*/
					recog.base.set_state(87);
					recog.if_statement()?;

					}
				}
			,
				3 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 3)?;
					recog.base.enter_outer_alt(None, 3)?;
					{
					/*InvokeRule set_statement*/
					recog.base.set_state(88);
					recog.set_statement()?;

					}
				}
			,
				4 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 4)?;
					recog.base.enter_outer_alt(None, 4)?;
					{
					/*InvokeRule shortcut_option_statement*/
					recog.base.set_state(89);
					recog.shortcut_option_statement()?;

					}
				}
			,
				5 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 5)?;
					recog.base.enter_outer_alt(None, 5)?;
					{
					/*InvokeRule call_statement*/
					recog.base.set_state(90);
					recog.call_statement()?;

					}
				}
			,
				6 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 6)?;
					recog.base.enter_outer_alt(None, 6)?;
					{
					/*InvokeRule command_statement*/
					recog.base.set_state(91);
					recog.command_statement()?;

					}
				}
			,
				7 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 7)?;
					recog.base.enter_outer_alt(None, 7)?;
					{
					/*InvokeRule declare_statement*/
					recog.base.set_state(92);
					recog.declare_statement()?;

					}
				}
			,
				8 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 8)?;
					recog.base.enter_outer_alt(None, 8)?;
					{
					/*InvokeRule jump_statement*/
					recog.base.set_state(93);
					recog.jump_statement()?;

					}
				}
			,
				9 =>{
					//recog.base.enter_outer_alt(_localctx.clone(), 9)?;
					recog.base.enter_outer_alt(None, 9)?;
					{
					recog.base.set_state(94);
					recog.base.match_token(YarnSpinnerParser_INDENT,&mut recog.err_handler)?;

					recog.base.set_state(98);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 1163266) != 0) {
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
					recog.base.match_token(YarnSpinnerParser_DEDENT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- line_statement ----------------
pub type Line_statementContextAll<'input> = Line_statementContext<'input>;


pub type Line_statementContext<'input> = BaseParserRuleContext<'input,Line_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Line_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Line_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Line_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_line_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_line_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Line_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_line_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Line_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_line_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_line_statement }
}
antlr4rust::tid!{Line_statementContextExt<'a>}

impl<'input> Line_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Line_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Line_statementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Line_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Line_statementContextExt<'input>>{

fn line_formatted_text(&self) -> Option<Rc<Line_formatted_textContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token NEWLINE
/// Returns `None` if there is no child corresponding to token NEWLINE
fn NEWLINE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_NEWLINE, 0)
}
fn line_condition(&self) -> Option<Rc<Line_conditionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn hashtag_all(&self) ->  Vec<Rc<HashtagContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hashtag(&self, i: usize) -> Option<Rc<HashtagContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Line_statementContextAttrs<'input> for Line_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn line_statement(&mut self,)
	-> Result<Rc<Line_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Line_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_line_statement);
        let mut _localctx: Rc<Line_statementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule line_formatted_text*/
			recog.base.set_state(104);
			recog.line_formatted_text()?;

			recog.base.set_state(106);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==YarnSpinnerParser_COMMAND_START {
				{
				/*InvokeRule line_condition*/
				recog.base.set_state(105);
				recog.line_condition()?;

				}
			}

			recog.base.set_state(111);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==YarnSpinnerParser_HASHTAG {
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
			recog.base.match_token(YarnSpinnerParser_NEWLINE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- line_formatted_text ----------------
pub type Line_formatted_textContextAll<'input> = Line_formatted_textContext<'input>;


pub type Line_formatted_textContext<'input> = BaseParserRuleContext<'input,Line_formatted_textContextExt<'input>>;

#[derive(Clone)]
pub struct Line_formatted_textContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Line_formatted_textContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Line_formatted_textContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_line_formatted_text(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_line_formatted_text(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Line_formatted_textContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_line_formatted_text(self);
	}
}

impl<'input> CustomRuleContext<'input> for Line_formatted_textContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_line_formatted_text }
	//fn type_rule_index() -> usize where Self: Sized { RULE_line_formatted_text }
}
antlr4rust::tid!{Line_formatted_textContextExt<'a>}

impl<'input> Line_formatted_textContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Line_formatted_textContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Line_formatted_textContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Line_formatted_textContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Line_formatted_textContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_START in current rule
fn EXPRESSION_START_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_START, starting from 0.
/// Returns `None` if number of children corresponding to token EXPRESSION_START is less or equal than `i`.
fn EXPRESSION_START(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_EXPRESSION_START, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_END in current rule
fn EXPRESSION_END_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_END, starting from 0.
/// Returns `None` if number of children corresponding to token EXPRESSION_END is less or equal than `i`.
fn EXPRESSION_END(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_EXPRESSION_END, i)
}
/// Retrieves all `TerminalNode`s corresponding to token TEXT in current rule
fn TEXT_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token TEXT, starting from 0.
/// Returns `None` if number of children corresponding to token TEXT is less or equal than `i`.
fn TEXT(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_TEXT, i)
}

}

impl<'input> Line_formatted_textContextAttrs<'input> for Line_formatted_textContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn line_formatted_text(&mut self,)
	-> Result<Rc<Line_formatted_textContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Line_formatted_textContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_line_formatted_text);
        let mut _localctx: Rc<Line_formatted_textContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(125); 
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			loop {
				{
				recog.base.set_state(125);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				YarnSpinnerParser_TEXT 
					=> {
						{
						recog.base.set_state(117); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = 1;
						loop {
							match _alt {
							    x if x == 1=>
								{
								{
								recog.base.set_state(116);
								recog.base.match_token(YarnSpinnerParser_TEXT,&mut recog.err_handler)?;

								}
								}

							_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
							}
							recog.base.set_state(119); 
							recog.err_handler.sync(&mut recog.base)?;
							_alt = recog.interpreter.adaptive_predict(9,&mut recog.base)?;
							if _alt==2 || _alt==INVALID_ALT { break }
						}
						}
					}

				YarnSpinnerParser_EXPRESSION_START 
					=> {
						{
						recog.base.set_state(121);
						recog.base.match_token(YarnSpinnerParser_EXPRESSION_START,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(122);
						recog.expression_rec(0)?;

						recog.base.set_state(123);
						recog.base.match_token(YarnSpinnerParser_EXPRESSION_END,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
				}
				}
				recog.base.set_state(127); 
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				if !(_la==YarnSpinnerParser_EXPRESSION_START || _la==YarnSpinnerParser_TEXT) {break}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- hashtag ----------------
pub type HashtagContextAll<'input> = HashtagContext<'input>;


pub type HashtagContext<'input> = BaseParserRuleContext<'input,HashtagContextExt<'input>>;

#[derive(Clone)]
pub struct HashtagContextExt<'input>{
	pub text: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

// Manually injected after generation.
impl<'input> HashtagContextExt<'input> {
	pub fn new_with_text(
		parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input>>,
		invoking_state: i32,
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
}

impl<'input> YarnSpinnerParserContext<'input> for HashtagContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for HashtagContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_hashtag(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_hashtag(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for HashtagContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_hashtag(self);
	}
}

impl<'input> CustomRuleContext<'input> for HashtagContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_hashtag }
	//fn type_rule_index() -> usize where Self: Sized { RULE_hashtag }
}
antlr4rust::tid!{HashtagContextExt<'a>}

impl<'input> HashtagContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<HashtagContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,HashtagContextExt{
				text: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait HashtagContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<HashtagContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token HASHTAG
/// Returns `None` if there is no child corresponding to token HASHTAG
fn HASHTAG(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_HASHTAG, 0)
}
/// Retrieves first TerminalNode corresponding to token HASHTAG_TEXT
/// Returns `None` if there is no child corresponding to token HASHTAG_TEXT
fn HASHTAG_TEXT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_HASHTAG_TEXT, 0)
}

}

impl<'input> HashtagContextAttrs<'input> for HashtagContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn hashtag(&mut self,)
	-> Result<Rc<HashtagContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = HashtagContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_hashtag);
        let mut _localctx: Rc<HashtagContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(129);
			recog.base.match_token(YarnSpinnerParser_HASHTAG,&mut recog.err_handler)?;

			recog.base.set_state(130);
			let tmp = recog.base.match_token(YarnSpinnerParser_HASHTAG_TEXT,&mut recog.err_handler)?;
			 cast_mut::<_,HashtagContext >(&mut _localctx).text = Some(tmp.clone());
			  

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- line_condition ----------------
pub type Line_conditionContextAll<'input> = Line_conditionContext<'input>;


pub type Line_conditionContext<'input> = BaseParserRuleContext<'input,Line_conditionContextExt<'input>>;

#[derive(Clone)]
pub struct Line_conditionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Line_conditionContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Line_conditionContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_line_condition(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_line_condition(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Line_conditionContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_line_condition(self);
	}
}

impl<'input> CustomRuleContext<'input> for Line_conditionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_line_condition }
	//fn type_rule_index() -> usize where Self: Sized { RULE_line_condition }
}
antlr4rust::tid!{Line_conditionContextExt<'a>}

impl<'input> Line_conditionContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Line_conditionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Line_conditionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Line_conditionContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Line_conditionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_IF
/// Returns `None` if there is no child corresponding to token COMMAND_IF
fn COMMAND_IF(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_IF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}

}

impl<'input> Line_conditionContextAttrs<'input> for Line_conditionContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn line_condition(&mut self,)
	-> Result<Rc<Line_conditionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Line_conditionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_line_condition);
        let mut _localctx: Rc<Line_conditionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(132);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(133);
			recog.base.match_token(YarnSpinnerParser_COMMAND_IF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(134);
			recog.expression_rec(0)?;

			recog.base.set_state(135);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- expression ----------------
#[derive(Debug)]
pub enum ExpressionContextAll<'input>{
	ExpParensContext(ExpParensContext<'input>),
	ExpMultDivModContext(ExpMultDivModContext<'input>),
	ExpComparisonContext(ExpComparisonContext<'input>),
	ExpNegativeContext(ExpNegativeContext<'input>),
	ExpAndOrXorContext(ExpAndOrXorContext<'input>),
	ExpAddSubContext(ExpAddSubContext<'input>),
	ExpNotContext(ExpNotContext<'input>),
	ExpValueContext(ExpValueContext<'input>),
	ExpEqualityContext(ExpEqualityContext<'input>),
Error(ExpressionContext<'input>)
}
antlr4rust::tid!{ExpressionContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ExpressionContextAll<'input>{}

impl<'input> YarnSpinnerParserContext<'input> for ExpressionContextAll<'input>{}

impl<'input> Deref for ExpressionContextAll<'input>{
	type Target = dyn ExpressionContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExpressionContextAll::*;
		match self{
			ExpParensContext(inner) => inner,
			ExpMultDivModContext(inner) => inner,
			ExpComparisonContext(inner) => inner,
			ExpNegativeContext(inner) => inner,
			ExpAndOrXorContext(inner) => inner,
			ExpAddSubContext(inner) => inner,
			ExpNotContext(inner) => inner,
			ExpValueContext(inner) => inner,
			ExpEqualityContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpressionContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpressionContextAll<'input>{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ExpressionContext<'input> = BaseParserRuleContext<'input,ExpressionContextExt<'input>>;

#[derive(Clone)]
pub struct ExpressionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for ExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpressionContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}
antlr4rust::tid!{ExpressionContextExt<'a>}

impl<'input> ExpressionContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ExpressionContextAll<'input>> {
		Rc::new(
		ExpressionContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExpressionContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExpressionContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<ExpressionContextExt<'input>>{


}

impl<'input> ExpressionContextAttrs<'input> for ExpressionContext<'input>{}

pub type ExpParensContext<'input> = BaseParserRuleContext<'input,ExpParensContextExt<'input>>;

pub trait ExpParensContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAREN
	/// Returns `None` if there is no child corresponding to token LPAREN
	fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_LPAREN, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAREN
	/// Returns `None` if there is no child corresponding to token RPAREN
	fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_RPAREN, 0)
	}
}

impl<'input> ExpParensContextAttrs<'input> for ExpParensContext<'input>{}

pub struct ExpParensContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpParensContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpParensContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpParensContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expParens(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expParens(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpParensContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expParens(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpParensContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpParensContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpParensContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpParensContext<'input> {}

impl<'input> ExpParensContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpParensContext(
				BaseParserRuleContext::copy_from(ctx,ExpParensContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpMultDivModContext<'input> = BaseParserRuleContext<'input,ExpMultDivModContextExt<'input>>;

pub trait ExpMultDivModContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MULTIPLICATION
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MULTIPLICATION
	fn OPERATOR_MATHS_MULTIPLICATION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_MULTIPLICATION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_DIVISION
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_DIVISION
	fn OPERATOR_MATHS_DIVISION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_DIVISION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MODULUS
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MODULUS
	fn OPERATOR_MATHS_MODULUS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_MODULUS, 0)
	}
}

impl<'input> ExpMultDivModContextAttrs<'input> for ExpMultDivModContext<'input>{}

pub struct ExpMultDivModContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpMultDivModContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpMultDivModContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpMultDivModContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expMultDivMod(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expMultDivMod(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpMultDivModContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expMultDivMod(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpMultDivModContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpMultDivModContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpMultDivModContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpMultDivModContext<'input> {}

impl<'input> ExpMultDivModContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpMultDivModContext(
				BaseParserRuleContext::copy_from(ctx,ExpMultDivModContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpComparisonContext<'input> = BaseParserRuleContext<'input,ExpComparisonContextExt<'input>>;

pub trait ExpComparisonContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_LESS_THAN_EQUALS
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_LESS_THAN_EQUALS
	fn OPERATOR_LOGICAL_LESS_THAN_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_LESS_THAN_EQUALS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_GREATER_THAN_EQUALS
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_GREATER_THAN_EQUALS
	fn OPERATOR_LOGICAL_GREATER_THAN_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_GREATER_THAN_EQUALS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_LESS
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_LESS
	fn OPERATOR_LOGICAL_LESS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_LESS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_GREATER
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_GREATER
	fn OPERATOR_LOGICAL_GREATER(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_GREATER, 0)
	}
}

impl<'input> ExpComparisonContextAttrs<'input> for ExpComparisonContext<'input>{}

pub struct ExpComparisonContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpComparisonContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpComparisonContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpComparisonContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expComparison(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expComparison(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpComparisonContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expComparison(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpComparisonContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpComparisonContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpComparisonContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpComparisonContext<'input> {}

impl<'input> ExpComparisonContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpComparisonContext(
				BaseParserRuleContext::copy_from(ctx,ExpComparisonContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpNegativeContext<'input> = BaseParserRuleContext<'input,ExpNegativeContextExt<'input>>;

pub trait ExpNegativeContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION
	fn OPERATOR_MATHS_SUBTRACTION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION, 0)
	}
}

impl<'input> ExpNegativeContextAttrs<'input> for ExpNegativeContext<'input>{}

pub struct ExpNegativeContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpNegativeContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpNegativeContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpNegativeContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expNegative(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expNegative(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpNegativeContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expNegative(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpNegativeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpNegativeContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpNegativeContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpNegativeContext<'input> {}

impl<'input> ExpNegativeContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpNegativeContext(
				BaseParserRuleContext::copy_from(ctx,ExpNegativeContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpAndOrXorContext<'input> = BaseParserRuleContext<'input,ExpAndOrXorContextExt<'input>>;

pub trait ExpAndOrXorContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_AND
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_AND
	fn OPERATOR_LOGICAL_AND(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_AND, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_OR
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_OR
	fn OPERATOR_LOGICAL_OR(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_OR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_XOR
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_XOR
	fn OPERATOR_LOGICAL_XOR(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_XOR, 0)
	}
}

impl<'input> ExpAndOrXorContextAttrs<'input> for ExpAndOrXorContext<'input>{}

pub struct ExpAndOrXorContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpAndOrXorContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpAndOrXorContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpAndOrXorContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expAndOrXor(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expAndOrXor(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpAndOrXorContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expAndOrXor(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpAndOrXorContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpAndOrXorContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpAndOrXorContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpAndOrXorContext<'input> {}

impl<'input> ExpAndOrXorContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpAndOrXorContext(
				BaseParserRuleContext::copy_from(ctx,ExpAndOrXorContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpAddSubContext<'input> = BaseParserRuleContext<'input,ExpAddSubContextExt<'input>>;

pub trait ExpAddSubContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_ADDITION
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_ADDITION
	fn OPERATOR_MATHS_ADDITION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_ADDITION, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION
	/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION
	fn OPERATOR_MATHS_SUBTRACTION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION, 0)
	}
}

impl<'input> ExpAddSubContextAttrs<'input> for ExpAddSubContext<'input>{}

pub struct ExpAddSubContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpAddSubContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpAddSubContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpAddSubContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expAddSub(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expAddSub(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpAddSubContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expAddSub(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpAddSubContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpAddSubContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpAddSubContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpAddSubContext<'input> {}

impl<'input> ExpAddSubContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpAddSubContext(
				BaseParserRuleContext::copy_from(ctx,ExpAddSubContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpNotContext<'input> = BaseParserRuleContext<'input,ExpNotContextExt<'input>>;

pub trait ExpNotContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_NOT
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_NOT
	fn OPERATOR_LOGICAL_NOT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_NOT, 0)
	}
}

impl<'input> ExpNotContextAttrs<'input> for ExpNotContext<'input>{}

pub struct ExpNotContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpNotContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpNotContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpNotContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expNot(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expNot(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpNotContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expNot(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpNotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpNotContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpNotContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpNotContext<'input> {}

impl<'input> ExpNotContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpNotContext(
				BaseParserRuleContext::copy_from(ctx,ExpNotContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpValueContext<'input> = BaseParserRuleContext<'input,ExpValueContextExt<'input>>;

pub trait ExpValueContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ExpValueContextAttrs<'input> for ExpValueContext<'input>{}

pub struct ExpValueContextExt<'input>{
	base:ExpressionContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpValueContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpValueContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpValueContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expValue(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expValue(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpValueContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expValue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpValueContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpValueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpValueContext<'input> {}

impl<'input> ExpValueContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpValueContext(
				BaseParserRuleContext::copy_from(ctx,ExpValueContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExpEqualityContext<'input> = BaseParserRuleContext<'input,ExpEqualityContextExt<'input>>;

pub trait ExpEqualityContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_EQUALS
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_EQUALS
	fn OPERATOR_LOGICAL_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_EQUALS, 0)
	}
	/// Retrieves first TerminalNode corresponding to token OPERATOR_LOGICAL_NOT_EQUALS
	/// Returns `None` if there is no child corresponding to token OPERATOR_LOGICAL_NOT_EQUALS
	fn OPERATOR_LOGICAL_NOT_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_OPERATOR_LOGICAL_NOT_EQUALS, 0)
	}
}

impl<'input> ExpEqualityContextAttrs<'input> for ExpEqualityContext<'input>{}

pub struct ExpEqualityContextExt<'input>{
	base:ExpressionContextExt<'input>,
	pub op: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ExpEqualityContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ExpEqualityContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ExpEqualityContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_expEquality(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_expEquality(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ExpEqualityContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_expEquality(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExpEqualityContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expression }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expression }
}

impl<'input> Borrow<ExpressionContextExt<'input>> for ExpEqualityContext<'input>{
	fn borrow(&self) -> &ExpressionContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExpressionContextExt<'input>> for ExpEqualityContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExpressionContextExt<'input> { &mut self.base }
}

impl<'input> ExpressionContextAttrs<'input> for ExpEqualityContext<'input> {}

impl<'input> ExpEqualityContextExt<'input>{
	fn new(ctx: &dyn ExpressionContextAttrs<'input>) -> Rc<ExpressionContextAll<'input>>  {
		Rc::new(
			ExpressionContextAll::ExpEqualityContext(
				BaseParserRuleContext::copy_from(ctx,ExpEqualityContextExt{
					op:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn  expression(&mut self,)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		self.expression_rec(0)
	}

	fn expression_rec(&mut self, _p: i32)
	-> Result<Rc<ExpressionContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExpressionContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 20, RULE_expression, _p);
	    let mut _localctx: Rc<ExpressionContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 20;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(147);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			YarnSpinnerParser_LPAREN 
				=> {
					{
					let mut tmp = ExpParensContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();

					recog.base.set_state(138);
					recog.base.match_token(YarnSpinnerParser_LPAREN,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(139);
					recog.expression_rec(0)?;

					recog.base.set_state(140);
					recog.base.match_token(YarnSpinnerParser_RPAREN,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION 
				=> {
					{
					let mut tmp = ExpNegativeContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(142);
					let tmp = recog.base.match_token(YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION,&mut recog.err_handler)?;
					if let ExpressionContextAll::ExpNegativeContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					/*InvokeRule expression*/
					recog.base.set_state(143);
					recog.expression_rec(8)?;

					}
				}

			YarnSpinnerParser_OPERATOR_LOGICAL_NOT 
				=> {
					{
					let mut tmp = ExpNotContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(144);
					let tmp = recog.base.match_token(YarnSpinnerParser_OPERATOR_LOGICAL_NOT,&mut recog.err_handler)?;
					if let ExpressionContextAll::ExpNotContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
					ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					/*InvokeRule expression*/
					recog.base.set_state(145);
					recog.expression_rec(7)?;

					}
				}

			YarnSpinnerParser_KEYWORD_TRUE |YarnSpinnerParser_KEYWORD_FALSE |YarnSpinnerParser_KEYWORD_NULL |
			YarnSpinnerParser_STRING |YarnSpinnerParser_FUNC_ID |YarnSpinnerParser_VAR_ID |
			YarnSpinnerParser_NUMBER 
				=> {
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

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(166);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event()?;
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(164);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpMultDivModContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression)?;
							_localctx = tmp;
							recog.base.set_state(149);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(150);
							if let ExpressionContextAll::ExpMultDivModContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 49)) & !0x3f) == 0 && ((1usize << (_la - 49)) & 7) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::ExpMultDivModContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(151);
							recog.expression_rec(7)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpAddSubContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression)?;
							_localctx = tmp;
							recog.base.set_state(152);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(153);
							if let ExpressionContextAll::ExpAddSubContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==YarnSpinnerParser_OPERATOR_MATHS_ADDITION || _la==YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::ExpAddSubContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(154);
							recog.expression_rec(6)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpComparisonContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression)?;
							_localctx = tmp;
							recog.base.set_state(155);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(156);
							if let ExpressionContextAll::ExpComparisonContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 32)) & !0x3f) == 0 && ((1usize << (_la - 32)) & 27) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::ExpComparisonContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(157);
							recog.expression_rec(5)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpEqualityContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression)?;
							_localctx = tmp;
							recog.base.set_state(158);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(159);
							if let ExpressionContextAll::ExpEqualityContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(_la==YarnSpinnerParser_OPERATOR_LOGICAL_EQUALS || _la==YarnSpinnerParser_OPERATOR_LOGICAL_NOT_EQUALS) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::ExpEqualityContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
								recog.err_handler.report_match(&mut recog.base);
								recog.base.consume(&mut recog.err_handler);
							}
							/*InvokeRule expression*/
							recog.base.set_state(160);
							recog.expression_rec(4)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ExpAndOrXorContextExt::new(&**ExpressionContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expression)?;
							_localctx = tmp;
							recog.base.set_state(161);
							if !({let _localctx = Some(_localctx.clone());
							recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(162);
							if let ExpressionContextAll::ExpAndOrXorContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
							ctx.op = recog.base.input.lt(1).cloned(); } else {unreachable!("cant cast");} 
							_la = recog.base.input.la(1);
							if { !(((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & 7) != 0)) } {
								let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
								if let ExpressionContextAll::ExpAndOrXorContext(ctx) = cast_mut::<_,ExpressionContextAll >(&mut _localctx){
								ctx.op = Some(tmp.clone()); } else {unreachable!("cant cast");}  

							}
							else {
								if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
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
				_alt = recog.interpreter.adaptive_predict(14,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx)?;

		Ok(_localctx)
	}
}
//------------------- value ----------------
#[derive(Debug)]
pub enum ValueContextAll<'input>{
	ValueNullContext(ValueNullContext<'input>),
	ValueNumberContext(ValueNumberContext<'input>),
	ValueTrueContext(ValueTrueContext<'input>),
	ValueFalseContext(ValueFalseContext<'input>),
	ValueFuncContext(ValueFuncContext<'input>),
	ValueVarContext(ValueVarContext<'input>),
	ValueStringContext(ValueStringContext<'input>),
Error(ValueContext<'input>)
}
antlr4rust::tid!{ValueContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for ValueContextAll<'input>{}

impl<'input> YarnSpinnerParserContext<'input> for ValueContextAll<'input>{}

impl<'input> Deref for ValueContextAll<'input>{
	type Target = dyn ValueContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ValueContextAll::*;
		match self{
			ValueNullContext(inner) => inner,
			ValueNumberContext(inner) => inner,
			ValueTrueContext(inner) => inner,
			ValueFalseContext(inner) => inner,
			ValueFuncContext(inner) => inner,
			ValueVarContext(inner) => inner,
			ValueStringContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueContextAll<'input>{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type ValueContext<'input> = BaseParserRuleContext<'input,ValueContextExt<'input>>;

#[derive(Clone)]
pub struct ValueContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for ValueContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueContext<'input>{
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ValueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}
antlr4rust::tid!{ValueContextExt<'a>}

impl<'input> ValueContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<ValueContextAll<'input>> {
		Rc::new(
		ValueContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ValueContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ValueContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<ValueContextExt<'input>>{


}

impl<'input> ValueContextAttrs<'input> for ValueContext<'input>{}

pub type ValueNullContext<'input> = BaseParserRuleContext<'input,ValueNullContextExt<'input>>;

pub trait ValueNullContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_NULL
	/// Returns `None` if there is no child corresponding to token KEYWORD_NULL
	fn KEYWORD_NULL(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_KEYWORD_NULL, 0)
	}
}

impl<'input> ValueNullContextAttrs<'input> for ValueNullContext<'input>{}

pub struct ValueNullContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueNullContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueNullContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueNullContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueNull(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueNull(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueNullContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueNull(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueNullContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueNullContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueNullContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueNullContext<'input> {}

impl<'input> ValueNullContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueNullContext(
				BaseParserRuleContext::copy_from(ctx,ValueNullContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueNumberContext<'input> = BaseParserRuleContext<'input,ValueNumberContextExt<'input>>;

pub trait ValueNumberContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NUMBER
	/// Returns `None` if there is no child corresponding to token NUMBER
	fn NUMBER(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_NUMBER, 0)
	}
}

impl<'input> ValueNumberContextAttrs<'input> for ValueNumberContext<'input>{}

pub struct ValueNumberContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueNumberContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueNumberContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueNumberContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueNumber(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueNumber(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueNumberContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueNumber(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueNumberContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueNumberContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueNumberContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueNumberContext<'input> {}

impl<'input> ValueNumberContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueNumberContext(
				BaseParserRuleContext::copy_from(ctx,ValueNumberContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueTrueContext<'input> = BaseParserRuleContext<'input,ValueTrueContextExt<'input>>;

pub trait ValueTrueContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_TRUE
	/// Returns `None` if there is no child corresponding to token KEYWORD_TRUE
	fn KEYWORD_TRUE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_KEYWORD_TRUE, 0)
	}
}

impl<'input> ValueTrueContextAttrs<'input> for ValueTrueContext<'input>{}

pub struct ValueTrueContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueTrueContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueTrueContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueTrueContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueTrue(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueTrue(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueTrueContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueTrue(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueTrueContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueTrueContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueTrueContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueTrueContext<'input> {}

impl<'input> ValueTrueContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueTrueContext(
				BaseParserRuleContext::copy_from(ctx,ValueTrueContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueFalseContext<'input> = BaseParserRuleContext<'input,ValueFalseContextExt<'input>>;

pub trait ValueFalseContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token KEYWORD_FALSE
	/// Returns `None` if there is no child corresponding to token KEYWORD_FALSE
	fn KEYWORD_FALSE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_KEYWORD_FALSE, 0)
	}
}

impl<'input> ValueFalseContextAttrs<'input> for ValueFalseContext<'input>{}

pub struct ValueFalseContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueFalseContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueFalseContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueFalseContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueFalse(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueFalse(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueFalseContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueFalse(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueFalseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueFalseContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueFalseContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueFalseContext<'input> {}

impl<'input> ValueFalseContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueFalseContext(
				BaseParserRuleContext::copy_from(ctx,ValueFalseContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueFuncContext<'input> = BaseParserRuleContext<'input,ValueFuncContextExt<'input>>;

pub trait ValueFuncContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ValueFuncContextAttrs<'input> for ValueFuncContext<'input>{}

pub struct ValueFuncContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueFuncContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueFuncContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueFuncContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueFunc(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueFunc(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueFuncContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueFunc(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueFuncContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueFuncContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueFuncContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueFuncContext<'input> {}

impl<'input> ValueFuncContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueFuncContext(
				BaseParserRuleContext::copy_from(ctx,ValueFuncContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueVarContext<'input> = BaseParserRuleContext<'input,ValueVarContextExt<'input>>;

pub trait ValueVarContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ValueVarContextAttrs<'input> for ValueVarContext<'input>{}

pub struct ValueVarContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueVarContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueVarContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueVarContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueVar(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueVar(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueVarContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueVarContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueVarContext<'input> {}

impl<'input> ValueVarContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueVarContext(
				BaseParserRuleContext::copy_from(ctx,ValueVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ValueStringContext<'input> = BaseParserRuleContext<'input,ValueStringContextExt<'input>>;

pub trait ValueStringContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRING
	/// Returns `None` if there is no child corresponding to token STRING
	fn STRING(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_STRING, 0)
	}
}

impl<'input> ValueStringContextAttrs<'input> for ValueStringContext<'input>{}

pub struct ValueStringContextExt<'input>{
	base:ValueContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{ValueStringContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for ValueStringContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for ValueStringContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_valueString(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_valueString(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for ValueStringContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_valueString(self);
	}
}

impl<'input> CustomRuleContext<'input> for ValueStringContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_value }
	//fn type_rule_index() -> usize where Self: Sized { RULE_value }
}

impl<'input> Borrow<ValueContextExt<'input>> for ValueStringContext<'input>{
	fn borrow(&self) -> &ValueContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ValueContextExt<'input>> for ValueStringContext<'input>{
	fn borrow_mut(&mut self) -> &mut ValueContextExt<'input> { &mut self.base }
}

impl<'input> ValueContextAttrs<'input> for ValueStringContext<'input> {}

impl<'input> ValueStringContextExt<'input>{
	fn new(ctx: &dyn ValueContextAttrs<'input>) -> Rc<ValueContextAll<'input>>  {
		Rc::new(
			ValueContextAll::ValueStringContext(
				BaseParserRuleContext::copy_from(ctx,ValueStringContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn value(&mut self,)
	-> Result<Rc<ValueContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ValueContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_value);
        let mut _localctx: Rc<ValueContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(176);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			YarnSpinnerParser_NUMBER 
				=> {
					let tmp = ValueNumberContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(169);
					recog.base.match_token(YarnSpinnerParser_NUMBER,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_KEYWORD_TRUE 
				=> {
					let tmp = ValueTrueContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(170);
					recog.base.match_token(YarnSpinnerParser_KEYWORD_TRUE,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_KEYWORD_FALSE 
				=> {
					let tmp = ValueFalseContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3)?;
					_localctx = tmp;
					{
					recog.base.set_state(171);
					recog.base.match_token(YarnSpinnerParser_KEYWORD_FALSE,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_VAR_ID 
				=> {
					let tmp = ValueVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4)?;
					_localctx = tmp;
					{
					/*InvokeRule variable*/
					recog.base.set_state(172);
					recog.variable()?;

					}
				}

			YarnSpinnerParser_STRING 
				=> {
					let tmp = ValueStringContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5)?;
					_localctx = tmp;
					{
					recog.base.set_state(173);
					recog.base.match_token(YarnSpinnerParser_STRING,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_KEYWORD_NULL 
				=> {
					let tmp = ValueNullContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6)?;
					_localctx = tmp;
					{
					recog.base.set_state(174);
					recog.base.match_token(YarnSpinnerParser_KEYWORD_NULL,&mut recog.err_handler)?;

					}
				}

			YarnSpinnerParser_FUNC_ID 
				=> {
					let tmp = ValueFuncContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7)?;
					_localctx = tmp;
					{
					/*InvokeRule function_call*/
					recog.base.set_state(175);
					recog.function_call()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- variable ----------------
pub type VariableContextAll<'input> = VariableContext<'input>;


pub type VariableContext<'input> = BaseParserRuleContext<'input,VariableContextExt<'input>>;

#[derive(Clone)]
pub struct VariableContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for VariableContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for VariableContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_variable(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_variable(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for VariableContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_variable(self);
	}
}

impl<'input> CustomRuleContext<'input> for VariableContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_variable }
	//fn type_rule_index() -> usize where Self: Sized { RULE_variable }
}
antlr4rust::tid!{VariableContextExt<'a>}

impl<'input> VariableContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<VariableContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,VariableContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait VariableContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<VariableContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token VAR_ID
/// Returns `None` if there is no child corresponding to token VAR_ID
fn VAR_ID(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_VAR_ID, 0)
}

}

impl<'input> VariableContextAttrs<'input> for VariableContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn variable(&mut self,)
	-> Result<Rc<VariableContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = VariableContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_variable);
        let mut _localctx: Rc<VariableContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(178);
			recog.base.match_token(YarnSpinnerParser_VAR_ID,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- function_call ----------------
pub type Function_callContextAll<'input> = Function_callContext<'input>;


pub type Function_callContext<'input> = BaseParserRuleContext<'input,Function_callContextExt<'input>>;

#[derive(Clone)]
pub struct Function_callContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Function_callContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Function_callContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_function_call(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_function_call(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Function_callContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_function_call(self);
	}
}

impl<'input> CustomRuleContext<'input> for Function_callContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_function_call }
	//fn type_rule_index() -> usize where Self: Sized { RULE_function_call }
}
antlr4rust::tid!{Function_callContextExt<'a>}

impl<'input> Function_callContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Function_callContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Function_callContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Function_callContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Function_callContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token FUNC_ID
/// Returns `None` if there is no child corresponding to token FUNC_ID
fn FUNC_ID(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_FUNC_ID, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAREN
/// Returns `None` if there is no child corresponding to token LPAREN
fn LPAREN(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_LPAREN, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAREN
/// Returns `None` if there is no child corresponding to token RPAREN
fn RPAREN(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_RPAREN, 0)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMA, i)
}

}

impl<'input> Function_callContextAttrs<'input> for Function_callContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn function_call(&mut self,)
	-> Result<Rc<Function_callContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Function_callContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_function_call);
        let mut _localctx: Rc<Function_callContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(180);
			recog.base.match_token(YarnSpinnerParser_FUNC_ID,&mut recog.err_handler)?;

			recog.base.set_state(181);
			recog.base.match_token(YarnSpinnerParser_LPAREN,&mut recog.err_handler)?;

			recog.base.set_state(183);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & 1879048192) != 0) || ((((_la - 41)) & !0x3f) == 0 && ((1usize << (_la - 41)) & 1411201) != 0) {
				{
				/*InvokeRule expression*/
				recog.base.set_state(182);
				recog.expression_rec(0)?;

				}
			}

			recog.base.set_state(189);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==YarnSpinnerParser_COMMA {
				{
				{
				recog.base.set_state(185);
				recog.base.match_token(YarnSpinnerParser_COMMA,&mut recog.err_handler)?;

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
			recog.base.match_token(YarnSpinnerParser_RPAREN,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- if_statement ----------------
pub type If_statementContextAll<'input> = If_statementContext<'input>;


pub type If_statementContext<'input> = BaseParserRuleContext<'input,If_statementContextExt<'input>>;

#[derive(Clone)]
pub struct If_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for If_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for If_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_if_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_if_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for If_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_if_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_statement }
}
antlr4rust::tid!{If_statementContextExt<'a>}

impl<'input> If_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<If_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_statementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait If_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<If_statementContextExt<'input>>{

fn if_clause(&self) -> Option<Rc<If_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_ENDIF
/// Returns `None` if there is no child corresponding to token COMMAND_ENDIF
fn COMMAND_ENDIF(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_ENDIF, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
fn else_if_clause_all(&self) ->  Vec<Rc<Else_if_clauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn else_if_clause(&self, i: usize) -> Option<Rc<Else_if_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn else_clause(&self) -> Option<Rc<Else_clauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> If_statementContextAttrs<'input> for If_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn if_statement(&mut self,)
	-> Result<Rc<If_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_if_statement);
        let mut _localctx: Rc<If_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			/*InvokeRule if_clause*/
			recog.base.set_state(194);
			recog.if_clause()?;

			recog.base.set_state(198);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
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
				_alt = recog.interpreter.adaptive_predict(18,&mut recog.base)?;
			}
			recog.base.set_state(202);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				x if x == 1=>{
					{
					/*InvokeRule else_clause*/
					recog.base.set_state(201);
					recog.else_clause()?;

					}
				}

				_ => {}
			}
			recog.base.set_state(204);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(205);
			recog.base.match_token(YarnSpinnerParser_COMMAND_ENDIF,&mut recog.err_handler)?;

			recog.base.set_state(206);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- if_clause ----------------
pub type If_clauseContextAll<'input> = If_clauseContext<'input>;


pub type If_clauseContext<'input> = BaseParserRuleContext<'input,If_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct If_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for If_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for If_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_if_clause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_if_clause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for If_clauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_if_clause(self);
	}
}

impl<'input> CustomRuleContext<'input> for If_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_if_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_if_clause }
}
antlr4rust::tid!{If_clauseContextExt<'a>}

impl<'input> If_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<If_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,If_clauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait If_clauseContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<If_clauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_IF
/// Returns `None` if there is no child corresponding to token COMMAND_IF
fn COMMAND_IF(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_IF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> If_clauseContextAttrs<'input> for If_clauseContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn if_clause(&mut self,)
	-> Result<Rc<If_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = If_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_if_clause);
        let mut _localctx: Rc<If_clauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(208);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(209);
			recog.base.match_token(YarnSpinnerParser_COMMAND_IF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(210);
			recog.expression_rec(0)?;

			recog.base.set_state(211);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			recog.base.set_state(215);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
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
				_alt = recog.interpreter.adaptive_predict(20,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- else_if_clause ----------------
pub type Else_if_clauseContextAll<'input> = Else_if_clauseContext<'input>;


pub type Else_if_clauseContext<'input> = BaseParserRuleContext<'input,Else_if_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Else_if_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Else_if_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Else_if_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_else_if_clause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_else_if_clause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Else_if_clauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_else_if_clause(self);
	}
}

impl<'input> CustomRuleContext<'input> for Else_if_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_else_if_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_else_if_clause }
}
antlr4rust::tid!{Else_if_clauseContextExt<'a>}

impl<'input> Else_if_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Else_if_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Else_if_clauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Else_if_clauseContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Else_if_clauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_ELSEIF
/// Returns `None` if there is no child corresponding to token COMMAND_ELSEIF
fn COMMAND_ELSEIF(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_ELSEIF, 0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Else_if_clauseContextAttrs<'input> for Else_if_clauseContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn else_if_clause(&mut self,)
	-> Result<Rc<Else_if_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Else_if_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 32, RULE_else_if_clause);
        let mut _localctx: Rc<Else_if_clauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(218);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(219);
			recog.base.match_token(YarnSpinnerParser_COMMAND_ELSEIF,&mut recog.err_handler)?;

			/*InvokeRule expression*/
			recog.base.set_state(220);
			recog.expression_rec(0)?;

			recog.base.set_state(221);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			recog.base.set_state(225);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
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
				_alt = recog.interpreter.adaptive_predict(21,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- else_clause ----------------
pub type Else_clauseContextAll<'input> = Else_clauseContext<'input>;


pub type Else_clauseContext<'input> = BaseParserRuleContext<'input,Else_clauseContextExt<'input>>;

#[derive(Clone)]
pub struct Else_clauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Else_clauseContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Else_clauseContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_else_clause(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_else_clause(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Else_clauseContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_else_clause(self);
	}
}

impl<'input> CustomRuleContext<'input> for Else_clauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_else_clause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_else_clause }
}
antlr4rust::tid!{Else_clauseContextExt<'a>}

impl<'input> Else_clauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Else_clauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Else_clauseContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Else_clauseContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Else_clauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_ELSE
/// Returns `None` if there is no child corresponding to token COMMAND_ELSE
fn COMMAND_ELSE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_ELSE, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Else_clauseContextAttrs<'input> for Else_clauseContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn else_clause(&mut self,)
	-> Result<Rc<Else_clauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Else_clauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_else_clause);
        let mut _localctx: Rc<Else_clauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(228);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(229);
			recog.base.match_token(YarnSpinnerParser_COMMAND_ELSE,&mut recog.err_handler)?;

			recog.base.set_state(230);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			recog.base.set_state(234);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
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
				_alt = recog.interpreter.adaptive_predict(22,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- set_statement ----------------
pub type Set_statementContextAll<'input> = Set_statementContext<'input>;


pub type Set_statementContext<'input> = BaseParserRuleContext<'input,Set_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Set_statementContextExt<'input>{
	pub op: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Set_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Set_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_set_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_set_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Set_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_set_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Set_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_set_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_set_statement }
}
antlr4rust::tid!{Set_statementContextExt<'a>}

impl<'input> Set_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Set_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Set_statementContextExt{
				op: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait Set_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Set_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_SET
/// Returns `None` if there is no child corresponding to token COMMAND_SET
fn COMMAND_SET(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_SET, 0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_ASSIGNMENT
/// Returns `None` if there is no child corresponding to token OPERATOR_ASSIGNMENT
fn OPERATOR_ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_ASSIGNMENT, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MULTIPLICATION_EQUALS
/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MULTIPLICATION_EQUALS
fn OPERATOR_MATHS_MULTIPLICATION_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_MATHS_MULTIPLICATION_EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_DIVISION_EQUALS
/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_DIVISION_EQUALS
fn OPERATOR_MATHS_DIVISION_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_MATHS_DIVISION_EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_MODULUS_EQUALS
/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_MODULUS_EQUALS
fn OPERATOR_MATHS_MODULUS_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_MATHS_MODULUS_EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_ADDITION_EQUALS
/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_ADDITION_EQUALS
fn OPERATOR_MATHS_ADDITION_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_MATHS_ADDITION_EQUALS, 0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_MATHS_SUBTRACTION_EQUALS
/// Returns `None` if there is no child corresponding to token OPERATOR_MATHS_SUBTRACTION_EQUALS
fn OPERATOR_MATHS_SUBTRACTION_EQUALS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_MATHS_SUBTRACTION_EQUALS, 0)
}

}

impl<'input> Set_statementContextAttrs<'input> for Set_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn set_statement(&mut self,)
	-> Result<Rc<Set_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Set_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 36, RULE_set_statement);
        let mut _localctx: Rc<Set_statementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(237);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(238);
			recog.base.match_token(YarnSpinnerParser_COMMAND_SET,&mut recog.err_handler)?;

			/*InvokeRule variable*/
			recog.base.set_state(239);
			recog.variable()?;

			recog.base.set_state(240);
			 cast_mut::<_,Set_statementContext >(&mut _localctx).op = recog.base.input.lt(1).cloned();
			 
			_la = recog.base.input.la(1);
			if { !(((((_la - 31)) & !0x3f) == 0 && ((1usize << (_la - 31)) & 63489) != 0)) } {
				let tmp = recog.err_handler.recover_inline(&mut recog.base)?;
				 cast_mut::<_,Set_statementContext >(&mut _localctx).op = Some(tmp.clone());
				  

			}
			else {
				if  recog.base.input.la(1)==TOKEN_EOF { recog.base.matched_eof = true };
				recog.err_handler.report_match(&mut recog.base);
				recog.base.consume(&mut recog.err_handler);
			}
			/*InvokeRule expression*/
			recog.base.set_state(241);
			recog.expression_rec(0)?;

			recog.base.set_state(242);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- call_statement ----------------
pub type Call_statementContextAll<'input> = Call_statementContext<'input>;


pub type Call_statementContext<'input> = BaseParserRuleContext<'input,Call_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Call_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Call_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Call_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_call_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_call_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Call_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_call_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Call_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_call_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_call_statement }
}
antlr4rust::tid!{Call_statementContextExt<'a>}

impl<'input> Call_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Call_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Call_statementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Call_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Call_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_CALL
/// Returns `None` if there is no child corresponding to token COMMAND_CALL
fn COMMAND_CALL(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_CALL, 0)
}
fn function_call(&self) -> Option<Rc<Function_callContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}

}

impl<'input> Call_statementContextAttrs<'input> for Call_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn call_statement(&mut self,)
	-> Result<Rc<Call_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Call_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 38, RULE_call_statement);
        let mut _localctx: Rc<Call_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(244);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(245);
			recog.base.match_token(YarnSpinnerParser_COMMAND_CALL,&mut recog.err_handler)?;

			/*InvokeRule function_call*/
			recog.base.set_state(246);
			recog.function_call()?;

			recog.base.set_state(247);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- command_statement ----------------
pub type Command_statementContextAll<'input> = Command_statementContext<'input>;


pub type Command_statementContext<'input> = BaseParserRuleContext<'input,Command_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Command_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Command_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Command_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_command_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_command_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Command_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_command_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Command_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_command_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_command_statement }
}
antlr4rust::tid!{Command_statementContextExt<'a>}

impl<'input> Command_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Command_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Command_statementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Command_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Command_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
fn command_formatted_text(&self) -> Option<Rc<Command_formatted_textContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_TEXT_END
/// Returns `None` if there is no child corresponding to token COMMAND_TEXT_END
fn COMMAND_TEXT_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_TEXT_END, 0)
}
fn hashtag_all(&self) ->  Vec<Rc<HashtagContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn hashtag(&self, i: usize) -> Option<Rc<HashtagContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Command_statementContextAttrs<'input> for Command_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn command_statement(&mut self,)
	-> Result<Rc<Command_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Command_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 40, RULE_command_statement);
        let mut _localctx: Rc<Command_statementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(249);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			/*InvokeRule command_formatted_text*/
			recog.base.set_state(250);
			recog.command_formatted_text()?;

			recog.base.set_state(251);
			recog.base.match_token(YarnSpinnerParser_COMMAND_TEXT_END,&mut recog.err_handler)?;

			{
			recog.base.set_state(255);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==YarnSpinnerParser_HASHTAG {
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
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- command_formatted_text ----------------
pub type Command_formatted_textContextAll<'input> = Command_formatted_textContext<'input>;


pub type Command_formatted_textContext<'input> = BaseParserRuleContext<'input,Command_formatted_textContextExt<'input>>;

#[derive(Clone)]
pub struct Command_formatted_textContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Command_formatted_textContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Command_formatted_textContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_command_formatted_text(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_command_formatted_text(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Command_formatted_textContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_command_formatted_text(self);
	}
}

impl<'input> CustomRuleContext<'input> for Command_formatted_textContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_command_formatted_text }
	//fn type_rule_index() -> usize where Self: Sized { RULE_command_formatted_text }
}
antlr4rust::tid!{Command_formatted_textContextExt<'a>}

impl<'input> Command_formatted_textContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Command_formatted_textContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Command_formatted_textContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Command_formatted_textContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Command_formatted_textContextExt<'input>>{

/// Retrieves all `TerminalNode`s corresponding to token COMMAND_TEXT in current rule
fn COMMAND_TEXT_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMAND_TEXT, starting from 0.
/// Returns `None` if number of children corresponding to token COMMAND_TEXT is less or equal than `i`.
fn COMMAND_TEXT(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_TEXT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMAND_EXPRESSION_START in current rule
fn COMMAND_EXPRESSION_START_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMAND_EXPRESSION_START, starting from 0.
/// Returns `None` if number of children corresponding to token COMMAND_EXPRESSION_START is less or equal than `i`.
fn COMMAND_EXPRESSION_START(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_EXPRESSION_START, i)
}
fn expression_all(&self) ->  Vec<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn expression(&self, i: usize) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token EXPRESSION_END in current rule
fn EXPRESSION_END_all(&self) -> Vec<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token EXPRESSION_END, starting from 0.
/// Returns `None` if number of children corresponding to token EXPRESSION_END is less or equal than `i`.
fn EXPRESSION_END(&self, i: usize) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_EXPRESSION_END, i)
}

}

impl<'input> Command_formatted_textContextAttrs<'input> for Command_formatted_textContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn command_formatted_text(&mut self,)
	-> Result<Rc<Command_formatted_textContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Command_formatted_textContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_command_formatted_text);
        let mut _localctx: Rc<Command_formatted_textContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(265);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==YarnSpinnerParser_COMMAND_EXPRESSION_START || _la==YarnSpinnerParser_COMMAND_TEXT {
				{
				recog.base.set_state(263);
				recog.err_handler.sync(&mut recog.base)?;
				match recog.base.input.la(1) {
				YarnSpinnerParser_COMMAND_TEXT 
					=> {
						{
						recog.base.set_state(258);
						recog.base.match_token(YarnSpinnerParser_COMMAND_TEXT,&mut recog.err_handler)?;

						}
					}

				YarnSpinnerParser_COMMAND_EXPRESSION_START 
					=> {
						{
						recog.base.set_state(259);
						recog.base.match_token(YarnSpinnerParser_COMMAND_EXPRESSION_START,&mut recog.err_handler)?;

						/*InvokeRule expression*/
						recog.base.set_state(260);
						recog.expression_rec(0)?;

						recog.base.set_state(261);
						recog.base.match_token(YarnSpinnerParser_EXPRESSION_END,&mut recog.err_handler)?;

						}
					}

					_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
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
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- shortcut_option_statement ----------------
pub type Shortcut_option_statementContextAll<'input> = Shortcut_option_statementContext<'input>;


pub type Shortcut_option_statementContext<'input> = BaseParserRuleContext<'input,Shortcut_option_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Shortcut_option_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Shortcut_option_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Shortcut_option_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_shortcut_option_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_shortcut_option_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Shortcut_option_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_shortcut_option_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Shortcut_option_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shortcut_option_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shortcut_option_statement }
}
antlr4rust::tid!{Shortcut_option_statementContextExt<'a>}

impl<'input> Shortcut_option_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Shortcut_option_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Shortcut_option_statementContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Shortcut_option_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Shortcut_option_statementContextExt<'input>>{

fn shortcut_option_all(&self) ->  Vec<Rc<Shortcut_optionContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn shortcut_option(&self, i: usize) -> Option<Rc<Shortcut_optionContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token BLANK_LINE_FOLLOWING_OPTION
/// Returns `None` if there is no child corresponding to token BLANK_LINE_FOLLOWING_OPTION
fn BLANK_LINE_FOLLOWING_OPTION(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_BLANK_LINE_FOLLOWING_OPTION, 0)
}

}

impl<'input> Shortcut_option_statementContextAttrs<'input> for Shortcut_option_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn shortcut_option_statement(&mut self,)
	-> Result<Rc<Shortcut_option_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Shortcut_option_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_shortcut_option_statement);
        let mut _localctx: Rc<Shortcut_option_statementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: i32;
			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(271);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
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
				_alt = recog.interpreter.adaptive_predict(26,&mut recog.base)?;
			}
			{
			/*InvokeRule shortcut_option*/
			recog.base.set_state(274);
			recog.shortcut_option()?;

			recog.base.set_state(276);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==YarnSpinnerParser_BLANK_LINE_FOLLOWING_OPTION {
				{
				recog.base.set_state(275);
				recog.base.match_token(YarnSpinnerParser_BLANK_LINE_FOLLOWING_OPTION,&mut recog.err_handler)?;

				}
			}

			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- shortcut_option ----------------
pub type Shortcut_optionContextAll<'input> = Shortcut_optionContext<'input>;


pub type Shortcut_optionContext<'input> = BaseParserRuleContext<'input,Shortcut_optionContextExt<'input>>;

#[derive(Clone)]
pub struct Shortcut_optionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Shortcut_optionContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Shortcut_optionContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_shortcut_option(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_shortcut_option(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Shortcut_optionContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_shortcut_option(self);
	}
}

impl<'input> CustomRuleContext<'input> for Shortcut_optionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_shortcut_option }
	//fn type_rule_index() -> usize where Self: Sized { RULE_shortcut_option }
}
antlr4rust::tid!{Shortcut_optionContextExt<'a>}

impl<'input> Shortcut_optionContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Shortcut_optionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Shortcut_optionContextExt{

				ph:PhantomData
			}),
		)
	}
}

pub trait Shortcut_optionContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Shortcut_optionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token SHORTCUT_ARROW
/// Returns `None` if there is no child corresponding to token SHORTCUT_ARROW
fn SHORTCUT_ARROW(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_SHORTCUT_ARROW, 0)
}
fn line_statement(&self) -> Option<Rc<Line_statementContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token INDENT
/// Returns `None` if there is no child corresponding to token INDENT
fn INDENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_INDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token DEDENT
/// Returns `None` if there is no child corresponding to token DEDENT
fn DEDENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_DEDENT, 0)
}
fn statement_all(&self) ->  Vec<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn statement(&self, i: usize) -> Option<Rc<StatementContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> Shortcut_optionContextAttrs<'input> for Shortcut_optionContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn shortcut_option(&mut self,)
	-> Result<Rc<Shortcut_optionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Shortcut_optionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_shortcut_option);
        let mut _localctx: Rc<Shortcut_optionContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(278);
			recog.base.match_token(YarnSpinnerParser_SHORTCUT_ARROW,&mut recog.err_handler)?;

			/*InvokeRule line_statement*/
			recog.base.set_state(279);
			recog.line_statement()?;

			recog.base.set_state(288);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(29,&mut recog.base)? {
				x if x == 1=>{
					{
					recog.base.set_state(280);
					recog.base.match_token(YarnSpinnerParser_INDENT,&mut recog.err_handler)?;

					recog.base.set_state(284);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & 1163266) != 0) {
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
					recog.base.match_token(YarnSpinnerParser_DEDENT,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- declare_statement ----------------
pub type Declare_statementContextAll<'input> = Declare_statementContext<'input>;


pub type Declare_statementContext<'input> = BaseParserRuleContext<'input,Declare_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Declare_statementContextExt<'input>{
	pub type_: Option<TokenType<'input>>,
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Declare_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Declare_statementContext<'input>{
		fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.enter_every_rule(self)?;
			listener.enter_declare_statement(self);
			Ok(())
		}
		fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
			listener.exit_declare_statement(self);
			listener.exit_every_rule(self)?;
			Ok(())
		}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Declare_statementContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_declare_statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for Declare_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declare_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declare_statement }
}
antlr4rust::tid!{Declare_statementContextExt<'a>}

impl<'input> Declare_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Declare_statementContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Declare_statementContextExt{
				type_: None, 

				ph:PhantomData
			}),
		)
	}
}

pub trait Declare_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Declare_statementContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token COMMAND_START
/// Returns `None` if there is no child corresponding to token COMMAND_START
fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_START, 0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_DECLARE
/// Returns `None` if there is no child corresponding to token COMMAND_DECLARE
fn COMMAND_DECLARE(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_DECLARE, 0)
}
fn variable(&self) -> Option<Rc<VariableContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token OPERATOR_ASSIGNMENT
/// Returns `None` if there is no child corresponding to token OPERATOR_ASSIGNMENT
fn OPERATOR_ASSIGNMENT(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_OPERATOR_ASSIGNMENT, 0)
}
fn value(&self) -> Option<Rc<ValueContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token COMMAND_END
/// Returns `None` if there is no child corresponding to token COMMAND_END
fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_COMMAND_END, 0)
}
/// Retrieves first TerminalNode corresponding to token EXPRESSION_AS
/// Returns `None` if there is no child corresponding to token EXPRESSION_AS
fn EXPRESSION_AS(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_EXPRESSION_AS, 0)
}
/// Retrieves first TerminalNode corresponding to token FUNC_ID
/// Returns `None` if there is no child corresponding to token FUNC_ID
fn FUNC_ID(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
	self.get_token(YarnSpinnerParser_FUNC_ID, 0)
}

}

impl<'input> Declare_statementContextAttrs<'input> for Declare_statementContext<'input>{}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn declare_statement(&mut self,)
	-> Result<Rc<Declare_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Declare_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_declare_statement);
        let mut _localctx: Rc<Declare_statementContextAll> = _localctx;
		let mut _la: i32 = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1)?;
			recog.base.enter_outer_alt(None, 1)?;
			{
			recog.base.set_state(290);
			recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

			recog.base.set_state(291);
			recog.base.match_token(YarnSpinnerParser_COMMAND_DECLARE,&mut recog.err_handler)?;

			/*InvokeRule variable*/
			recog.base.set_state(292);
			recog.variable()?;

			recog.base.set_state(293);
			recog.base.match_token(YarnSpinnerParser_OPERATOR_ASSIGNMENT,&mut recog.err_handler)?;

			/*InvokeRule value*/
			recog.base.set_state(294);
			recog.value()?;

			recog.base.set_state(297);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==YarnSpinnerParser_EXPRESSION_AS {
				{
				recog.base.set_state(295);
				recog.base.match_token(YarnSpinnerParser_EXPRESSION_AS,&mut recog.err_handler)?;

				recog.base.set_state(296);
				let tmp = recog.base.match_token(YarnSpinnerParser_FUNC_ID,&mut recog.err_handler)?;
				 cast_mut::<_,Declare_statementContext >(&mut _localctx).type_ = Some(tmp.clone());
				  

				}
			}

			recog.base.set_state(299);
			recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
//------------------- jump_statement ----------------
#[derive(Debug)]
pub enum Jump_statementContextAll<'input>{
	JumpToNodeNameContext(JumpToNodeNameContext<'input>),
	JumpToExpressionContext(JumpToExpressionContext<'input>),
Error(Jump_statementContext<'input>)
}
antlr4rust::tid!{Jump_statementContextAll<'a>}

impl<'input> antlr4rust::parser_rule_context::DerefSeal for Jump_statementContextAll<'input>{}

impl<'input> YarnSpinnerParserContext<'input> for Jump_statementContextAll<'input>{}

impl<'input> Deref for Jump_statementContextAll<'input>{
	type Target = dyn Jump_statementContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use Jump_statementContextAll::*;
		match self{
			JumpToNodeNameContext(inner) => inner,
			JumpToExpressionContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Jump_statementContextAll<'input>{
	fn accept(&self, visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) { self.deref().accept(visitor) }
}
impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Jump_statementContextAll<'input>{
    fn enter(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> { self.deref().exit(listener) }
}



pub type Jump_statementContext<'input> = BaseParserRuleContext<'input,Jump_statementContextExt<'input>>;

#[derive(Clone)]
pub struct Jump_statementContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> YarnSpinnerParserContext<'input> for Jump_statementContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for Jump_statementContext<'input>{
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for Jump_statementContext<'input>{
}

impl<'input> CustomRuleContext<'input> for Jump_statementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jump_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}
antlr4rust::tid!{Jump_statementContextExt<'a>}

impl<'input> Jump_statementContextExt<'input>{
	fn new(parent: Option<Rc<dyn YarnSpinnerParserContext<'input> + 'input > >, invoking_state: i32) -> Rc<Jump_statementContextAll<'input>> {
		Rc::new(
		Jump_statementContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,Jump_statementContextExt{

				ph:PhantomData
			}),
		)
		)
	}
}

pub trait Jump_statementContextAttrs<'input>: YarnSpinnerParserContext<'input> + BorrowMut<Jump_statementContextExt<'input>>{


}

impl<'input> Jump_statementContextAttrs<'input> for Jump_statementContext<'input>{}

pub type JumpToNodeNameContext<'input> = BaseParserRuleContext<'input,JumpToNodeNameContextExt<'input>>;

pub trait JumpToNodeNameContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COMMAND_START
	/// Returns `None` if there is no child corresponding to token COMMAND_START
	fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_START, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMMAND_JUMP
	/// Returns `None` if there is no child corresponding to token COMMAND_JUMP
	fn COMMAND_JUMP(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_JUMP, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMMAND_END
	/// Returns `None` if there is no child corresponding to token COMMAND_END
	fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_END, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ID
	/// Returns `None` if there is no child corresponding to token ID
	fn ID(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_ID, 0)
	}
}

impl<'input> JumpToNodeNameContextAttrs<'input> for JumpToNodeNameContext<'input>{}

pub struct JumpToNodeNameContextExt<'input>{
	base:Jump_statementContextExt<'input>,
	pub destination: Option<TokenType<'input>>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{JumpToNodeNameContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for JumpToNodeNameContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for JumpToNodeNameContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_jumpToNodeName(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_jumpToNodeName(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for JumpToNodeNameContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_jumpToNodeName(self);
	}
}

impl<'input> CustomRuleContext<'input> for JumpToNodeNameContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jump_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}

impl<'input> Borrow<Jump_statementContextExt<'input>> for JumpToNodeNameContext<'input>{
	fn borrow(&self) -> &Jump_statementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Jump_statementContextExt<'input>> for JumpToNodeNameContext<'input>{
	fn borrow_mut(&mut self) -> &mut Jump_statementContextExt<'input> { &mut self.base }
}

impl<'input> Jump_statementContextAttrs<'input> for JumpToNodeNameContext<'input> {}

impl<'input> JumpToNodeNameContextExt<'input>{
	fn new(ctx: &dyn Jump_statementContextAttrs<'input>) -> Rc<Jump_statementContextAll<'input>>  {
		Rc::new(
			Jump_statementContextAll::JumpToNodeNameContext(
				BaseParserRuleContext::copy_from(ctx,JumpToNodeNameContextExt{
					destination:None, 
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type JumpToExpressionContext<'input> = BaseParserRuleContext<'input,JumpToExpressionContextExt<'input>>;

pub trait JumpToExpressionContextAttrs<'input>: YarnSpinnerParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token COMMAND_START
	/// Returns `None` if there is no child corresponding to token COMMAND_START
	fn COMMAND_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_START, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMMAND_JUMP
	/// Returns `None` if there is no child corresponding to token COMMAND_JUMP
	fn COMMAND_JUMP(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_JUMP, 0)
	}
	/// Retrieves first TerminalNode corresponding to token EXPRESSION_START
	/// Returns `None` if there is no child corresponding to token EXPRESSION_START
	fn EXPRESSION_START(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_EXPRESSION_START, 0)
	}
	fn expression(&self) -> Option<Rc<ExpressionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token EXPRESSION_END
	/// Returns `None` if there is no child corresponding to token EXPRESSION_END
	fn EXPRESSION_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_EXPRESSION_END, 0)
	}
	/// Retrieves first TerminalNode corresponding to token COMMAND_END
	/// Returns `None` if there is no child corresponding to token COMMAND_END
	fn COMMAND_END(&self) -> Option<Rc<TerminalNode<'input,YarnSpinnerParserContextType>>> where Self:Sized{
		self.get_token(YarnSpinnerParser_COMMAND_END, 0)
	}
}

impl<'input> JumpToExpressionContextAttrs<'input> for JumpToExpressionContext<'input>{}

pub struct JumpToExpressionContextExt<'input>{
	base:Jump_statementContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr4rust::tid!{JumpToExpressionContextExt<'a>}

impl<'input> YarnSpinnerParserContext<'input> for JumpToExpressionContext<'input>{}

impl<'input,'a> Listenable<dyn YarnSpinnerParserListener<'input> + 'a> for JumpToExpressionContext<'input>{
	fn enter(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.enter_every_rule(self)?;
		listener.enter_jumpToExpression(self);
		Ok(())
	}
	fn exit(&self,listener: &mut (dyn YarnSpinnerParserListener<'input> + 'a)) -> Result<(), ANTLRError> {
		listener.exit_jumpToExpression(self);
		listener.exit_every_rule(self)?;
		Ok(())
	}
}

impl<'input,'a> Visitable<dyn YarnSpinnerParserVisitor<'input> + 'a> for JumpToExpressionContext<'input>{
	fn accept(&self,visitor: &mut (dyn YarnSpinnerParserVisitor<'input> + 'a)) {
		visitor.visit_jumpToExpression(self);
	}
}

impl<'input> CustomRuleContext<'input> for JumpToExpressionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = YarnSpinnerParserContextType;
	fn get_rule_index(&self) -> usize { RULE_jump_statement }
	//fn type_rule_index() -> usize where Self: Sized { RULE_jump_statement }
}

impl<'input> Borrow<Jump_statementContextExt<'input>> for JumpToExpressionContext<'input>{
	fn borrow(&self) -> &Jump_statementContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<Jump_statementContextExt<'input>> for JumpToExpressionContext<'input>{
	fn borrow_mut(&mut self) -> &mut Jump_statementContextExt<'input> { &mut self.base }
}

impl<'input> Jump_statementContextAttrs<'input> for JumpToExpressionContext<'input> {}

impl<'input> JumpToExpressionContextExt<'input>{
	fn new(ctx: &dyn Jump_statementContextAttrs<'input>) -> Rc<Jump_statementContextAll<'input>>  {
		Rc::new(
			Jump_statementContextAll::JumpToExpressionContext(
				BaseParserRuleContext::copy_from(ctx,JumpToExpressionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I> YarnSpinnerParser<'input, I>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	pub fn jump_statement(&mut self,)
	-> Result<Rc<Jump_statementContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = Jump_statementContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_jump_statement);
        let mut _localctx: Rc<Jump_statementContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(312);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
				1 =>{
					let tmp = JumpToNodeNameContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1)?;
					_localctx = tmp;
					{
					recog.base.set_state(301);
					recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

					recog.base.set_state(302);
					recog.base.match_token(YarnSpinnerParser_COMMAND_JUMP,&mut recog.err_handler)?;

					recog.base.set_state(303);
					let tmp = recog.base.match_token(YarnSpinnerParser_ID,&mut recog.err_handler)?;
					if let Jump_statementContextAll::JumpToNodeNameContext(ctx) = cast_mut::<_,Jump_statementContextAll >(&mut _localctx){
					ctx.destination = Some(tmp.clone()); } else {unreachable!("cant cast");}  

					recog.base.set_state(304);
					recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = JumpToExpressionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2)?;
					_localctx = tmp;
					{
					recog.base.set_state(305);
					recog.base.match_token(YarnSpinnerParser_COMMAND_START,&mut recog.err_handler)?;

					recog.base.set_state(306);
					recog.base.match_token(YarnSpinnerParser_COMMAND_JUMP,&mut recog.err_handler)?;

					recog.base.set_state(307);
					recog.base.match_token(YarnSpinnerParser_EXPRESSION_START,&mut recog.err_handler)?;

					/*InvokeRule expression*/
					recog.base.set_state(308);
					recog.expression_rec(0)?;

					recog.base.set_state(309);
					recog.base.match_token(YarnSpinnerParser_EXPRESSION_END,&mut recog.err_handler)?;

					recog.base.set_state(310);
					recog.base.match_token(YarnSpinnerParser_COMMAND_END,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule()?;

		Ok(_localctx)
	}
}
	lazy_static!{
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(&mut _serializedATN.iter()));
    static ref _decision_to_DFA: Arc<Vec<antlr4rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len() as i32;
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i,
            ).into())
        }
        Arc::new(dfa)
    };
	static ref _serializedATN: Vec<i32> = vec![
		4, 1, 84, 315, 2, 0, 7, 0, 2, 1, 7, 1, 2, 2, 7, 2, 2, 3, 7, 3, 2, 4, 7, 
		4, 2, 5, 7, 5, 2, 6, 7, 6, 2, 7, 7, 7, 2, 8, 7, 8, 2, 9, 7, 9, 2, 10, 
		7, 10, 2, 11, 7, 11, 2, 12, 7, 12, 2, 13, 7, 13, 2, 14, 7, 14, 2, 15, 
		7, 15, 2, 16, 7, 16, 2, 17, 7, 17, 2, 18, 7, 18, 2, 19, 7, 19, 2, 20, 
		7, 20, 2, 21, 7, 21, 2, 22, 7, 22, 2, 23, 7, 23, 2, 24, 7, 24, 2, 25, 
		7, 25, 1, 0, 5, 0, 54, 8, 0, 10, 0, 12, 0, 57, 9, 0, 1, 0, 4, 0, 60, 8, 
		0, 11, 0, 12, 0, 61, 1, 1, 1, 1, 1, 1, 1, 2, 4, 2, 68, 8, 2, 11, 2, 12, 
		2, 69, 1, 2, 1, 2, 1, 2, 1, 2, 1, 3, 1, 3, 1, 3, 3, 3, 79, 8, 3, 1, 4, 
		5, 4, 82, 8, 4, 10, 4, 12, 4, 85, 9, 4, 1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 
		1, 5, 1, 5, 1, 5, 1, 5, 1, 5, 5, 5, 97, 8, 5, 10, 5, 12, 5, 100, 9, 5, 
		1, 5, 3, 5, 103, 8, 5, 1, 6, 1, 6, 3, 6, 107, 8, 6, 1, 6, 5, 6, 110, 8, 
		6, 10, 6, 12, 6, 113, 9, 6, 1, 6, 1, 6, 1, 7, 4, 7, 118, 8, 7, 11, 7, 
		12, 7, 119, 1, 7, 1, 7, 1, 7, 1, 7, 4, 7, 126, 8, 7, 11, 7, 12, 7, 127, 
		1, 8, 1, 8, 1, 8, 1, 9, 1, 9, 1, 9, 1, 9, 1, 9, 1, 10, 1, 10, 1, 10, 1, 
		10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 3, 10, 148, 8, 10, 1, 10, 
		1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 1, 10, 
		1, 10, 1, 10, 1, 10, 1, 10, 5, 10, 165, 8, 10, 10, 10, 12, 10, 168, 9, 
		10, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 1, 11, 3, 11, 177, 8, 11, 
		1, 12, 1, 12, 1, 13, 1, 13, 1, 13, 3, 13, 184, 8, 13, 1, 13, 1, 13, 5, 
		13, 188, 8, 13, 10, 13, 12, 13, 191, 9, 13, 1, 13, 1, 13, 1, 14, 1, 14, 
		5, 14, 197, 8, 14, 10, 14, 12, 14, 200, 9, 14, 1, 14, 3, 14, 203, 8, 14, 
		1, 14, 1, 14, 1, 14, 1, 14, 1, 15, 1, 15, 1, 15, 1, 15, 1, 15, 5, 15, 
		214, 8, 15, 10, 15, 12, 15, 217, 9, 15, 1, 16, 1, 16, 1, 16, 1, 16, 1, 
		16, 5, 16, 224, 8, 16, 10, 16, 12, 16, 227, 9, 16, 1, 17, 1, 17, 1, 17, 
		1, 17, 5, 17, 233, 8, 17, 10, 17, 12, 17, 236, 9, 17, 1, 18, 1, 18, 1, 
		18, 1, 18, 1, 18, 1, 18, 1, 18, 1, 19, 1, 19, 1, 19, 1, 19, 1, 19, 1, 
		20, 1, 20, 1, 20, 1, 20, 5, 20, 254, 8, 20, 10, 20, 12, 20, 257, 9, 20, 
		1, 21, 1, 21, 1, 21, 1, 21, 1, 21, 5, 21, 264, 8, 21, 10, 21, 12, 21, 
		267, 9, 21, 1, 22, 5, 22, 270, 8, 22, 10, 22, 12, 22, 273, 9, 22, 1, 22, 
		1, 22, 3, 22, 277, 8, 22, 1, 23, 1, 23, 1, 23, 1, 23, 5, 23, 283, 8, 23, 
		10, 23, 12, 23, 286, 9, 23, 1, 23, 3, 23, 289, 8, 23, 1, 24, 1, 24, 1, 
		24, 1, 24, 1, 24, 1, 24, 1, 24, 3, 24, 298, 8, 24, 1, 24, 1, 24, 1, 25, 
		1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 1, 25, 
		3, 25, 313, 8, 25, 1, 25, 0, 1, 20, 26, 0, 2, 4, 6, 8, 10, 12, 14, 16, 
		18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48, 50, 0, 
		6, 1, 0, 49, 51, 1, 0, 47, 48, 2, 0, 32, 33, 35, 36, 2, 0, 34, 34, 37, 
		37, 1, 0, 38, 40, 2, 0, 31, 31, 42, 46, 337, 0, 55, 1, 0, 0, 0, 2, 63, 
		1, 0, 0, 0, 4, 67, 1, 0, 0, 0, 6, 75, 1, 0, 0, 0, 8, 83, 1, 0, 0, 0, 10, 
		102, 1, 0, 0, 0, 12, 104, 1, 0, 0, 0, 14, 125, 1, 0, 0, 0, 16, 129, 1, 
		0, 0, 0, 18, 132, 1, 0, 0, 0, 20, 147, 1, 0, 0, 0, 22, 176, 1, 0, 0, 0, 
		24, 178, 1, 0, 0, 0, 26, 180, 1, 0, 0, 0, 28, 194, 1, 0, 0, 0, 30, 208, 
		1, 0, 0, 0, 32, 218, 1, 0, 0, 0, 34, 228, 1, 0, 0, 0, 36, 237, 1, 0, 0, 
		0, 38, 244, 1, 0, 0, 0, 40, 249, 1, 0, 0, 0, 42, 265, 1, 0, 0, 0, 44, 
		271, 1, 0, 0, 0, 46, 278, 1, 0, 0, 0, 48, 290, 1, 0, 0, 0, 50, 312, 1, 
		0, 0, 0, 52, 54, 3, 2, 1, 0, 53, 52, 1, 0, 0, 0, 54, 57, 1, 0, 0, 0, 55, 
		53, 1, 0, 0, 0, 55, 56, 1, 0, 0, 0, 56, 59, 1, 0, 0, 0, 57, 55, 1, 0, 
		0, 0, 58, 60, 3, 4, 2, 0, 59, 58, 1, 0, 0, 0, 60, 61, 1, 0, 0, 0, 61, 
		59, 1, 0, 0, 0, 61, 62, 1, 0, 0, 0, 62, 1, 1, 0, 0, 0, 63, 64, 5, 10, 
		0, 0, 64, 65, 5, 26, 0, 0, 65, 3, 1, 0, 0, 0, 66, 68, 3, 6, 3, 0, 67, 
		66, 1, 0, 0, 0, 68, 69, 1, 0, 0, 0, 69, 67, 1, 0, 0, 0, 69, 70, 1, 0, 
		0, 0, 70, 71, 1, 0, 0, 0, 71, 72, 5, 8, 0, 0, 72, 73, 3, 8, 4, 0, 73, 
		74, 5, 13, 0, 0, 74, 5, 1, 0, 0, 0, 75, 76, 5, 7, 0, 0, 76, 78, 5, 9, 
		0, 0, 77, 79, 5, 11, 0, 0, 78, 77, 1, 0, 0, 0, 78, 79, 1, 0, 0, 0, 79, 
		7, 1, 0, 0, 0, 80, 82, 3, 10, 5, 0, 81, 80, 1, 0, 0, 0, 82, 85, 1, 0, 
		0, 0, 83, 81, 1, 0, 0, 0, 83, 84, 1, 0, 0, 0, 84, 9, 1, 0, 0, 0, 85, 83, 
		1, 0, 0, 0, 86, 103, 3, 12, 6, 0, 87, 103, 3, 28, 14, 0, 88, 103, 3, 36, 
		18, 0, 89, 103, 3, 44, 22, 0, 90, 103, 3, 38, 19, 0, 91, 103, 3, 40, 20, 
		0, 92, 103, 3, 48, 24, 0, 93, 103, 3, 50, 25, 0, 94, 98, 5, 1, 0, 0, 95, 
		97, 3, 10, 5, 0, 96, 95, 1, 0, 0, 0, 97, 100, 1, 0, 0, 0, 98, 96, 1, 0, 
		0, 0, 98, 99, 1, 0, 0, 0, 99, 101, 1, 0, 0, 0, 100, 98, 1, 0, 0, 0, 101, 
		103, 5, 2, 0, 0, 102, 86, 1, 0, 0, 0, 102, 87, 1, 0, 0, 0, 102, 88, 1, 
		0, 0, 0, 102, 89, 1, 0, 0, 0, 102, 90, 1, 0, 0, 0, 102, 91, 1, 0, 0, 0, 
		102, 92, 1, 0, 0, 0, 102, 93, 1, 0, 0, 0, 102, 94, 1, 0, 0, 0, 103, 11, 
		1, 0, 0, 0, 104, 106, 3, 14, 7, 0, 105, 107, 3, 18, 9, 0, 106, 105, 1, 
		0, 0, 0, 106, 107, 1, 0, 0, 0, 107, 111, 1, 0, 0, 0, 108, 110, 3, 16, 
		8, 0, 109, 108, 1, 0, 0, 0, 110, 113, 1, 0, 0, 0, 111, 109, 1, 0, 0, 0, 
		111, 112, 1, 0, 0, 0, 112, 114, 1, 0, 0, 0, 113, 111, 1, 0, 0, 0, 114, 
		115, 5, 6, 0, 0, 115, 13, 1, 0, 0, 0, 116, 118, 5, 20, 0, 0, 117, 116, 
		1, 0, 0, 0, 118, 119, 1, 0, 0, 0, 119, 117, 1, 0, 0, 0, 119, 120, 1, 0, 
		0, 0, 120, 126, 1, 0, 0, 0, 121, 122, 5, 16, 0, 0, 122, 123, 3, 20, 10, 
		0, 123, 124, 5, 58, 0, 0, 124, 126, 1, 0, 0, 0, 125, 117, 1, 0, 0, 0, 
		125, 121, 1, 0, 0, 0, 126, 127, 1, 0, 0, 0, 127, 125, 1, 0, 0, 0, 127, 
		128, 1, 0, 0, 0, 128, 15, 1, 0, 0, 0, 129, 130, 5, 10, 0, 0, 130, 131, 
		5, 26, 0, 0, 131, 17, 1, 0, 0, 0, 132, 133, 5, 15, 0, 0, 133, 134, 5, 
		64, 0, 0, 134, 135, 3, 20, 10, 0, 135, 136, 5, 76, 0, 0, 136, 19, 1, 0, 
		0, 0, 137, 138, 6, 10, -1, 0, 138, 139, 5, 52, 0, 0, 139, 140, 3, 20, 
		10, 0, 140, 141, 5, 53, 0, 0, 141, 148, 1, 0, 0, 0, 142, 143, 5, 48, 0, 
		0, 143, 148, 3, 20, 10, 8, 144, 145, 5, 41, 0, 0, 145, 148, 3, 20, 10, 
		7, 146, 148, 3, 22, 11, 0, 147, 137, 1, 0, 0, 0, 147, 142, 1, 0, 0, 0, 
		147, 144, 1, 0, 0, 0, 147, 146, 1, 0, 0, 0, 148, 166, 1, 0, 0, 0, 149, 
		150, 10, 6, 0, 0, 150, 151, 7, 0, 0, 0, 151, 165, 3, 20, 10, 7, 152, 153, 
		10, 5, 0, 0, 153, 154, 7, 1, 0, 0, 154, 165, 3, 20, 10, 6, 155, 156, 10, 
		4, 0, 0, 156, 157, 7, 2, 0, 0, 157, 165, 3, 20, 10, 5, 158, 159, 10, 3, 
		0, 0, 159, 160, 7, 3, 0, 0, 160, 165, 3, 20, 10, 4, 161, 162, 10, 2, 0, 
		0, 162, 163, 7, 4, 0, 0, 163, 165, 3, 20, 10, 3, 164, 149, 1, 0, 0, 0, 
		164, 152, 1, 0, 0, 0, 164, 155, 1, 0, 0, 0, 164, 158, 1, 0, 0, 0, 164, 
		161, 1, 0, 0, 0, 165, 168, 1, 0, 0, 0, 166, 164, 1, 0, 0, 0, 166, 167, 
		1, 0, 0, 0, 167, 21, 1, 0, 0, 0, 168, 166, 1, 0, 0, 0, 169, 177, 5, 61, 
		0, 0, 170, 177, 5, 28, 0, 0, 171, 177, 5, 29, 0, 0, 172, 177, 3, 24, 12, 
		0, 173, 177, 5, 56, 0, 0, 174, 177, 5, 30, 0, 0, 175, 177, 3, 26, 13, 
		0, 176, 169, 1, 0, 0, 0, 176, 170, 1, 0, 0, 0, 176, 171, 1, 0, 0, 0, 176, 
		172, 1, 0, 0, 0, 176, 173, 1, 0, 0, 0, 176, 174, 1, 0, 0, 0, 176, 175, 
		1, 0, 0, 0, 177, 23, 1, 0, 0, 0, 178, 179, 5, 59, 0, 0, 179, 25, 1, 0, 
		0, 0, 180, 181, 5, 57, 0, 0, 181, 183, 5, 52, 0, 0, 182, 184, 3, 20, 10, 
		0, 183, 182, 1, 0, 0, 0, 183, 184, 1, 0, 0, 0, 184, 189, 1, 0, 0, 0, 185, 
		186, 5, 54, 0, 0, 186, 188, 3, 20, 10, 0, 187, 185, 1, 0, 0, 0, 188, 191, 
		1, 0, 0, 0, 189, 187, 1, 0, 0, 0, 189, 190, 1, 0, 0, 0, 190, 192, 1, 0, 
		0, 0, 191, 189, 1, 0, 0, 0, 192, 193, 5, 53, 0, 0, 193, 27, 1, 0, 0, 0, 
		194, 198, 3, 30, 15, 0, 195, 197, 3, 32, 16, 0, 196, 195, 1, 0, 0, 0, 
		197, 200, 1, 0, 0, 0, 198, 196, 1, 0, 0, 0, 198, 199, 1, 0, 0, 0, 199, 
		202, 1, 0, 0, 0, 200, 198, 1, 0, 0, 0, 201, 203, 3, 34, 17, 0, 202, 201, 
		1, 0, 0, 0, 202, 203, 1, 0, 0, 0, 203, 204, 1, 0, 0, 0, 204, 205, 5, 15, 
		0, 0, 205, 206, 5, 68, 0, 0, 206, 207, 5, 76, 0, 0, 207, 29, 1, 0, 0, 
		0, 208, 209, 5, 15, 0, 0, 209, 210, 5, 64, 0, 0, 210, 211, 3, 20, 10, 
		0, 211, 215, 5, 76, 0, 0, 212, 214, 3, 10, 5, 0, 213, 212, 1, 0, 0, 0, 
		214, 217, 1, 0, 0, 0, 215, 213, 1, 0, 0, 0, 215, 216, 1, 0, 0, 0, 216, 
		31, 1, 0, 0, 0, 217, 215, 1, 0, 0, 0, 218, 219, 5, 15, 0, 0, 219, 220, 
		5, 65, 0, 0, 220, 221, 3, 20, 10, 0, 221, 225, 5, 76, 0, 0, 222, 224, 
		3, 10, 5, 0, 223, 222, 1, 0, 0, 0, 224, 227, 1, 0, 0, 0, 225, 223, 1, 
		0, 0, 0, 225, 226, 1, 0, 0, 0, 226, 33, 1, 0, 0, 0, 227, 225, 1, 0, 0, 
		0, 228, 229, 5, 15, 0, 0, 229, 230, 5, 66, 0, 0, 230, 234, 5, 76, 0, 0, 
		231, 233, 3, 10, 5, 0, 232, 231, 1, 0, 0, 0, 233, 236, 1, 0, 0, 0, 234, 
		232, 1, 0, 0, 0, 234, 235, 1, 0, 0, 0, 235, 35, 1, 0, 0, 0, 236, 234, 
		1, 0, 0, 0, 237, 238, 5, 15, 0, 0, 238, 239, 5, 67, 0, 0, 239, 240, 3, 
		24, 12, 0, 240, 241, 7, 5, 0, 0, 241, 242, 3, 20, 10, 0, 242, 243, 5, 
		76, 0, 0, 243, 37, 1, 0, 0, 0, 244, 245, 5, 15, 0, 0, 245, 246, 5, 69, 
		0, 0, 246, 247, 3, 26, 13, 0, 247, 248, 5, 76, 0, 0, 248, 39, 1, 0, 0, 
		0, 249, 250, 5, 15, 0, 0, 250, 251, 3, 42, 21, 0, 251, 255, 5, 78, 0, 
		0, 252, 254, 3, 16, 8, 0, 253, 252, 1, 0, 0, 0, 254, 257, 1, 0, 0, 0, 
		255, 253, 1, 0, 0, 0, 255, 256, 1, 0, 0, 0, 256, 41, 1, 0, 0, 0, 257, 
		255, 1, 0, 0, 0, 258, 264, 5, 80, 0, 0, 259, 260, 5, 79, 0, 0, 260, 261, 
		3, 20, 10, 0, 261, 262, 5, 58, 0, 0, 262, 264, 1, 0, 0, 0, 263, 258, 1, 
		0, 0, 0, 263, 259, 1, 0, 0, 0, 264, 267, 1, 0, 0, 0, 265, 263, 1, 0, 0, 
		0, 265, 266, 1, 0, 0, 0, 266, 43, 1, 0, 0, 0, 267, 265, 1, 0, 0, 0, 268, 
		270, 3, 46, 23, 0, 269, 268, 1, 0, 0, 0, 270, 273, 1, 0, 0, 0, 271, 269, 
		1, 0, 0, 0, 271, 272, 1, 0, 0, 0, 272, 274, 1, 0, 0, 0, 273, 271, 1, 0, 
		0, 0, 274, 276, 3, 46, 23, 0, 275, 277, 5, 3, 0, 0, 276, 275, 1, 0, 0, 
		0, 276, 277, 1, 0, 0, 0, 277, 45, 1, 0, 0, 0, 278, 279, 5, 14, 0, 0, 279, 
		288, 3, 12, 6, 0, 280, 284, 5, 1, 0, 0, 281, 283, 3, 10, 5, 0, 282, 281, 
		1, 0, 0, 0, 283, 286, 1, 0, 0, 0, 284, 282, 1, 0, 0, 0, 284, 285, 1, 0, 
		0, 0, 285, 287, 1, 0, 0, 0, 286, 284, 1, 0, 0, 0, 287, 289, 5, 2, 0, 0, 
		288, 280, 1, 0, 0, 0, 288, 289, 1, 0, 0, 0, 289, 47, 1, 0, 0, 0, 290, 
		291, 5, 15, 0, 0, 291, 292, 5, 70, 0, 0, 292, 293, 3, 24, 12, 0, 293, 
		294, 5, 31, 0, 0, 294, 297, 3, 22, 11, 0, 295, 296, 5, 55, 0, 0, 296, 
		298, 5, 57, 0, 0, 297, 295, 1, 0, 0, 0, 297, 298, 1, 0, 0, 0, 298, 299, 
		1, 0, 0, 0, 299, 300, 5, 76, 0, 0, 300, 49, 1, 0, 0, 0, 301, 302, 5, 15, 
		0, 0, 302, 303, 5, 71, 0, 0, 303, 304, 5, 7, 0, 0, 304, 313, 5, 76, 0, 
		0, 305, 306, 5, 15, 0, 0, 306, 307, 5, 71, 0, 0, 307, 308, 5, 16, 0, 0, 
		308, 309, 3, 20, 10, 0, 309, 310, 5, 58, 0, 0, 310, 311, 5, 76, 0, 0, 
		311, 313, 1, 0, 0, 0, 312, 301, 1, 0, 0, 0, 312, 305, 1, 0, 0, 0, 313, 
		51, 1, 0, 0, 0, 32, 55, 61, 69, 78, 83, 98, 102, 106, 111, 119, 125, 127, 
		147, 164, 166, 176, 183, 189, 198, 202, 215, 225, 234, 255, 263, 265, 
		271, 276, 284, 288, 297, 312
	];
}
