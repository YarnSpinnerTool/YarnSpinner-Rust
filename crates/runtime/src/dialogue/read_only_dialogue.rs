use crate::prelude::*;
use std::sync::{Arc, RwLock};
use yarn_slinger_core::prelude::*;

#[derive(Debug, Clone)]
pub struct ReadOnlyDialogue {
    pub(crate) program: Arc<RwLock<Option<Program>>>,
    pub(crate) log_debug_message: Logger,
    pub(crate) log_error_message: Logger,
}

impl ReadOnlyDialogue {
    /// Gets the names of the nodes in the currently loaded Program, if there is one.
    pub fn node_names(&self) -> Option<Vec<String>> {
        self.program
            .read()
            .unwrap()
            .as_ref()
            .map(|program| program.nodes.keys().cloned().collect())
    }

    /// Returns the string ID that contains the original, uncompiled source
    /// text for a node.
    ///
    /// A node's source text will only be present in the string table if its
    /// `tags` header contains `rawText`.
    ///
    /// Because the [`Dialogue`] API is designed to be unaware
    /// of the contents of the string table, this method does not test to
    /// see if the string table contains an entry with the line ID. You will
    /// need to test for that yourself.
    pub fn get_string_id_for_node(&self, node_name: &str) -> Option<String> {
        self.get_node_logging_errors(node_name)
            .map(|_| format!("line:{node_name}"))
    }

    /// Returns the tags for the node `node_name`.
    ///
    /// The tags for a node are defined by setting the `tags` header in
    /// the node's source code. This header must be a space-separated list
    ///
    /// Returns [`None`] if the node is not present in the program.
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.get_node_logging_errors(node_name)
            .map(|node| node.tags)
    }

    /// Gets a value indicating whether a specified node exists in the
    /// Program.
    pub fn node_exists(&self, node_name: &str) -> bool {
        // Not calling `get_node_logging_errors` because this method does not write errors when there are no nodes.
        if let Some(program) = self.program.read().unwrap().as_ref() {
            program.nodes.contains_key(node_name)
        } else {
            self.log_error_message
                .call("Tried to call NodeExists, but no program has been loaded".to_owned());
            false
        }
    }

    /// Replaces all substitution markers in a text with the given
    /// substitution list.
    ///
    /// This method replaces substitution markers - for example, `{0}`
    /// - with the corresponding entry in `substitutions`.
    /// If `test` contains a substitution marker whose
    /// index is not present in `substitutions`, it is
    /// ignored.
    pub fn expand_substitutions<'a>(
        text: &str,
        substitutions: impl IntoIterator<Item = &'a str>,
    ) -> String {
        substitutions
            .into_iter()
            .enumerate()
            .fold(text.to_owned(), |text, (i, substitution)| {
                text.replace(&format!("{{{i}}}",), substitution)
            })
    }

    fn get_node_logging_errors(&self, node_name: &str) -> Option<Node> {
        if let Some(program) = self.program.read().unwrap().as_ref() {
            if program.nodes.is_empty() {
                self.log_error_message
                    .call("No nodes are loaded".to_owned());
                None
            } else if let Some(node) = program.nodes.get(node_name) {
                Some(node.clone())
            } else {
                self.log_error_message
                    .call(format!("No node named {node_name}"));
                None
            }
        } else {
            self.log_error_message
                .call("No program is loaded".to_owned());
            None
        }
    }
}
