pub(crate) use self::{asset::StringsFile, updating::UpdateAllStringsFilesForStringTableEvent};
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

mod asset;
mod updating;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.fn_plugin(asset::strings_file_asset_plugin)
        .fn_plugin(updating::strings_file_updating_plugin);
}
