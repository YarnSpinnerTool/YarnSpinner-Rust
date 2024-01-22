use crate::prelude::*;
use crate::project::YarnProjectConfigToLoad;
use bevy::prelude::*;
use std::path::Path;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Slinger plugin: {e}");
    }
}

pub(crate) fn in_development(
    project: Option<Res<YarnProject>>,
    project_to_load: Option<Res<YarnProjectConfigToLoad>>,
) -> bool {
    if let Some(project) = project {
        return project.development_file_generation == DevelopmentFileGeneration::Full;
    }
    if let Some(project_to_load) = project_to_load {
        return project_to_load.development_file_generation == DevelopmentFileGeneration::Full;
    }
    false
}

pub(crate) fn has_localizations(
    project: Option<Res<YarnProject>>,
    project_to_load: Option<Res<YarnProjectConfigToLoad>>,
) -> bool {
    if let Some(project) = project {
        return project.localizations.is_some();
    }
    if let Some(project_to_load) = project_to_load {
        return matches!(project_to_load.localizations, Some(Some(_)));
    }
    false
}

pub(crate) fn events_in_queue<T: Event>() -> impl FnMut(EventReader<T>) -> bool + Clone {
    move |reader: EventReader<T>| !reader.is_empty()
}

pub(crate) fn get_assets_dir_path(asset_server: &AssetServer) -> Result<impl AsRef<Path> + '_> {
    let _asset_server = asset_server;
    Ok(Path::new("./assets"))
}
