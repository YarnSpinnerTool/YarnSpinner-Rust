//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Runtime/StringTableEntry.cs>

use crate::prelude::*;
use anyhow::{Result, bail};
use bevy::asset::{AssetLoader, LoadContext, io::Reader};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::reflect::TypePath;

pub(crate) fn strings_file_asset_plugin(app: &mut App) {
    app.init_asset::<StringsFile>()
        .init_asset_loader::<StringsFileAssetLoader>();
}

#[derive(Debug, Default, TypePath)]
struct StringsFileAssetLoader;

impl AssetLoader for StringsFileAssetLoader {
    type Asset = StringsFile;
    type Settings = ();
    type Error = anyhow::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        _load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut csv_reader = csv::Reader::from_reader(bytes.as_slice());
        let records: csv::Result<Vec<_>> = csv_reader.deserialize().collect();
        let strings_file = StringsFile::new_with_single_language(records?)?;
        Ok(strings_file)
    }

    fn extensions(&self) -> &[&str] {
        &["strings.csv"]
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Asset, TypePath)]
#[non_exhaustive]
pub(crate) struct StringsFile(HashMap<LineId, StringsFileRecord>);

impl StringsFile {
    pub(crate) fn new_with_single_language(records: Vec<StringsFileRecord>) -> Result<Self> {
        if let Some(language) = records.first().map(|record| &record.language) {
            for record in records.iter().skip(1) {
                if record.language != *language {
                    bail!(
                        "Loaded strings file with mixed languages records must have the same language. Expected \"{}\", got \"{}\" in record: {:#?}",
                        language,
                        record.language,
                        record
                    )
                }
            }
        }
        let records = records
            .into_iter()
            .map(|record| (record.id.clone(), record))
            .collect::<HashMap<_, _>>();
        Ok(Self(records))
    }

    pub(crate) fn get_offending_language(
        &self,
        expected_language: &Language,
    ) -> Option<&StringsFileRecord> {
        self.0
            .values()
            .find(|record| &record.language != expected_language)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&LineId, &StringsFileRecord)> {
        self.0.iter()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
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
    /// .Yarn file, the Lock value is copied over from the base CSV file,
    /// and used for the translated entry.
    ///
    /// Because the base localization CSV is regenerated every time the
    /// .Yarn file is imported, the base localization Lock value will change
    /// if a line's text changes. This means that if the base lock and
    /// translated lock differ, the translated line is out of date, and
    /// needs to be updated.
    pub(crate) lock: Lock,
    /// A comment used to describe this line to translators.
    pub(crate) comment: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct Lock(String);
