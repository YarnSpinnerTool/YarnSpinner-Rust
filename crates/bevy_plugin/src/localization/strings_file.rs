use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use seldom_fn_plugin::FnPluginExt;
use sha2::{Digest, Sha256};

mod asset;
mod creation;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.register_type::<StringsFile>()
        .register_type::<StringsFileRecord>()
        .fn_plugin(asset::strings_file_asset_plugin)
        .fn_plugin(creation::strings_file_creation_plugin);
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect, TypeUuid,
)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
#[uuid = "2e897914-f0f7-4b7f-b181-4d84b8ff6164"]
#[non_exhaustive]
pub(crate) struct StringsFile(pub(crate) Vec<StringsFileRecord>);

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
