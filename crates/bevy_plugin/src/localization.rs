pub use self::config::*;
use self::line_id_generation::*;
use crate::prelude::*;
use bevy::prelude::*;

mod config;
mod line_id_generation;

pub(crate) struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Localizations>()
            .register_type::<Localization>()
            .register_type::<FileGenerationMode>()
            .add_system(
                generate_missing_line_ids_in_yarn_file
                    .pipe(panic_on_err)
                    .run_if(is_in_development),
            );
    }
}
