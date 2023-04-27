use crate::prelude::*;
use rusty_yarn_spinner_core::types::Type;

/// A visitor that walks the parse tree, checking for type consistency
/// in expressions. Existing type information is provided via the
/// [`existing_declarations`] property. This visitor will also
/// attempt to infer the type of variables that don't have an explicit
/// declaration; for each of these, a new Declaration will be created
/// and made available via the [`new_declaration`] property.
pub(crate) struct TypeCheckVisitor<'a, 'input: 'a> {
    /// <summary>
    /// Gets the collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    /// </summary>
    pub(crate) diagnostics: Vec<Diagnostic>,

    /// Gets the collection of new variable declarations that were
    /// found as a result of using this  [`TypeCheckVisitor`] to visit a [`ParserRuleContext`].
    pub(crate) new_declarations: Vec<Declaration>,

    // the list of variables we aren't actually sure about
    pub(crate) deferred_types: Vec<DeferredTypeDiagnostic>,

    // The collection of variable declarations we know about before
    // starting our work
    existing_declarations: Vec<Declaration>,

    // The name of the node that we're currently visiting.
    current_node_name: Option<String>,

    // The name of the file that we're currently in.
    source_file_name: String,

    tokens: &'a ActualTokenStream<'input>,
    _dummy: (),
}

impl<'a, 'input: 'a> TypeCheckVisitor<'a, 'input> {
    pub(crate) fn new(
        source_file_name: String,
        existing_declarations: Vec<Declaration>,
        tokens: &'a ActualTokenStream<'input>,
    ) -> Self {
        Self {
            existing_declarations,
            source_file_name,
            tokens,
            diagnostics: Default::default(),
            new_declarations: Default::default(),
            deferred_types: Default::default(),
            current_node_name: Default::default(),
            _dummy: Default::default(),
        }
    }

    /// Gets the collection of all declarations - both the ones we received
    /// at the start, and the new ones we've derived ourselves.
    pub(crate) fn declarations(&self) -> Vec<Declaration> {
        self.existing_declarations
            .iter()
            .cloned()
            .chain(self.new_declarations.iter().cloned())
            .collect()
    }
}

/// {0} = variable name
const CANT_DETERMINE_VARIABLE_TYPE_ERROR: &str = "Can't figure out the type of variable {0} given its context. Specify its type with a <<declare>> statement.";
