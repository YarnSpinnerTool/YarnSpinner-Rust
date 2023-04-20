#![allow(nonstandard_style)]
// Generated from .\YarnSpinnerParser.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::yarnspinnerparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link YarnSpinnerParser}.
 */
pub trait YarnSpinnerParserVisitor<'input>: ParseTreeVisitor<'input,YarnSpinnerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#dialogue}.
	 * @param ctx the parse tree
	 */
	fn visit_dialogue(&mut self, ctx: &DialogueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#file_hashtag}.
	 * @param ctx the parse tree
	 */
	fn visit_file_hashtag(&mut self, ctx: &File_hashtagContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#node}.
	 * @param ctx the parse tree
	 */
	fn visit_node(&mut self, ctx: &NodeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#header}.
	 * @param ctx the parse tree
	 */
	fn visit_header(&mut self, ctx: &HeaderContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#body}.
	 * @param ctx the parse tree
	 */
	fn visit_body(&mut self, ctx: &BodyContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#statement}.
	 * @param ctx the parse tree
	 */
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_formatted_text}.
	 * @param ctx the parse tree
	 */
	fn visit_line_formatted_text(&mut self, ctx: &Line_formatted_textContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#hashtag}.
	 * @param ctx the parse tree
	 */
	fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_condition}.
	 * @param ctx the parse tree
	 */
	fn visit_line_condition(&mut self, ctx: &Line_conditionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expParens}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expMultDivMod}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expMultDivMod(&mut self, ctx: &ExpMultDivModContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expComparison}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expComparison(&mut self, ctx: &ExpComparisonContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expNegative}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expNegative(&mut self, ctx: &ExpNegativeContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expAndOrXor}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expAddSub}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expAddSub(&mut self, ctx: &ExpAddSubContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expNot}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expNot(&mut self, ctx: &ExpNotContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expValue}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code expEquality}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
	fn visit_expEquality(&mut self, ctx: &ExpEqualityContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueNumber}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueTrue}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueTrue(&mut self, ctx: &ValueTrueContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueFalse}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueFalse(&mut self, ctx: &ValueFalseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueVar}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueString}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueNull}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code valueFunc}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
	fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#variable}.
	 * @param ctx the parse tree
	 */
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#function_call}.
	 * @param ctx the parse tree
	 */
	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#if_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#if_clause}.
	 * @param ctx the parse tree
	 */
	fn visit_if_clause(&mut self, ctx: &If_clauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#else_if_clause}.
	 * @param ctx the parse tree
	 */
	fn visit_else_if_clause(&mut self, ctx: &Else_if_clauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#else_clause}.
	 * @param ctx the parse tree
	 */
	fn visit_else_clause(&mut self, ctx: &Else_clauseContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#set_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#call_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_call_statement(&mut self, ctx: &Call_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#command_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_command_statement(&mut self, ctx: &Command_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#command_formatted_text}.
	 * @param ctx the parse tree
	 */
	fn visit_command_formatted_text(&mut self, ctx: &Command_formatted_textContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#shortcut_option_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_shortcut_option_statement(&mut self, ctx: &Shortcut_option_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#shortcut_option}.
	 * @param ctx the parse tree
	 */
	fn visit_shortcut_option(&mut self, ctx: &Shortcut_optionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#declare_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code jumpToNodeName}
	 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_jumpToNodeName(&mut self, ctx: &JumpToNodeNameContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code jumpToExpression}
	 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
	 * @param ctx the parse tree
	 */
	fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>) { self.visit_children(ctx) }

}

pub trait YarnSpinnerParserVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= YarnSpinnerParserContextType>{
	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#dialogue}.
	 * @param ctx the parse tree
	 */
		fn visit_dialogue(&mut self, ctx: &DialogueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#file_hashtag}.
	 * @param ctx the parse tree
	 */
		fn visit_file_hashtag(&mut self, ctx: &File_hashtagContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#node}.
	 * @param ctx the parse tree
	 */
		fn visit_node(&mut self, ctx: &NodeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#header}.
	 * @param ctx the parse tree
	 */
		fn visit_header(&mut self, ctx: &HeaderContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#body}.
	 * @param ctx the parse tree
	 */
		fn visit_body(&mut self, ctx: &BodyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#statement}.
	 * @param ctx the parse tree
	 */
		fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_formatted_text}.
	 * @param ctx the parse tree
	 */
		fn visit_line_formatted_text(&mut self, ctx: &Line_formatted_textContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#hashtag}.
	 * @param ctx the parse tree
	 */
		fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#line_condition}.
	 * @param ctx the parse tree
	 */
		fn visit_line_condition(&mut self, ctx: &Line_conditionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expParens}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expMultDivMod}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expMultDivMod(&mut self, ctx: &ExpMultDivModContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expComparison}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expComparison(&mut self, ctx: &ExpComparisonContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expNegative}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expNegative(&mut self, ctx: &ExpNegativeContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expAndOrXor}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expAddSub}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expAddSub(&mut self, ctx: &ExpAddSubContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expNot}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expNot(&mut self, ctx: &ExpNotContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expValue}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code expEquality}
	 * labeled alternative in {@link YarnSpinnerParser#expression}.
	 * @param ctx the parse tree
	 */
		fn visit_expEquality(&mut self, ctx: &ExpEqualityContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueNumber}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueTrue}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueTrue(&mut self, ctx: &ValueTrueContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueFalse}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueFalse(&mut self, ctx: &ValueFalseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueVar}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueString}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueNull}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code valueFunc}
	 * labeled alternative in {@link YarnSpinnerParser#value}.
	 * @param ctx the parse tree
	 */
		fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#variable}.
	 * @param ctx the parse tree
	 */
		fn visit_variable(&mut self, ctx: &VariableContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#function_call}.
	 * @param ctx the parse tree
	 */
		fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#if_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#if_clause}.
	 * @param ctx the parse tree
	 */
		fn visit_if_clause(&mut self, ctx: &If_clauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#else_if_clause}.
	 * @param ctx the parse tree
	 */
		fn visit_else_if_clause(&mut self, ctx: &Else_if_clauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#else_clause}.
	 * @param ctx the parse tree
	 */
		fn visit_else_clause(&mut self, ctx: &Else_clauseContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#set_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#call_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_call_statement(&mut self, ctx: &Call_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#command_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_command_statement(&mut self, ctx: &Command_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#command_formatted_text}.
	 * @param ctx the parse tree
	 */
		fn visit_command_formatted_text(&mut self, ctx: &Command_formatted_textContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#shortcut_option_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_shortcut_option_statement(&mut self, ctx: &Shortcut_option_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#shortcut_option}.
	 * @param ctx the parse tree
	 */
		fn visit_shortcut_option(&mut self, ctx: &Shortcut_optionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by {@link YarnSpinnerParser#declare_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code jumpToNodeName}
	 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_jumpToNodeName(&mut self, ctx: &JumpToNodeNameContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code jumpToExpression}
	 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
	 * @param ctx the parse tree
	 */
		fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> YarnSpinnerParserVisitor<'input> for T
where
	T: YarnSpinnerParserVisitorCompat<'input>
{
	fn visit_dialogue(&mut self, ctx: &DialogueContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_dialogue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_file_hashtag(&mut self, ctx: &File_hashtagContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_file_hashtag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_node(&mut self, ctx: &NodeContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_node(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_header(&mut self, ctx: &HeaderContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_header(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_body(&mut self, ctx: &BodyContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_body(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_statement(&mut self, ctx: &StatementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_line_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_line_formatted_text(&mut self, ctx: &Line_formatted_textContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_line_formatted_text(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_hashtag(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_line_condition(&mut self, ctx: &Line_conditionContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_line_condition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expParens(&mut self, ctx: &ExpParensContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expParens(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expMultDivMod(&mut self, ctx: &ExpMultDivModContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expMultDivMod(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expComparison(&mut self, ctx: &ExpComparisonContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expComparison(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expNegative(&mut self, ctx: &ExpNegativeContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expNegative(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expAndOrXor(&mut self, ctx: &ExpAndOrXorContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expAndOrXor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expAddSub(&mut self, ctx: &ExpAddSubContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expAddSub(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expNot(&mut self, ctx: &ExpNotContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expNot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expValue(&mut self, ctx: &ExpValueContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_expEquality(&mut self, ctx: &ExpEqualityContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_expEquality(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueNumber(&mut self, ctx: &ValueNumberContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueNumber(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueTrue(&mut self, ctx: &ValueTrueContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueTrue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueFalse(&mut self, ctx: &ValueFalseContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueFalse(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueVar(&mut self, ctx: &ValueVarContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueVar(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueString(&mut self, ctx: &ValueStringContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueNull(&mut self, ctx: &ValueNullContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueNull(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_valueFunc(&mut self, ctx: &ValueFuncContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_valueFunc(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_variable(&mut self, ctx: &VariableContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_variable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_function_call(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_if_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_if_clause(&mut self, ctx: &If_clauseContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_if_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_else_if_clause(&mut self, ctx: &Else_if_clauseContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_else_if_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_else_clause(&mut self, ctx: &Else_clauseContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_else_clause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_set_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_call_statement(&mut self, ctx: &Call_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_call_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_command_statement(&mut self, ctx: &Command_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_command_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_command_formatted_text(&mut self, ctx: &Command_formatted_textContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_command_formatted_text(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shortcut_option_statement(&mut self, ctx: &Shortcut_option_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_shortcut_option_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_shortcut_option(&mut self, ctx: &Shortcut_optionContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_shortcut_option(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_declare_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jumpToNodeName(&mut self, ctx: &JumpToNodeNameContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_jumpToNodeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_jumpToExpression(&mut self, ctx: &JumpToExpressionContext<'input>){
		let result = <Self as YarnSpinnerParserVisitorCompat>::visit_jumpToExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}