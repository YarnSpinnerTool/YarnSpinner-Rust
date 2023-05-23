pub use self::creation::CreateMissingStringsFilesEvent;
pub(crate) use self::{
    asset::StringsFile,
    current::{CurrentStringsFile, UpdateBaseLanguageTextProviderForStringTableEvent},
    updating::UpdateAllStringsFilesForStringTableEvent,
};
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

mod asset;
mod creation;
mod current;
mod updating;

pub(crate) fn strings_file_plugin(app: &mut App) {
    app.fn_plugin(asset::strings_file_asset_plugin)
        .fn_plugin(creation::strings_file_creation_plugin)
        .fn_plugin(updating::strings_file_updating_plugin)
        .fn_plugin(current::current_strings_file_plugin);
}
