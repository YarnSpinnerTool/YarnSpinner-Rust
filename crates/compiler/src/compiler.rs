use std::borrow::BorrowMut;
use std::collections::HashMap;

use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::tree::{ParseTreeListener, ParseTreeVisitorCompat, Tree};
use parser::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;
use parser::generated::yarnspinnerparservisitor::YarnSpinnerParserVisitor;

use super::*;
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
pub fn compile(compilation_job: CompilationJob) -> CompilationResult {
    // TODO: other steps
    let compiler_steps: Vec<&dyn CompilerStep> = vec![&add_built_in_types, &create_string_tables];

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
        .fold(initial, |acc, curr| curr.apply(&compilation_job, acc))
}

trait CompilerStep {
    fn apply(&self, job: &CompilationJob, previous: CompilationResult) -> CompilationResult;
}

impl<F> CompilerStep for F
where
    F: Fn(&CompilationJob, CompilationResult) -> CompilationResult,
{
    fn apply(&self, job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
        self(job, previous)
    }
}

fn add_built_in_types(job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
    previous
}

fn create_string_tables(job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
    // TODO:
    // # LastLineBeforeOptionsVisitor not done

    previous
}

/// Represents StringTableGeneratorVisitor
struct StringTableGeneratorVisitor(HashMap<String, StringInfo>);

impl<'input> ParseTreeVisitorCompat<'input> for StringTableGeneratorVisitor {
    type Node = YarnSpinnerParserContextType;

    type Return = HashMap<String, StringInfo>;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.0
    }
}

impl<'input> YarnSpinnerParserVisitor<'input> for StringTableGeneratorVisitor {
    /// VisitLine_statement of StringTableGeneratorVisitor
    fn visit_line_statement(&mut self, ctx: &Line_statementContext<'input>) {
        let line_number = ctx.start().line;
        let hashtags = ctx.hashtag_all();
        let line_id_tag: Option<HashtagContext> = todo!();
        let line_id = line_id_tag.as_ref().and_then(|t| t.text.as_ref());

        let hashtag_texts = hashtags.iter().map(|t| t.text.as_ref());

        let generate_formatted_text = |_| (todo!(), todo!());
        let (composed_string, expression_count) =
            generate_formatted_text(ctx.line_formatted_text().unwrap().get_children());

        if let Some(line_id) = line_id {
            if self.0.contains_key(&line_id.to_string()) {
                // TODO: Duplicate line ID, add to diagnostics
            }
        };

        // TODO
        self.0.insert(
            composed_string,
            StringInfo {
                text: (),
                node_name: (),
                line_number: (),
                file_name: (),
                is_implicit_tag: (),
                metadata: (),
            },
        );

        /*




               string stringID = stringTableManager.RegisterString(
                   composedString.ToString(),
                   fileName,
                   currentNodeName,
                   lineID,
                   lineNumber,
                   hashtagText);

               if (lineID == null)
               {
                   var hashtag = new YarnSpinnerParser.HashtagContext(context, 0);
                   hashtag.text = new CommonToken(YarnSpinnerLexer.HASHTAG_TEXT, stringID);
                   context.AddChild(hashtag);
               }

               return 0;
        */
        self.visit_children(ctx);
    }
}

#[cfg(test)]
mod test {
    use super::CompilationJob;
    use super::*;

    #[test]
    fn can_call_compile_without_crash() {
        compile(CompilationJob {
            files: vec![],
            library: None,
            compilation_type: CompilationType::FullCompilation,
            variable_declarations: vec![],
        });
    }
}
