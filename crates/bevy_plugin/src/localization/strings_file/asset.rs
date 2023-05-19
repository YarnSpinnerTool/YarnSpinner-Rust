//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner-Unity/blob/462c735766a4c4881cd1ef1f15de28c83b2ba0a8/Runtime/StringTableEntry.cs>

use crate::localization::strings_file::StringsFile;
use crate::prelude::*;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;

pub(crate) fn strings_file_asset_plugin(app: &mut App) {
    app.add_asset::<StringsFile>()
        .init_asset_loader::<StringsFileAssetLoader>();
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
            let strings_file = StringsFile::new_with_single_language(records?);
            load_context.set_default_asset(LoadedAsset::new(strings_file));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["strings.csv"]
    }
}
