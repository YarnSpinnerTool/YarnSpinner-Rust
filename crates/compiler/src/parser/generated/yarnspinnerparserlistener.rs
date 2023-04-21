#![allow(nonstandard_style)]
// Generated from .\YarnSpinnerParser.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::yarnspinnerparser::*;

pub trait YarnSpinnerParserListener<'input> : ParseTreeListener<'input,YarnSpinnerParserContextType>{
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#dialogue}.
 * @param ctx the parse tree
 */
fn enter_dialogue(&mut self, _ctx: &DialogueContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#dialogue}.
 * @param ctx the parse tree
 */
fn exit_dialogue(&mut self, _ctx: &DialogueContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#file_hashtag}.
 * @param ctx the parse tree
 */
fn enter_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#file_hashtag}.
 * @param ctx the parse tree
 */
fn exit_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#node}.
 * @param ctx the parse tree
 */
fn enter_node(&mut self, _ctx: &NodeContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#node}.
 * @param ctx the parse tree
 */
fn exit_node(&mut self, _ctx: &NodeContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#header}.
 * @param ctx the parse tree
 */
fn enter_header(&mut self, _ctx: &HeaderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#header}.
 * @param ctx the parse tree
 */
fn exit_header(&mut self, _ctx: &HeaderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#body}.
 * @param ctx the parse tree
 */
fn enter_body(&mut self, _ctx: &BodyContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#body}.
 * @param ctx the parse tree
 */
fn exit_body(&mut self, _ctx: &BodyContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#statement}.
 * @param ctx the parse tree
 */
fn enter_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#statement}.
 * @param ctx the parse tree
 */
fn exit_statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#line_statement}.
 * @param ctx the parse tree
 */
fn enter_line_statement(&mut self, _ctx: &Line_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#line_statement}.
 * @param ctx the parse tree
 */
fn exit_line_statement(&mut self, _ctx: &Line_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#line_formatted_text}.
 * @param ctx the parse tree
 */
fn enter_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#line_formatted_text}.
 * @param ctx the parse tree
 */
fn exit_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#hashtag}.
 * @param ctx the parse tree
 */
fn enter_hashtag(&mut self, _ctx: &HashtagContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#hashtag}.
 * @param ctx the parse tree
 */
fn exit_hashtag(&mut self, _ctx: &HashtagContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#line_condition}.
 * @param ctx the parse tree
 */
fn enter_line_condition(&mut self, _ctx: &Line_conditionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#line_condition}.
 * @param ctx the parse tree
 */
fn exit_line_condition(&mut self, _ctx: &Line_conditionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expParens}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expParens(&mut self, _ctx: &ExpParensContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expParens}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expParens(&mut self, _ctx: &ExpParensContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expMultDivMod}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expMultDivMod(&mut self, _ctx: &ExpMultDivModContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expMultDivMod}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expMultDivMod(&mut self, _ctx: &ExpMultDivModContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expComparison}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expComparison(&mut self, _ctx: &ExpComparisonContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expComparison}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expComparison(&mut self, _ctx: &ExpComparisonContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expNegative}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expNegative(&mut self, _ctx: &ExpNegativeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expNegative}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expNegative(&mut self, _ctx: &ExpNegativeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expAndOrXor}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expAndOrXor(&mut self, _ctx: &ExpAndOrXorContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expAndOrXor}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expAndOrXor(&mut self, _ctx: &ExpAndOrXorContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expAddSub}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expAddSub(&mut self, _ctx: &ExpAddSubContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expAddSub}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expAddSub(&mut self, _ctx: &ExpAddSubContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expNot}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expNot(&mut self, _ctx: &ExpNotContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expNot}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expNot(&mut self, _ctx: &ExpNotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expValue}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expValue(&mut self, _ctx: &ExpValueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expValue}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expValue(&mut self, _ctx: &ExpValueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code expEquality}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn enter_expEquality(&mut self, _ctx: &ExpEqualityContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code expEquality}
 * labeled alternative in {@link YarnSpinnerParser#expression}.
 * @param ctx the parse tree
 */
fn exit_expEquality(&mut self, _ctx: &ExpEqualityContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueNumber}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueNumber}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueTrue}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueTrue}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueFalse}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueFalse}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueVar}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueVar(&mut self, _ctx: &ValueVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueVar}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueVar(&mut self, _ctx: &ValueVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueString}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueString(&mut self, _ctx: &ValueStringContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueString}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueString(&mut self, _ctx: &ValueStringContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueNull}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueNull(&mut self, _ctx: &ValueNullContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueNull}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueNull(&mut self, _ctx: &ValueNullContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code valueFunc}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn enter_valueFunc(&mut self, _ctx: &ValueFuncContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code valueFunc}
 * labeled alternative in {@link YarnSpinnerParser#value}.
 * @param ctx the parse tree
 */
fn exit_valueFunc(&mut self, _ctx: &ValueFuncContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#variable}.
 * @param ctx the parse tree
 */
fn enter_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#variable}.
 * @param ctx the parse tree
 */
fn exit_variable(&mut self, _ctx: &VariableContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#function_call}.
 * @param ctx the parse tree
 */
fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#function_call}.
 * @param ctx the parse tree
 */
fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#if_statement}.
 * @param ctx the parse tree
 */
fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#if_statement}.
 * @param ctx the parse tree
 */
fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#if_clause}.
 * @param ctx the parse tree
 */
fn enter_if_clause(&mut self, _ctx: &If_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#if_clause}.
 * @param ctx the parse tree
 */
fn exit_if_clause(&mut self, _ctx: &If_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#else_if_clause}.
 * @param ctx the parse tree
 */
fn enter_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#else_if_clause}.
 * @param ctx the parse tree
 */
fn exit_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#else_clause}.
 * @param ctx the parse tree
 */
fn enter_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#else_clause}.
 * @param ctx the parse tree
 */
fn exit_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#set_statement}.
 * @param ctx the parse tree
 */
fn enter_set_statement(&mut self, _ctx: &Set_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#set_statement}.
 * @param ctx the parse tree
 */
fn exit_set_statement(&mut self, _ctx: &Set_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#call_statement}.
 * @param ctx the parse tree
 */
fn enter_call_statement(&mut self, _ctx: &Call_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#call_statement}.
 * @param ctx the parse tree
 */
fn exit_call_statement(&mut self, _ctx: &Call_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#command_statement}.
 * @param ctx the parse tree
 */
fn enter_command_statement(&mut self, _ctx: &Command_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#command_statement}.
 * @param ctx the parse tree
 */
fn exit_command_statement(&mut self, _ctx: &Command_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#command_formatted_text}.
 * @param ctx the parse tree
 */
fn enter_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#command_formatted_text}.
 * @param ctx the parse tree
 */
fn exit_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#shortcut_option_statement}.
 * @param ctx the parse tree
 */
fn enter_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#shortcut_option_statement}.
 * @param ctx the parse tree
 */
fn exit_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#shortcut_option}.
 * @param ctx the parse tree
 */
fn enter_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#shortcut_option}.
 * @param ctx the parse tree
 */
fn exit_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link YarnSpinnerParser#declare_statement}.
 * @param ctx the parse tree
 */
fn enter_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) { }
/**
 * Exit a parse tree produced by {@link YarnSpinnerParser#declare_statement}.
 * @param ctx the parse tree
 */
fn exit_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code jumpToNodeName}
 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
 * @param ctx the parse tree
 */
fn enter_jumpToNodeName(&mut self, _ctx: &JumpToNodeNameContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code jumpToNodeName}
 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
 * @param ctx the parse tree
 */
fn exit_jumpToNodeName(&mut self, _ctx: &JumpToNodeNameContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code jumpToExpression}
 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
 * @param ctx the parse tree
 */
fn enter_jumpToExpression(&mut self, _ctx: &JumpToExpressionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code jumpToExpression}
 * labeled alternative in {@link YarnSpinnerParser#jump_statement}.
 * @param ctx the parse tree
 */
fn exit_jumpToExpression(&mut self, _ctx: &JumpToExpressionContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : YarnSpinnerParserListener<'input> }


