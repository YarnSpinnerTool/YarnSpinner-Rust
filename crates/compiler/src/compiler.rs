pub use crate::compiler::compilation_job::*;
use crate::output::*;
use crate::prelude::generated::yarnspinnerparser::*;
use antlr_rust::token::Token;
use antlr_rust::token_factory::{CommonTokenFactory, TokenFactory};
use antlr_rust::InputStream;
use std::rc::Rc;

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

pub(crate) fn get_line_id_tag<'a>(
    hashtag_contexts: &[Rc<HashtagContextAll<'a>>],
) -> Option<Rc<HashtagContextAll<'a>>> {
    hashtag_contexts
        .iter()
        .find(|h| h.text.as_ref().expect("Hashtag held no text").get_text() == "line:")
        .cloned()
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

fn add_built_in_types(_job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
    previous
}

fn create_string_tables(_job: &CompilationJob, previous: CompilationResult) -> CompilationResult {
    // TODO:
    // # LastLineBeforeOptionsVisitor not done

    previous
}

pub(crate) fn get_line_id_for_node_name(name: &str) -> String {
    format!("line:{name}")
}

fn add_hashtag_child<'input>(
    parent: Rc<impl YarnSpinnerParserContext<'input> + 'input>,
    token_factory: &'input CommonTokenFactory,
    text: String,
) {
    // Taken from C# implementation of `CommonToken`s constructor
    let string_id_token = token_factory.create::<InputStream<&'input str>>(
        None,
        HASHTAG_TEXT,
        text.into(),
        0,
        0,
        0,
        0,
        -1,
    );
    // `new_with_text` was hacked into the generated parser. Also, `FooContextExt::new` is usually private...
    let hashtag = HashtagContextExt::new_with_text(Some(parent.clone()), 0, string_id_token);
    parent.add_child(hashtag);
}

#[cfg(test)]
mod test {
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
