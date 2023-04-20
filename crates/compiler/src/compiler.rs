pub use crate::compiler::compilation_job::*;
use crate::{output::*, prelude::generated::yarnspinnerparserlistener};

mod compilation_job;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(_compilation_job: CompilationJob) -> CompilationResult {
    // TODO: Implement this :)
    CompilationResult {
        program: None,
        string_table: Default::default(),
        declarations: None,
        contains_implicit_string_tags: false,
        file_tags: Default::default(),
        diagnostics: vec![],
        debug_info: Default::default(),
    }
}

struct ImplicitStringTagGenerator {
    generated_implicit_string_tag: bool,
}

impl yarnspinnerparserlistener::YarnSpinnerParserListener for ImplicitStringTagGenerator {
    fn generate_string_tag_if_absent(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::DialogueContext<'input>,
    ) {
        if (!&self.has_string_tag(_ctx)) {
            &self.generate_string_tag(_ctx);
            self.generated_implicit_string_tag = true;
        }

        todo!()
    }

    fn has_string_tag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::DialogueContext<'input>,
    ) -> bool {
        todo!()
    }

    fn generate_string_tag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::DialogueContext<'input>,
    ) {
        todo!()
    }

    fn enter_dialogue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::DialogueContext<'input>,
    ) {
    }

    fn exit_dialogue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::DialogueContext<'input>,
    ) {
    }

    fn enter_file_hashtag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::File_hashtagContext<'input>,
    ) {
    }

    fn exit_file_hashtag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::File_hashtagContext<'input>,
    ) {
    }

    fn enter_node(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::NodeContext<'input>,
    ) {
    }

    fn exit_node(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::NodeContext<'input>,
    ) {
    }

    fn enter_header(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::HeaderContext<'input>,
    ) {
    }

    fn exit_header(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::HeaderContext<'input>,
    ) {
    }

    fn enter_body(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::BodyContext<'input>,
    ) {
    }

    fn exit_body(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::BodyContext<'input>,
    ) {
    }

    fn enter_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::StatementContext<'input>,
    ) {
    }

    fn exit_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::StatementContext<'input>,
    ) {
    }

    fn enter_line_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_statementContext<'input>,
    ) {
    }

    fn exit_line_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_statementContext<'input>,
    ) {
    }

    fn enter_line_formatted_text(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_formatted_textContext<'input>,
    ) {
    }

    fn exit_line_formatted_text(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_formatted_textContext<'input>,
    ) {
    }

    fn enter_hashtag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::HashtagContext<'input>,
    ) {
    }

    fn exit_hashtag(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::HashtagContext<'input>,
    ) {
    }

    fn enter_line_condition(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_conditionContext<'input>,
    ) {
    }

    fn exit_line_condition(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Line_conditionContext<'input>,
    ) {
    }

    fn enter_expParens(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpParensContext<'input>,
    ) {
    }

    fn exit_expParens(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpParensContext<'input>,
    ) {
    }

    fn enter_expMultDivMod(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpMultDivModContext<'input>,
    ) {
    }

    fn exit_expMultDivMod(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpMultDivModContext<'input>,
    ) {
    }

    fn enter_expComparison(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpComparisonContext<'input>,
    ) {
    }

    fn exit_expComparison(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpComparisonContext<'input>,
    ) {
    }

    fn enter_expNegative(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpNegativeContext<'input>,
    ) {
    }

    fn exit_expNegative(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpNegativeContext<'input>,
    ) {
    }

    fn enter_expAndOrXor(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpAndOrXorContext<'input>,
    ) {
    }

    fn exit_expAndOrXor(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpAndOrXorContext<'input>,
    ) {
    }

    fn enter_expAddSub(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpAddSubContext<'input>,
    ) {
    }

    fn exit_expAddSub(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpAddSubContext<'input>,
    ) {
    }

    fn enter_expNot(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpNotContext<'input>,
    ) {
    }

    fn exit_expNot(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpNotContext<'input>,
    ) {
    }

    fn enter_expValue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpValueContext<'input>,
    ) {
    }

    fn exit_expValue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpValueContext<'input>,
    ) {
    }

    fn enter_expEquality(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpEqualityContext<'input>,
    ) {
    }

    fn exit_expEquality(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ExpEqualityContext<'input>,
    ) {
    }

    fn enter_valueNumber(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueNumberContext<'input>,
    ) {
    }

    fn exit_valueNumber(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueNumberContext<'input>,
    ) {
    }

    fn enter_valueTrue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueTrueContext<'input>,
    ) {
    }

    fn exit_valueTrue(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueTrueContext<'input>,
    ) {
    }

    fn enter_valueFalse(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueFalseContext<'input>,
    ) {
    }

    fn exit_valueFalse(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueFalseContext<'input>,
    ) {
    }

    fn enter_valueVar(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueVarContext<'input>,
    ) {
    }

    fn exit_valueVar(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueVarContext<'input>,
    ) {
    }

    fn enter_valueString(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueStringContext<'input>,
    ) {
    }

    fn exit_valueString(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueStringContext<'input>,
    ) {
    }

    fn enter_valueNull(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueNullContext<'input>,
    ) {
    }

    fn exit_valueNull(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueNullContext<'input>,
    ) {
    }

    fn enter_valueFunc(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueFuncContext<'input>,
    ) {
    }

    fn exit_valueFunc(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::ValueFuncContext<'input>,
    ) {
    }

    fn enter_variable(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::VariableContext<'input>,
    ) {
    }

    fn exit_variable(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::VariableContext<'input>,
    ) {
    }

    fn enter_function_call(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Function_callContext<'input>,
    ) {
    }

    fn exit_function_call(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Function_callContext<'input>,
    ) {
    }

    fn enter_if_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::If_statementContext<'input>,
    ) {
    }

    fn exit_if_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::If_statementContext<'input>,
    ) {
    }

    fn enter_if_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::If_clauseContext<'input>,
    ) {
    }

    fn exit_if_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::If_clauseContext<'input>,
    ) {
    }

    fn enter_else_if_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Else_if_clauseContext<'input>,
    ) {
    }

    fn exit_else_if_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Else_if_clauseContext<'input>,
    ) {
    }

    fn enter_else_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Else_clauseContext<'input>,
    ) {
    }

    fn exit_else_clause(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Else_clauseContext<'input>,
    ) {
    }

    fn enter_set_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Set_statementContext<'input>,
    ) {
    }

    fn exit_set_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Set_statementContext<'input>,
    ) {
    }

    fn enter_call_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Call_statementContext<'input>,
    ) {
    }

    fn exit_call_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Call_statementContext<'input>,
    ) {
    }

    fn enter_command_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Command_statementContext<'input>,
    ) {
    }

    fn exit_command_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Command_statementContext<'input>,
    ) {
    }

    fn enter_command_formatted_text(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Command_formatted_textContext<'input>,
    ) {
    }

    fn exit_command_formatted_text(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Command_formatted_textContext<'input>,
    ) {
    }

    fn enter_shortcut_option_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Shortcut_option_statementContext<
            'input,
        >,
    ) {
    }

    fn exit_shortcut_option_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Shortcut_option_statementContext<
            'input,
        >,
    ) {
    }

    fn enter_shortcut_option(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Shortcut_optionContext<'input>,
    ) {
    }

    fn exit_shortcut_option(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Shortcut_optionContext<'input>,
    ) {
    }

    fn enter_declare_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Declare_statementContext<'input>,
    ) {
    }

    fn exit_declare_statement(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::Declare_statementContext<'input>,
    ) {
    }

    fn enter_jumpToNodeName(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::JumpToNodeNameContext<'input>,
    ) {
    }

    fn exit_jumpToNodeName(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::JumpToNodeNameContext<'input>,
    ) {
    }

    fn enter_jumpToExpression(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::JumpToExpressionContext<'input>,
    ) {
    }

    fn exit_jumpToExpression(
        &mut self,
        _ctx: &crate::prelude::generated::yarnspinnerparser::JumpToExpressionContext<'input>,
    ) {
    }
}
