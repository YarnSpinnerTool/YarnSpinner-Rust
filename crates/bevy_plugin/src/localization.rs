pub use self::localizations::*;
pub(crate) use self::{
    line_id_generation::LineIdUpdateSystemSet,
    strings_file::UpdateAllStringsFilesForStringTableEvent, strings_file::*,
};
use bevy::prelude::*;

mod line_id_generation;
mod localizations;
mod strings_file;

pub(crate) fn localization_plugin(app: &mut App) {
    app.add_plugins(localizations::localization_config_plugin)
        .add_plugins(line_id_generation::line_id_generation_plugin)
        .add_plugins(strings_file::strings_file_plugin);
}
