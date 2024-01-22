use crate::prelude::*;
use crate::project::YarnProjectConfigToLoad;
use bevy::prelude::*;
use std::path::{Path, PathBuf};

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

pub(crate) trait AssetServerExt {
    fn get_assets_dir_path(&self) -> Result<PathBuf>;
    fn is_file(&self, path: impl AsRef<Path>) -> bool;
}

impl AssetServerExt for AssetServer {
    fn get_assets_dir_path(&self) -> Result<PathBuf> {
        // Revert https://github.com/yarn-slinger/yarn-slinger/pull/164/commits/b82f5d2be090b6f1367b740f3b29be5fa7b25723
        // as soon as https://github.com/bevyengine/bevy/issues/10455 has been resolved
        Ok(PathBuf::from("./assets"))
    }

    fn is_file(&self, path: impl AsRef<Path>) -> bool {
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        panic!("AssetServer::is_file is not supported on this platform. This is a bug. Please report it at https://github.com/yarn-slinger/yarn_slinger/issues/new");
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            let Ok(root) = self.get_assets_dir_path() else {
                return false;
            };
            let path = root.join(path.as_ref());
            path.is_file()
        }
    }
}
