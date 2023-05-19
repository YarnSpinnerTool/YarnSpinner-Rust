use crate::prelude::*;
use anyhow::Context;
use bevy::prelude::*;

pub(crate) fn strings_file_updating_plugin(app: &mut App) {
    app.add_system(
        update_strings_file_on_yarn_file_change
            .pipe(panic_on_err)
            .run_if(resource_exists::<Localizations>()),
    );
}

fn update_strings_file_on_yarn_file_change() -> SystemResult {
    Ok(())
}
