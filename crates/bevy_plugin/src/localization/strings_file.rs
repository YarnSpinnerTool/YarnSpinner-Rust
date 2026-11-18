pub(crate) use self::asset::StringsFile;
use bevy::prelude::*;

mod asset;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.add_plugins(asset::strings_file_asset_plugin);
}
