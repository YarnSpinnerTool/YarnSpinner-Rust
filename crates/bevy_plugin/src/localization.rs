pub use self::config::*;
use self::{line_id_generation::*, strings_file_asset::*, strings_file_creation::*};
use crate::prelude::*;
use bevy::prelude::*;

mod config;
mod line_id_generation;
mod strings_file_asset;
mod strings_file_creation;

pub(crate) fn localization_plugin(app: &mut App) {
    app.fn_plugin(localization_config_plugin)
        .fn_plugin(line_id_generation_plugin)
        .fn_plugin(strings_file_asset_plugin)
        .fn_plugin(strings_file_manipulation_plugin);
}
