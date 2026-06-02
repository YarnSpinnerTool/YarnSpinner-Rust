//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Compiler/Utils.cs>

use crate::listeners::{DiagnosticVec, UntaggedLineListener};
use crate::prelude::generated::yarnspinnerparser::YarnSpinnerParserTreeWalker;
use crate::prelude::*;
use std::sync::atomic::Ordering;

impl Compiler {
    /// Given Yarn source code, adds line tags to the ends of all lines
    /// that need one and do not already have one.
    ///
    /// This method ensures that it does not generate line
    /// tags that are already present in the file, or present in the
    /// `existing_line_tags` collection.
    ///
    /// Line tags are added to any line of source code that contains
    /// user-visible text: lines, options, and shortcut options.
    ///
    /// ## Parameters
    ///
    /// `contents`: The source code to add line tags
    /// to.
    /// `existing_line_tags`: The collection of line tags
    /// already exist elsewhere in the source code; the newly added
    /// line tags will not be duplicates of any in this
    /// collection.
    ///
    /// ## Return value
    /// Returns the modified source code, with line tags added.
    /// If all nodes already have line tags, returns `None`.
    #[deprecated(
        note = "This method doesn't return the new tags, just the modified text which can cause issues with multiple files. Please use TagLines instead"
    )]
    pub fn add_tags_to_lines(
        contents: impl Into<String>,
        existing_line_tags: Vec<LineId>,
    ) -> crate::Result<Option<(String, Vec<LineId>)>> {
        let contents = contents.into();
        let chars: Vec<_> = contents.chars().map(|c| c as u32).collect();
        // First, get the parse tree for this source code.
        let file = File {
            file_name: "<input>".to_string(),
            source: contents,
        };
        let (parse_source, diagnostics) = parse_source(&file, &chars);
        let tree = parse_source.tree.clone();
        // Were there any error-level diagnostics?
        if diagnostics.has_errors() {
            // We encountered a parse error. Bail here; we aren't confident in our ability to correctly insert a line tag.
            return Err(CompilerError(diagnostics));
        }

        // Create the line listener, which will produce TextReplacements for each new line tag.
        let untagged_line_listener =
            Box::new(UntaggedLineListener::new(existing_line_tags, parse_source));
        let rewritten_nodes = untagged_line_listener.rewritten_lines.clone();
        let rewrote_anything = untagged_line_listener.rewrote_anything.clone();

        // Walk the tree with this listener, and generate text replacements containing line tags.
        let untagged_line_listener =
            YarnSpinnerParserTreeWalker::walk(untagged_line_listener, tree.as_ref())
                .expect("internal error: tree walk failed");
        // Apply these text replacements to the original source and return it.

        if rewrote_anything.load(Ordering::Relaxed) {
            let result = rewritten_nodes.take();
            let mut string = result.join("\n");
            string.push('\n');
            Ok(Some((
                string,
                untagged_line_listener.existing_line_tags.clone(),
            )))
        } else {
            Ok(None)
        }
    }

    /// Given Yarn source code, adds line tags to the ends of all lines
    /// that need one and do not already have one.
    ///
    /// This method ensures that it does not generate line
    /// tags that are already present in the file, or present in the
    /// `existing_line_tags` collection.
    ///
    /// Line tags are added to any line of source code that contains
    /// user-visible text: lines, options, and shortcut options.
    ///
    /// ## Parameters
    ///
    /// `contents`: The source code to add line tags
    /// to.
    /// `existing_line_tags`: The collection of line tags
    /// already exist elsewhere in the source code; the newly added
    /// line tags will not be duplicates of any in this
    /// collection.
    ///
    /// ## Return value
    /// Returns Tuple of the modified source code, with line tags
    /// added and the list of new line tags generated.
    pub fn tag_lines(
        contents: impl Into<String>,
        existing_line_tags: Vec<LineId>,
    ) -> crate::Result<Option<(String, Vec<LineId>)>> {
        let contents = contents.into();
        let chars: Vec<_> = contents.chars().map(|c| c as u32).collect();
        // First, get the parse tree for this source code.
        let file = File {
            file_name: "<input>".to_string(),
            source: contents,
        };
        let (parse_source, diagnostics) = parse_source(&file, &chars);
        let tree = parse_source.tree.clone();
        // Were there any error-level diagnostics?
        if diagnostics.has_errors() {
            // We encountered a parse error. Bail here; we aren't confident in our ability to correctly insert a line tag.
            return Err(CompilerError(diagnostics));
        }

        // Create the line listener, which will produce TextReplacements for each new line tag.
        let untagged_line_listener =
            Box::new(UntaggedLineListener::new(existing_line_tags, parse_source));
        let rewritten_nodes = untagged_line_listener.rewritten_lines.clone();
        let rewrote_anything = untagged_line_listener.rewrote_anything.clone();

        // Walk the tree with this listener, and generate text replacements containing line tags.
        let untagged_line_listener =
            YarnSpinnerParserTreeWalker::walk(untagged_line_listener, tree.as_ref())
                .expect("interal error: tree walk failed");
        // Apply these text replacements to the original source and return it.

        if rewrote_anything.load(Ordering::Relaxed) {
            let result = rewritten_nodes.take();
            let mut string = result.join("\n");
            string.push('\n');
            Ok(Some((
                string,
                untagged_line_listener.existing_line_tags.clone(),
            )))
        } else {
            Ok(None)
        }
    }
}

/// Parses a string of Yarn source code, and produces a [`FileParseResult`]
/// and (if there were any problems) a collection of [`Diagnostic`]s.
fn parse_source<'a, 'b: 'a>(
    file: &'b File,
    chars: &'a [u32],
) -> (FileParseResult<'a>, Vec<Diagnostic>) {
    let mut diagnostics = Vec::new();

    let result = parse_syntax_tree(file, chars, &mut diagnostics);

    (result, diagnostics)
}
