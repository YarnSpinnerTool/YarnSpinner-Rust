use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Slinger plugin: {e}");
    }
}

pub(crate) fn is_in_development(localizations: Option<Res<Localizations>>) -> bool {
    localizations
        .as_ref()
        .map(|localizations| localizations.file_generation_mode == FileGenerationMode::Development)
        .unwrap_or_default()
}
