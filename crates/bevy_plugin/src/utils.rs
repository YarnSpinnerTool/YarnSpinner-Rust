use crate::prelude::*;
use crate::project::YarnProjectConfigToLoad;
use bevy::prelude::*;

pub(crate) fn panic_on_err(In(result): In<SystemResult>) {
    if let Err(e) = result {
        panic!("Error in Yarn Spinner plugin: {e}");
    }
}

pub(crate) fn log_error(In(result): In<SystemResult>) {
    if let Err(e) = result {
        error!("Error in Yarn Spinner plugin: {e}");
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

pub(crate) fn events_in_queue<T: Message>() -> impl FnMut(MessageReader<T>) -> bool + Clone {
    move |reader: MessageReader<T>| !reader.is_empty()
}
