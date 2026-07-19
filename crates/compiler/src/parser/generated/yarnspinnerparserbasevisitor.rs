
// Generated from third-party/YarnSpinner/YarnSpinner.Compiler/Grammars/YarnSpinnerParser.g4 by ANTLR 4.13.2

use antlr4rust::tree::ParseTreeVisitor;
use super::yarnspinnerparser::*;

// A complete Visitor for a parse tree produced by YarnSpinnerParser.

pub trait YarnSpinnerParserBaseVisitor<'input>:
    ParseTreeVisitor<'input, YarnSpinnerParserContextType> {
	// Visit a parse tree produced by YarnSpinnerParser#dialogue.
	fn visit_dialogue(&mut self, ctx: &DialogueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#file_hashtag.
	fn visit_file_hashtag(&mut self, ctx: &File_hashtagContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#node.
	fn visit_node(&mut self, ctx: &NodeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#title_header.
	fn visit_title_header(&mut self, ctx: &Title_headerContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#when_header.
	fn visit_when_header(&mut self, ctx: &When_headerContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#header.
	fn visit_header(&mut self, ctx: &HeaderContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#header_when_expression.
	fn visit_header_when_expression(&mut self, ctx: &Header_when_expressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#body.
	fn visit_body(&mut self, ctx: &BodyContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#statement.
	fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#line_statement.
	fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#line_formatted_text.
	fn visit_line_formatted_text(&mut self, ctx: &Line_formatted_textContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#hashtag.
	fn visit_hashtag(&mut self, ctx: &HashtagContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#lineCondition.
	fn visit_linecondition(&mut self, ctx: &LineConditionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#lineOnceCondition.
	fn visit_lineoncecondition(&mut self, ctx: &LineOnceConditionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expParens.
	fn visit_expparens(&mut self, ctx: &ExpParensContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expMultDivMod.
	fn visit_expmultdivmod(&mut self, ctx: &ExpMultDivModContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expComparison.
	fn visit_expcomparison(&mut self, ctx: &ExpComparisonContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expNegative.
	fn visit_expnegative(&mut self, ctx: &ExpNegativeContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expAndOrXor.
	fn visit_expandorxor(&mut self, ctx: &ExpAndOrXorContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expAddSub.
	fn visit_expaddsub(&mut self, ctx: &ExpAddSubContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expNot.
	fn visit_expnot(&mut self, ctx: &ExpNotContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expValue.
	fn visit_expvalue(&mut self, ctx: &ExpValueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#expEquality.
	fn visit_expequality(&mut self, ctx: &ExpEqualityContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueNumber.
	fn visit_valuenumber(&mut self, ctx: &ValueNumberContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueTrue.
	fn visit_valuetrue(&mut self, ctx: &ValueTrueContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueFalse.
	fn visit_valuefalse(&mut self, ctx: &ValueFalseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueVar.
	fn visit_valuevar(&mut self, ctx: &ValueVarContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueString.
	fn visit_valuestring(&mut self, ctx: &ValueStringContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueFunc.
	fn visit_valuefunc(&mut self, ctx: &ValueFuncContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#valueTypeMemberReference.
	fn visit_valuetypememberreference(&mut self, ctx: &ValueTypeMemberReferenceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#variable.
	fn visit_variable(&mut self, ctx: &VariableContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#function_call.
	fn visit_function_call(&mut self, ctx: &Function_callContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#typeMemberReference.
	fn visit_typememberreference(&mut self, ctx: &TypeMemberReferenceContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#if_statement.
	fn visit_if_statement(&mut self, ctx: &If_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#if_clause.
	fn visit_if_clause(&mut self, ctx: &If_clauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#else_if_clause.
	fn visit_else_if_clause(&mut self, ctx: &Else_if_clauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#else_clause.
	fn visit_else_clause(&mut self, ctx: &Else_clauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#set_statement.
	fn visit_set_statement(&mut self, ctx: &Set_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#call_statement.
	fn visit_call_statement(&mut self, ctx: &Call_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#command_statement.
	fn visit_command_statement(&mut self, ctx: &Command_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#command_formatted_text.
	fn visit_command_formatted_text(&mut self, ctx: &Command_formatted_textContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#shortcut_option_statement.
	fn visit_shortcut_option_statement(&mut self, ctx: &Shortcut_option_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#shortcut_option.
	fn visit_shortcut_option(&mut self, ctx: &Shortcut_optionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#line_group_statement.
	fn visit_line_group_statement(&mut self, ctx: &Line_group_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#line_group_item.
	fn visit_line_group_item(&mut self, ctx: &Line_group_itemContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#declare_statement.
	fn visit_declare_statement(&mut self, ctx: &Declare_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#enum_statement.
	fn visit_enum_statement(&mut self, ctx: &Enum_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#enum_case_statement.
	fn visit_enum_case_statement(&mut self, ctx: &Enum_case_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#jumpToNodeName.
	fn visit_jumptonodename(&mut self, ctx: &JumpToNodeNameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#jumpToExpression.
	fn visit_jumptoexpression(&mut self, ctx: &JumpToExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#detourToNodeName.
	fn visit_detourtonodename(&mut self, ctx: &DetourToNodeNameContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#detourToExpression.
	fn visit_detourtoexpression(&mut self, ctx: &DetourToExpressionContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#return_statement.
	fn visit_return_statement(&mut self, ctx: &Return_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#once_statement.
	fn visit_once_statement(&mut self, ctx: &Once_statementContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#once_primary_clause.
	fn visit_once_primary_clause(&mut self, ctx: &Once_primary_clauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#once_alternate_clause.
	fn visit_once_alternate_clause(&mut self, ctx: &Once_alternate_clauseContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#structured_command.
	fn visit_structured_command(&mut self, ctx: &Structured_commandContext<'input>) {
            self.visit_children(ctx)
        }

	// Visit a parse tree produced by YarnSpinnerParser#structured_command_value.
	fn visit_structured_command_value(&mut self, ctx: &Structured_command_valueContext<'input>) {
            self.visit_children(ctx)
        }

}