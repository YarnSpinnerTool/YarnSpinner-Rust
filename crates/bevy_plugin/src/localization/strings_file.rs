pub(crate) use self::asset::StringsFile;
#[cfg(feature = "compiler")]
pub(crate) use self::updating::UpdateAllStringsFilesForStringTableEvent;
use bevy::prelude::*;

mod asset;
#[cfg(feature = "compiler")]
mod updating;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.add_plugins(asset::strings_file_asset_plugin);
    #[cfg(feature = "compiler")]
    app.add_plugins(updating::strings_file_updating_plugin);
}
