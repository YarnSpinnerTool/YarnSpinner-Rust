use crate::prelude::*;
use log::{debug, error};
use std::sync::{Arc, RwLock};
use yarn_slinger_core::prelude::*;

/// A view of a [`Dialogue`]. Represents the subset of methods that are okay to be called from handlers.
/// Since this type is `Send + Sync`, you can get a copy with [`Dialogue::get_handler_safe_dialogue`] and `move` it into a handler.
///
/// ## Implementation notes
///
/// This type is not present in the original. We need to use it to cleanly borrow data from handlers.
/// The original just calls [`Dialogue`] for both mutable and immutable access anywhere,
/// which is of course a big no-no in Rust.
#[derive(Debug, Clone)]
pub struct HandlerSafeDialogue {
    pub(crate) program: Arc<RwLock<Option<Program>>>,
    pub(crate) current_node_name: Arc<RwLock<Option<String>>>,
    pub(crate) log_debug_message: Logger,
    pub(crate) log_error_message: Logger,
    pub(crate) language_code: Arc<RwLock<Option<String>>>,
}

impl Default for HandlerSafeDialogue {
    fn default() -> Self {
        HandlerSafeDialogue {
            program: Arc::new(RwLock::new(None)),
            current_node_name: Arc::new(RwLock::new(None)),
            log_debug_message: Logger(Box::new(|msg, _| debug!("{}", msg))),
            log_error_message: Logger(Box::new(|msg, _| error!("{}", msg))),
            language_code: Arc::new(RwLock::new(None)),
        }
    }
}

impl HandlerSafeDialogue {
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
            self.log_error_message.call(
                "Tried to call NodeExists, but no program has been loaded".to_owned(),
                self,
            );
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

    /// Gets the name of the node that this Dialogue is currently executing.
    ///
    /// If [`Dialogue::continue_`] has never been called, this value
    /// will be [`None`].
    pub fn current_node(&self) -> Option<String> {
        self.current_node_name.read().unwrap().clone()
    }

    /// The [`Dialogue`]'s locale, as an IETF BCP 47 code.
    ///
    /// This code is used to determine how the `plural` and `ordinal`
    /// markers determine the plural class of numbers.
    ///
    /// For example, the code "en-US" represents the English language as
    /// used in the United States.
    pub fn language_code(&self) -> Option<String> {
        self.language_code.read().unwrap().clone()
    }

    pub fn analyse(&self) -> ! {
        todo!()
    }

    pub fn parse_markup(&self, line: &str) -> String {
        // ## Implementation notes
        // It would be more ergonomic to not expose this and call it automatically.
        // We should probs remove this from the API.
        // Pass the MarkupResult directly into the LineHandler
        // todo!()
        line.to_owned()
    }

    fn get_node_logging_errors(&self, node_name: &str) -> Option<Node> {
        if let Some(program) = self.program.read().unwrap().as_ref() {
            if program.nodes.is_empty() {
                self.log_error_message
                    .call("No nodes are loaded".to_owned(), self);
                None
            } else if let Some(node) = program.nodes.get(node_name) {
                Some(node.clone())
            } else {
                self.log_error_message
                    .call(format!("No node named {node_name}"), self);
                None
            }
        } else {
            self.log_error_message
                .call("No program is loaded".to_owned(), self);
            None
        }
    }
}
