pub use self::config::*;
pub(crate) use self::strings_file::*;
pub use self::strings_file::{
    CreateMissingStringsFilesEvent, UpdateAllStringsFilesForStringTableEvent,
};
use crate::prelude::*;
use bevy::prelude::*;
pub use language::CurrentLanguage;

mod config;
mod language;
mod line_id_generation;
mod strings_file;

pub(crate) fn localization_plugin(app: &mut App) {
    app.fn_plugin(language::language_plugin)
        .fn_plugin(config::localization_config_plugin)
        .fn_plugin(line_id_generation::line_id_generation_plugin)
        .fn_plugin(strings_file::strings_file_plugin);
}
