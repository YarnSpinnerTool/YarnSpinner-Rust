use crate::prelude::*;
use yarn_slinger_core::prelude::*;

pub(crate) fn parse_files(mut state: CompilationIntermediate) -> CompilationIntermediate {
    for file in &state.job.files {
        let parse_result = parse_syntax_tree(file, &mut state.diagnostics);
        state.parsed_files.push(parse_result);
    }
    state
}
