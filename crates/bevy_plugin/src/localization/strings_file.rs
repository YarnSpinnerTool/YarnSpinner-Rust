use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::utils::HashMap;
use seldom_fn_plugin::FnPluginExt;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::path::Path;

mod asset;
mod creation;
mod updating;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.register_type::<StringsFile>()
        .register_type::<StringsFileRecord>()
        .fn_plugin(asset::strings_file_asset_plugin)
        .fn_plugin(creation::strings_file_creation_plugin)
        .fn_plugin(updating::strings_file_updating_plugin);
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect, TypeUuid,
)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
#[uuid = "2e897914-f0f7-4b7f-b181-4d84b8ff6164"]
#[non_exhaustive]
pub(crate) struct StringsFile(pub(crate) Vec<StringsFileRecord>);

/// Generates a string with the line metadata. This string is intended
/// to be used in the "comment" column of a strings table CSV. Because
/// of this, it will ignore the line ID if it exists (which is also
/// part of the line metadata).
///
/// ## Return value
/// A string prefixed with "Line metadata: ", followed by each
/// piece of metadata separated by whitespace. If no metadata exists or
/// only the line ID is part of the metadata, returns an empty string
/// instead.
fn read_comments(metadata: &[String]) -> String {
    // Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Importers/YarnProjectImporter.cs#L652>
    let cleaned_metadata: Vec<_> = metadata
        .iter()
        .filter(|metadata| !metadata.starts_with("line:"))
        .cloned()
        .collect();
    if cleaned_metadata.is_empty() {
        String::new()
    } else {
        format!("Line metadata: {}", cleaned_metadata.join(" "))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct StringsFileRecord {
    /// The language that the line is written in.
    pub(crate) language: Language,
    /// The line ID for this line. This value will be the same across all localizations.
    pub(crate) id: LineId,
    /// The text of this line, in the language specified by [`language`](StringsFileRecord::language).
    pub(crate) text: String,
    /// The name of the Yarn script in which this line was originally found.
    pub(crate) file: String,
    /// The name of the node in which this line was originally found.
    ///
    /// This node can be found in the file indicated by [`file`](StringsFileRecord::file).
    pub(crate) node: String,

    /// The 1-indexed line number in the file indicated by [`file`](StringsFileRecord::file) at
    /// which the original version of this line can be found.
    pub(crate) line_number: usize,
    /// A string used as part of a mechanism for checking if translated
    /// versions of this string are out of date.
    ///
    /// This field contains the first 8 characters of the SHA-256 hash of
    /// the line's text as it appeared in the base localization CSV file.
    ///
    /// When a new StringTableEntry is created in a localized CSV file for a
    /// .yarn file, the Lock value is copied over from the base CSV file,
    /// and used for the translated entry.
    ///
    /// Because the base localization CSV is regenerated every time the
    /// .yarn file is imported, the base localization Lock value will change
    /// if a line's text changes. This means that if the base lock and
    /// translated lock differ, the translated line is out of date, and
    /// needs to be updated.
    pub(crate) lock: Lock,
    /// A comment used to describe this line to translators.
    pub(crate) comment: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct Lock(String);

impl Lock {
    /// Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Editor/Importers/YarnImporter.cs#L149>
    pub(crate) fn compute_from(text: &str) -> Self {
        const MAX_CHARS: usize = 8;
        let hash = Sha256::digest(text);
        let hex = format!("{hash:x}");
        let lock = hex.chars().take(MAX_CHARS).collect();
        Self(lock)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
struct LanguagesToStringsFiles(HashMap<Language, Handle<StringsFile>>);

impl LanguagesToStringsFiles {
    fn get_language(&self, handle: &Handle<StringsFile>) -> Option<&Language> {
        self.0
            .iter()
            .find_map(|(lang, h)| (h == handle).then_some(lang))
    }
}

impl StringsFile {
    pub(crate) fn new_with_single_language(records: Vec<StringsFileRecord>) -> Result<Self> {
        if let Some(language) = records.first().map(|record| &record.language) {
            for record in records.iter().skip(1) {
                if record.language != *language {
                    bail!("Loaded strings file with mixed languages records must have the same language. Expected \"{}\", got \"{}\" in record: {:#?}",
                    language,
                    record.language,
                    record)
                }
            }
        }
        Ok(Self(records))
    }
    pub(crate) fn language(&self) -> Option<&Language> {
        self.0.first().map(|record| &record.language)
    }
    pub(crate) fn from_yarn_files<'a>(
        language: impl Into<Language>,
        files: impl Iterator<Item = &'a YarnFile>,
    ) -> Self {
        let mut files: Vec<_> = files
            .flat_map(|yarn_file| {
                yarn_file
                    .string_table
                    .iter()
                    .map(|(id, line_info)| (id, line_info, yarn_file.file.file_name.as_str()))
                    .collect::<Vec<_>>()
            })
            .collect();
        files.sort_by(
            |(_, lhs_string_info, lhs_file_name), (_, rhs_string_info, rhs_file_name)| {
                lhs_file_name.cmp(rhs_file_name).then(
                    lhs_string_info
                        .line_number
                        .cmp(&rhs_string_info.line_number),
                )
            },
        );
        let language = language.into();
        let strings_file_records = files
            .into_iter()
            .map(|(line_id, string_info, file_name)| StringsFileRecord {
                language: language.clone(),
                id: line_id.clone(),
                text: string_info.text.clone(),
                file: file_name.to_string(),
                node: string_info.node_name.clone(),
                line_number: string_info.line_number,
                lock: Lock::compute_from(&string_info.text),
                comment: read_comments(&string_info.metadata),
            })
            .collect();
        Self(strings_file_records)
    }

    pub(crate) fn write_asset(&self, asset_server: &AssetServer, path: &Path) -> Result<()> {
        let assets_path = get_assets_dir_path(asset_server)?;
        let assets_path = assets_path.as_ref();
        let full_path = assets_path.join(path);
        let file = File::create(&full_path)
            .with_context(|| format!("Failed to create strings file \"{}\"", full_path.display(),))?;
        let mut writer = csv::Writer::from_writer(file);
        for record in &self.0 {
            writer.serialize(record)?;
        }
        writer.flush()?;
        Ok(())
    }
}
