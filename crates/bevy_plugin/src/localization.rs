pub use self::localizations::*;
pub(crate) use self::{
    line_id_generation::LineIdUpdateSystemSet,
    strings_file::UpdateAllStringsFilesForStringTableEvent, strings_file::*,
};
use crate::prelude::*;
use bevy::prelude::*;

mod line_id_generation;
mod localizations;
mod strings_file;

pub(crate) fn localization_plugin(app: &mut App) {
    app.fn_plugin(localizations::localization_config_plugin)
        .fn_plugin(line_id_generation::line_id_generation_plugin)
        .fn_plugin(strings_file::strings_file_plugin);
}
