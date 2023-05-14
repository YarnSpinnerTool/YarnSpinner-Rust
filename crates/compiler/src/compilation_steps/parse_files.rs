use crate::prelude::*;

pub(crate) fn parse_files(mut state: CompilationIntermediate) -> CompilationIntermediate {
    for i in 0..state.job.files.len() {
        let file = &state.job.files[i];
        let chars = &state.file_chars[i];
        let parse_result = parse_syntax_tree(file, chars, &mut state.diagnostics);
        state.parsed_files.push(parse_result);
    }
    state
}
