use antlr_rust::tree::ParseTreeListener;

pub use crate::compiler::compilation_job::*;
use crate::prelude::generated::yarnspinnerparser::*;
use crate::{
    output::*,
    prelude::generated::{
        yarnspinnerparser::{DialogueContext, YarnSpinnerParserContextType},
        yarnspinnerparserlistener,
    },
};

mod compilation_job;

/// Compile Yarn code, as specified by a compilation job.
pub fn compile(_compilation_job: CompilationJob) -> CompilationResult {
    // TODO: other steps
    let compiler_steps: Vec<dyn CompilerStep> =
        vec![StringTableGenerator {}, BuiltInTypesProvider {}];

    let initial = CompilationResult {
        program: None,
        string_table: Default::default(),
        declarations: None,
        contains_implicit_string_tags: false,
        file_tags: Default::default(),
        diagnostics: vec![],
        debug_info: Default::default(),
    };

    compiler_steps
        .into_iter()
        .fold(initial, |acc, curr| curr.run(acc))
}

trait CompilerStep {
    // TODO: make this mut or not? That's a general style decision...
    fn run(input: &CompilationResult) -> CompilationResult;
}

struct BuiltInTypesProvider {}

impl CompilerStep for BuiltInTypesProvider {
    fn run(_input: &CompilationResult) -> CompilationResult {
        todo!()
    }
}

struct StringTableGenerator {}

impl CompilerStep for StringTableGenerator {
    fn run(_input: &CompilationResult) -> CompilationResult {
        todo!()
    }
}

impl<'input> ParseTreeListener<'input, YarnSpinnerParserContextType> for StringTableGenerator {}

impl<'input> StringTableGenerator {
    fn generate_string_tag_if_absent(&mut self, _ctx: &DialogueContext<'input>) {
        if (!&self.has_string_tag(_ctx)) {
            self.generate_string_tag(_ctx);
            // TODO: how to access CompilationResult? Or do we add field?
            // self.generated_implicit_string_tag = true;
        }

        todo!()
    }

    fn has_string_tag(&mut self, _ctx: &DialogueContext<'input>) -> bool {
        todo!()
    }

    fn generate_string_tag(&mut self, _ctx: &DialogueContext<'input>) {
        todo!()
    }
}

/// Adapted from https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.3.0/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs
impl<'input> yarnspinnerparserlistener::YarnSpinnerParserListener<'input> for StringTableGenerator {
    fn enter_dialogue(&mut self, _ctx: &DialogueContext<'input>) {}

    fn exit_dialogue(&mut self, _ctx: &DialogueContext<'input>) {}

    fn enter_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) {}

    fn exit_file_hashtag(&mut self, _ctx: &File_hashtagContext<'input>) {}

    fn enter_node(&mut self, _ctx: &NodeContext<'input>) {}

    fn exit_node(&mut self, _ctx: &NodeContext<'input>) {}

    fn enter_header(&mut self, _ctx: &HeaderContext<'input>) {}

    fn exit_header(&mut self, _ctx: &HeaderContext<'input>) {}

    fn enter_body(&mut self, _ctx: &BodyContext<'input>) {}

    fn exit_body(&mut self, _ctx: &BodyContext<'input>) {}

    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}

    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}

    fn enter_line_statement(&mut self, _ctx: &Line_statementContext<'input>) {}

    fn exit_line_statement(&mut self, _ctx: &Line_statementContext<'input>) {}

    fn enter_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) {}

    fn exit_line_formatted_text(&mut self, _ctx: &Line_formatted_textContext<'input>) {}

    fn enter_hashtag(&mut self, _ctx: &HashtagContext<'input>) {}

    fn exit_hashtag(&mut self, _ctx: &HashtagContext<'input>) {}

    fn enter_line_condition(&mut self, _ctx: &Line_conditionContext<'input>) {}

    fn exit_line_condition(&mut self, _ctx: &Line_conditionContext<'input>) {}

    fn enter_expParens(&mut self, _ctx: &ExpParensContext<'input>) {}

    fn exit_expParens(&mut self, _ctx: &ExpParensContext<'input>) {}

    fn enter_expMultDivMod(&mut self, _ctx: &ExpMultDivModContext<'input>) {}

    fn exit_expMultDivMod(&mut self, _ctx: &ExpMultDivModContext<'input>) {}

    fn enter_expComparison(&mut self, _ctx: &ExpComparisonContext<'input>) {}

    fn exit_expComparison(&mut self, _ctx: &ExpComparisonContext<'input>) {}

    fn enter_expNegative(&mut self, _ctx: &ExpNegativeContext<'input>) {}

    fn exit_expNegative(&mut self, _ctx: &ExpNegativeContext<'input>) {}

    fn enter_expAndOrXor(&mut self, _ctx: &ExpAndOrXorContext<'input>) {}

    fn exit_expAndOrXor(&mut self, _ctx: &ExpAndOrXorContext<'input>) {}

    fn enter_expAddSub(&mut self, _ctx: &ExpAddSubContext<'input>) {}

    fn exit_expAddSub(&mut self, _ctx: &ExpAddSubContext<'input>) {}

    fn enter_expNot(&mut self, _ctx: &ExpNotContext<'input>) {}

    fn exit_expNot(&mut self, _ctx: &ExpNotContext<'input>) {}

    fn enter_expValue(&mut self, _ctx: &ExpValueContext<'input>) {}

    fn exit_expValue(&mut self, _ctx: &ExpValueContext<'input>) {}

    fn enter_expEquality(&mut self, _ctx: &ExpEqualityContext<'input>) {}

    fn exit_expEquality(&mut self, _ctx: &ExpEqualityContext<'input>) {}

    fn enter_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) {}

    fn exit_valueNumber(&mut self, _ctx: &ValueNumberContext<'input>) {}

    fn enter_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) {}

    fn exit_valueTrue(&mut self, _ctx: &ValueTrueContext<'input>) {}

    fn enter_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) {}

    fn exit_valueFalse(&mut self, _ctx: &ValueFalseContext<'input>) {}

    fn enter_valueVar(&mut self, _ctx: &ValueVarContext<'input>) {}

    fn exit_valueVar(&mut self, _ctx: &ValueVarContext<'input>) {}

    fn enter_valueString(&mut self, _ctx: &ValueStringContext<'input>) {}

    fn exit_valueString(&mut self, _ctx: &ValueStringContext<'input>) {}

    fn enter_valueNull(&mut self, _ctx: &ValueNullContext<'input>) {}

    fn exit_valueNull(&mut self, _ctx: &ValueNullContext<'input>) {}

    fn enter_valueFunc(&mut self, _ctx: &ValueFuncContext<'input>) {}

    fn exit_valueFunc(&mut self, _ctx: &ValueFuncContext<'input>) {}

    fn enter_variable(&mut self, _ctx: &VariableContext<'input>) {}

    fn exit_variable(&mut self, _ctx: &VariableContext<'input>) {}

    fn enter_function_call(&mut self, _ctx: &Function_callContext<'input>) {}

    fn exit_function_call(&mut self, _ctx: &Function_callContext<'input>) {}

    fn enter_if_statement(&mut self, _ctx: &If_statementContext<'input>) {}

    fn exit_if_statement(&mut self, _ctx: &If_statementContext<'input>) {}

    fn enter_if_clause(&mut self, _ctx: &If_clauseContext<'input>) {}

    fn exit_if_clause(&mut self, _ctx: &If_clauseContext<'input>) {}

    fn enter_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) {}

    fn exit_else_if_clause(&mut self, _ctx: &Else_if_clauseContext<'input>) {}

    fn enter_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) {}

    fn exit_else_clause(&mut self, _ctx: &Else_clauseContext<'input>) {}

    fn enter_set_statement(&mut self, _ctx: &Set_statementContext<'input>) {}

    fn exit_set_statement(&mut self, _ctx: &Set_statementContext<'input>) {}

    fn enter_call_statement(&mut self, _ctx: &Call_statementContext<'input>) {}

    fn exit_call_statement(&mut self, _ctx: &Call_statementContext<'input>) {}

    fn enter_command_statement(&mut self, _ctx: &Command_statementContext<'input>) {}

    fn exit_command_statement(&mut self, _ctx: &Command_statementContext<'input>) {}

    fn enter_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) {}

    fn exit_command_formatted_text(&mut self, _ctx: &Command_formatted_textContext<'input>) {}

    fn enter_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) {
    }

    fn exit_shortcut_option_statement(&mut self, _ctx: &Shortcut_option_statementContext<'input>) {}

    fn enter_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) {}

    fn exit_shortcut_option(&mut self, _ctx: &Shortcut_optionContext<'input>) {}

    fn enter_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) {}

    fn exit_declare_statement(&mut self, _ctx: &Declare_statementContext<'input>) {}

    fn enter_jumpToNodeName(&mut self, _ctx: &JumpToNodeNameContext<'input>) {}

    fn exit_jumpToNodeName(&mut self, _ctx: &JumpToNodeNameContext<'input>) {}

    fn enter_jumpToExpression(&mut self, _ctx: &JumpToExpressionContext<'input>) {}

    fn exit_jumpToExpression(&mut self, _ctx: &JumpToExpressionContext<'input>) {}
}
