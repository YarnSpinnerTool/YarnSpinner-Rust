//! Replace the method AddErrorsForEmptyNodes of https://github.com/YarnSpinnerTool/YarnSpinner/blob/838761ad55d8d08be3b46e6f2bea2d017208e445/YarnSpinner.Compiler/Compiler.cs#L657

use std::collections::HashSet;

use antlr4rust::token::Token;

use crate::parser::generated::yarnspinnerparser::{
    BodyContextAttrs, DialogueContextAttrs, NodeContextAttrs,
};
use crate::prelude::CompilationIntermediate;
use crate::prelude::*;

pub(crate) fn add_error_for_empty_nodes(
    mut state: CompilationIntermediate,
) -> CompilationIntermediate {
    let mut empty_nodes: HashSet<String> = HashSet::new();

    let empties = state
        .parsed_files
        .iter()
        .flat_map(|(file, _)| {
            file.tree
                .node_all()
                .iter()
                .map(|node| (node.clone(), file))
                .collect::<Vec<_>>()
        })
        .filter(|(node, _)| {
            if let Some(body) = node.body() {
                body.statement_all().is_empty()
            } else {
                false
            }
        });

    for (node, file) in empties {
        let (title, title_header) = node
            .header_all()
            .iter()
            .find(|header| header.header_key.as_ref().unwrap().get_text() == "title")
            .map(|title_header| {
                let title = title_header
                    .header_value
                    .as_ref()
                    .unwrap()
                    .get_text()
                    .to_owned();
                (title, Some(title_header.clone()))
            })
            .unwrap_or_default();

        let mut diag = Diagnostic::from_message(format!(
            "Node \"{title}\" is empty and will not be included in the compiled output.",
        ))
        .with_file_name(file.name.clone())
        .with_severity(DiagnosticSeverity::Warning);

        if let Some(context) = title_header {
            diag = diag.with_parser_context(context.as_ref(), file.tokens());
        }

        state.diagnostics.push(diag);

        empty_nodes.insert(title);
    }

    state.skip_nodes = empty_nodes;
    state
}
