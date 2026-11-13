use crate::prelude::*;
use std::collections::HashMap;

/// Additional functionality on [`CompiledFile`]
pub trait CompiledFileExt {
    /// Generates a table from [`LineId`] to [`StringInfo`]
    fn string_table(&self) -> HashMap<LineId, StringInfo>;
    /// Creates a [`CompiledFile`] from a rust [`Compilation`] result.
    fn from_compilation(c: Compilation) -> Self;
}

impl CompiledFileExt for CompiledFile {
    /// Converts this raw line data to something more useful.
    /// This returns owned data by cloning.
    fn string_table(&self) -> HashMap<LineId, StringInfo> {
        let tags = self.metadata.tags_by_id();
        self.lines
            .iter()
            .map(|line| {
                let info = StringInfo {
                    text: line.text.clone(),
                    node_name: line.node.clone(),
                    line_number: line.line_number,
                    file_name: line.file.clone(),
                    is_implicit_tag: false, // ysc already gave every line an ID
                    metadata: tags
                        .get(&line.id)
                        .map(|tags| tags.to_vec())
                        .unwrap_or_default(),
                };
                (line.id.clone(), info)
            })
            .collect()
    }

    fn from_compilation(c: Compilation) -> Self {
        let mut lines = Vec::with_capacity(c.string_table.len());
        let mut metadata = Vec::new();
        for (id, info) in c.string_table {
            if !info.metadata.is_empty() {
                metadata.push(MetadataLine {
                    id: id.clone(),
                    node: info.node_name.clone(),
                    line_number: info.line_number,
                    tags: info.metadata,
                });
            }
            lines.push(YscLine {
                id,
                text: info.text,
                file: info.file_name,
                node: info.node_name,
                line_number: info.line_number,
            });
        }
        Self {
            program: c.program.expect("full compilation has a program"),
            lines: lines.into(),
            metadata: metadata.into(),
        }
    }
}
