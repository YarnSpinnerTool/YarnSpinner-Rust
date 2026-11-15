use crate::prelude::*;

// TEMP:
// This whole module should either be deleted or move to `core` once we get ysc compilation support.

/// Additional functionality on [`CompiledFile`]
pub trait CompiledFileExt {
    /// Creates a [`CompiledFile`] from a rust [`Compilation`] result.
    fn from_compilation(c: Compilation) -> Self;
}

impl CompiledFileExt for CompiledFile {
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
            lines: yarnspinner::core::YscLines::new(lines),
            metadata: yarnspinner::core::YscMetadata::new(metadata),
        }
    }
}
