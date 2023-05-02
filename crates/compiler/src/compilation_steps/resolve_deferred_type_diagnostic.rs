use crate::prelude::*;

pub(crate) fn resolve_deferred_type_diagnostic(
    mut state: CompilationIntermediate,
) -> CompilationIntermediate {
    for deferred_type_diagnostic in &state.potential_issues {
        let resolved = state
            .known_variable_declarations
            .iter()
            .any(|decl| decl.name == deferred_type_diagnostic.name);

        if !resolved {
            state
                .diagnostics
                .push(deferred_type_diagnostic.diagnostic.clone())
        }
    }
    state
}
