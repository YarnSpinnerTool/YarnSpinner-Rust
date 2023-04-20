use antlr_rust::tree::ParseTreeListener;
use parser::generated::yarnspinnerparserlistener::YarnSpinnerParserListener;

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
    let compiler_steps: Vec<&dyn CompilerStep> =
        vec![&BuiltInTypesProvider {}, &StringTableGenerator {}];

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
        .fold(initial, |acc, curr| curr.run(&compilation_job, &acc))
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

trait CompilerStep {
    fn run(&self, job: &CompilationJob, previous: &CompilationResult) -> CompilationResult;
}

struct BuiltInTypesProvider {}

impl CompilerStep for BuiltInTypesProvider {
    fn run(&self, job: &CompilationJob, previous: &CompilationResult) -> CompilationResult {
        todo!()
    }
}

struct StringTableGenerator {}

impl CompilerStep for StringTableGenerator {
    fn run(&self, job: &CompilationJob, previous: &CompilationResult) -> CompilationResult {
        todo!()
    }
}

impl<'input> ParseTreeListener<'input, YarnSpinnerParserContextType> for StringTableGenerator {}

/// Adapted from https://github.com/YarnSpinnerTool/YarnSpinner/blob/v2.3.0/YarnSpinner.Compiler/StringTableGeneratorVisitor.cs
impl<'input> YarnSpinnerParserListener<'input> for StringTableGenerator {
    fn exit_dialogue(&mut self, _ctx: &DialogueContext<'input>) {
        println!("{:?}", _ctx);
    }
}

// TODO: needed?
impl<'input> StringTableGenerator {
    fn generate_string_tag_if_absent(&mut self, _ctx: &DialogueContext<'input>) {
        if !&self.has_string_tag(_ctx) {
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
