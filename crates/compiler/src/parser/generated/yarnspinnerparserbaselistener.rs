// Generated from third-party/YarnSpinner/YarnSpinner.Compiler/Grammars/YarnSpinnerParser.g4 by ANTLR 4.13.2

use super::yarnspinnerparser::*;
use antlr4rust::tree::ParseTreeListener;

// A complete Visitor for a parse tree produced by YarnSpinnerParser.

pub trait YarnSpinnerParserBaseListener<'input>:
    ParseTreeListener<'input, YarnSpinnerParserContextType> {

    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_dialogue(&mut self, _ctx: &DialogueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_dialogue(&mut self, _ctx: &DialogueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_node(&mut self, _ctx: &NodeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_node(&mut self, _ctx: &NodeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_title_header(&mut self, _ctx: &Title_headerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_title_header(&mut self, _ctx: &Title_headerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_when_header(&mut self, _ctx: &When_headerContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_when_header(&mut self, _ctx: &When_headerContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_header(&mut self, _ctx: &HeaderContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_header(&mut self, _ctx: &HeaderContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_header_when_expression(&mut self, _ctx: &Header_when_expressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_header_when_expression(&mut self, _ctx: &Header_when_expressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_body(&mut self, _ctx: &BodyContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_body(&mut self, _ctx: &BodyContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_line_statement(&mut self, _ctx: &Line_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_line_statement(&mut self, _ctx: &Line_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_hashtag(&mut self, _ctx: &HashtagContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_hashtag(&mut self, _ctx: &HashtagContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_linecondition(&mut self, _ctx: &LineConditionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_linecondition(&mut self, _ctx: &LineConditionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_lineoncecondition(&mut self, _ctx: &LineOnceConditionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_lineoncecondition(&mut self, _ctx: &LineOnceConditionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expparens(&mut self, _ctx: &ExpParensContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expparens(&mut self, _ctx: &ExpParensContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expmultdivmod(&mut self, _ctx: &ExpMultDivModContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expmultdivmod(&mut self, _ctx: &ExpMultDivModContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expcomparison(&mut self, _ctx: &ExpComparisonContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expcomparison(&mut self, _ctx: &ExpComparisonContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expnegative(&mut self, _ctx: &ExpNegativeContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expnegative(&mut self, _ctx: &ExpNegativeContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expandorxor(&mut self, _ctx: &ExpAndOrXorContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expandorxor(&mut self, _ctx: &ExpAndOrXorContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expaddsub(&mut self, _ctx: &ExpAddSubContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expaddsub(&mut self, _ctx: &ExpAddSubContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expnot(&mut self, _ctx: &ExpNotContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expnot(&mut self, _ctx: &ExpNotContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expvalue(&mut self, _ctx: &ExpValueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expvalue(&mut self, _ctx: &ExpValueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_expequality(&mut self, _ctx: &ExpEqualityContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_expequality(&mut self, _ctx: &ExpEqualityContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuenumber(&mut self, _ctx: &ValueNumberContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuenumber(&mut self, _ctx: &ValueNumberContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuetrue(&mut self, _ctx: &ValueTrueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuetrue(&mut self, _ctx: &ValueTrueContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuefalse(&mut self, _ctx: &ValueFalseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuefalse(&mut self, _ctx: &ValueFalseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuevar(&mut self, _ctx: &ValueVarContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuevar(&mut self, _ctx: &ValueVarContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuestring(&mut self, _ctx: &ValueStringContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuestring(&mut self, _ctx: &ValueStringContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuefunc(&mut self, _ctx: &ValueFuncContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuefunc(&mut self, _ctx: &ValueFuncContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_valuetypememberreference(&mut self, _ctx: &ValueTypeMemberReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_valuetypememberreference(&mut self, _ctx: &ValueTypeMemberReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_typememberreference(&mut self, _ctx: &TypeMemberReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_typememberreference(&mut self, _ctx: &TypeMemberReferenceContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_if_clause(&mut self, _ctx: &If_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_if_clause(&mut self, _ctx: &If_clauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_set_statement(&mut self, _ctx: &Set_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_set_statement(&mut self, _ctx: &Set_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_call_statement(&mut self, _ctx: &Call_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_call_statement(&mut self, _ctx: &Call_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_command_statement(&mut self, _ctx: &Command_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_command_statement(&mut self, _ctx: &Command_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_line_group_statement(&mut self, _ctx: &Line_group_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_line_group_statement(&mut self, _ctx: &Line_group_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_line_group_item(&mut self, _ctx: &Line_group_itemContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_line_group_item(&mut self, _ctx: &Line_group_itemContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enum_statement(&mut self, _ctx: &Enum_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enum_statement(&mut self, _ctx: &Enum_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_enum_case_statement(&mut self, _ctx: &Enum_case_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_enum_case_statement(&mut self, _ctx: &Enum_case_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_jumptonodename(&mut self, _ctx: &JumpToNodeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_jumptonodename(&mut self, _ctx: &JumpToNodeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_jumptoexpression(&mut self, _ctx: &JumpToExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_jumptoexpression(&mut self, _ctx: &JumpToExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_detourtonodename(&mut self, _ctx: &DetourToNodeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_detourtonodename(&mut self, _ctx: &DetourToNodeNameContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_detourtoexpression(&mut self, _ctx: &DetourToExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_detourtoexpression(&mut self, _ctx: &DetourToExpressionContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_return_statement(&mut self, _ctx: &Return_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_return_statement(&mut self, _ctx: &Return_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_once_statement(&mut self, _ctx: &Once_statementContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_once_statement(&mut self, _ctx: &Once_statementContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_once_primary_clause(&mut self, _ctx: &Once_primary_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_once_primary_clause(&mut self, _ctx: &Once_primary_clauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_once_alternate_clause(&mut self, _ctx: &Once_alternate_clauseContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_once_alternate_clause(&mut self, _ctx: &Once_alternate_clauseContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structured_command(&mut self, _ctx: &Structured_commandContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structured_command(&mut self, _ctx: &Structured_commandContext<'input>) {}


    /**
     * Enter a parse tree produced by \{@link YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn enter_structured_command_value(&mut self, _ctx: &Structured_command_valueContext<'input>) {}
    /**
     * Exit a parse tree produced by \{@link  YarnSpinnerParserBaseParser#s}.
     * @param ctx the parse tree
     */
    fn exit_structured_command_value(&mut self, _ctx: &Structured_command_valueContext<'input>) {}


}