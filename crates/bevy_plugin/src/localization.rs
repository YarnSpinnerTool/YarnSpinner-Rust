pub use self::localizations::*;
pub(crate) use self::strings_file::*;
use bevy::prelude::*;

mod localizations;
mod strings_file;

pub(crate) fn localization_plugin(app: &mut App) {
    app.add_plugins(localizations::localization_config_plugin)
        .add_plugins(strings_file::strings_file_plugin);
}
