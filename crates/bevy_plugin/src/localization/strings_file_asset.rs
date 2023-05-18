//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Runtime/StringTableEntry.cs>

use crate::prelude::*;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub(crate) fn strings_file_asset_plugin(app: &mut App) {
    app.register_type::<StringsFile>()
        .register_type::<StringsFileRecord>()
        .add_asset::<StringsFile>()
        .init_asset_loader::<StringsFileAssetLoader>();
}

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect, TypeUuid,
)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
#[uuid = "2e897914-f0f7-4b7f-b181-4d84b8ff6164"]
#[non_exhaustive]
pub(crate) struct StringsFile(pub(crate) Vec<StringsFileRecord>);

impl StringsFile {
    pub(crate) fn new(records: Vec<StringsFileRecord>) -> Self {
        if !records.is_empty() {
            let language = &records[0].language;
            assert!(
                records.iter().all(|record| &record.language == language),
                "Failed to load strings file: some records have different languages."
            );
        }
        Self(records)
    }

    pub(crate) fn language(&self) -> Option<&Language> {
        self.0.first().map(|record| &record.language)
    }

    pub(crate) fn has_language(&self, language: &Language) -> bool {
        self.language() == Some(language)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct StringsFileRecord {
    pub(crate) language: Language,
    pub(crate) id: LineId,
    pub(crate) text: String,
    pub(crate) file: String,
    pub(crate) node: String,
    pub(crate) line_number: usize,
    pub(crate) lock: String,
    pub(crate) comment: String,
}

#[derive(Debug, Default)]
struct StringsFileAssetLoader;

impl AssetLoader for StringsFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, SystemResult> {
        Box::pin(async move {
            let mut reader = csv::Reader::from_reader(bytes);
            let records: csv::Result<Vec<_>> = reader.deserialize().collect();
            let strings_file = StringsFile::new(records?);
            load_context.set_default_asset(LoadedAsset::new(strings_file));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["strings.csv"]
    }
}
