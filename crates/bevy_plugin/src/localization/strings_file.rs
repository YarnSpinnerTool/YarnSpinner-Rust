use crate::prelude::*;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;

#[derive(
    Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect, TypeUuid,
)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
#[uuid = "2e897914-f0f7-4b7f-b181-4d84b8ff6164"]
pub(crate) struct StringsFile(pub(crate) Vec<StringsFileRecord>);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct StringsFileRecord {
    language: String,
    id: LineId,
    text: String,
    file: String,
    node: String,
    line_number: usize,
    lock: String,
    comment: String,
}

#[derive(Debug, Default)]
pub(crate) struct StringsFileAssetLoader;

impl AssetLoader for StringsFileAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, SystemResult> {
        Box::pin(async move {
            let mut reader = csv::Reader::from_reader(bytes);
            let records: csv::Result<Vec<_>> = reader.deserialize().collect();
            let strings_file = StringsFile(records?);
            load_context.set_default_asset(LoadedAsset::new(strings_file));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["strings.csv"]
    }
}
