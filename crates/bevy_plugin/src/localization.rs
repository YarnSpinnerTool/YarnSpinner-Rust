pub use self::config::*;
use crate::localization::line_id_generation::line_id_generation_plugin;
use crate::prelude::*;
use bevy::prelude::*;

mod config;
mod line_id_generation;
mod strings_file;

pub(crate) fn localization_plugin(app: &mut App) {
    app.fn_plugin(localization_config_plugin)
        .fn_plugin(line_id_generation_plugin);
}
