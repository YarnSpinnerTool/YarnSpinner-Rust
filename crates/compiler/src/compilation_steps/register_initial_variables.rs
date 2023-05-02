use crate::prelude::*;
use yarn_slinger_core::prelude::*;

pub(crate) fn register_initial_variables(
    mut state: CompilationIntermediate,
) -> CompilationIntermediate {
    let variables = &mut state.known_variable_declarations;
    let job_variable_declarations = state.job.variable_declarations.clone();
    variables.extend(job_variable_declarations);
    let standard_library_declarations = get_declarations_from_library(&Library::standard_library());
    variables.extend(standard_library_declarations);
    if let Some(library) = &state.job.library {
        let job_library_declarations = get_declarations_from_library(library);
        variables.extend(job_library_declarations);
    }
    state
}
