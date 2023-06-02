use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;
pub(crate) use compilation::{
    RecompileLoadedYarnFilesEvent, YarnFilesBeingLoaded, YarnProjectConfigToLoad,
};
use std::fmt::Debug;
mod compilation;

pub(crate) fn project_plugin(app: &mut App) {
    app.fn_plugin(compilation::project_compilation_plugin)
        .add_event::<LoadYarnProjectEvent>()
        .register_type::<LoadYarnProjectEvent>();
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct CompilationSystemSet;

#[derive(Resource)]
pub struct YarnProject {
    pub(crate) yarn_files: HashSet<Handle<YarnFile>>,
    pub(crate) compilation: Compilation,
    pub(crate) localizations: Option<Localizations>,
    pub(crate) asset_server: AssetServer,
}

impl Debug for YarnProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("YarnProject")
            .field("yarn_files", &self.yarn_files)
            .field("compilation", &self.compilation)
            .field("localizations", &self.localizations)
            .field("asset_server", &())
            .finish()
    }
}

impl YarnProject {
    pub fn yarn_files(&self) -> impl Iterator<Item = &Handle<YarnFile>> {
        self.yarn_files.iter()
    }

    pub fn compilation(&self) -> &Compilation {
        &self.compilation
    }

    pub fn localizations(&self) -> Option<&Localizations> {
        self.localizations.as_ref()
    }

    pub fn default_dialogue_runner(&self) -> Result<DialogueRunner> {
        DialogueRunnerBuilder::from_yarn_project(self).build()
    }

    pub fn build_dialogue_runner(&self) -> DialogueRunnerBuilder {
        DialogueRunnerBuilder::from_yarn_project(self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect, FromReflect)]
#[reflect(Debug, PartialEq)]
pub struct LoadYarnProjectEvent {
    pub localizations: Option<Localizations>,
    pub yarn_files: HashSet<YarnFileSource>,
}

impl LoadYarnProjectEvent {
    #[must_use]
    pub fn with_yarn_files(yarn_files: Vec<impl Into<YarnFileSource>>) -> Self {
        let yarn_files = yarn_files
            .into_iter()
            .map(|yarn_file| yarn_file.into())
            .collect();
        Self {
            localizations: None,
            yarn_files,
        }
    }

    #[must_use]
    pub fn add_yarn_file(mut self, yarn_file: impl Into<YarnFileSource>) -> Self {
        self.yarn_files.insert(yarn_file.into());
        self
    }

    #[must_use]
    pub fn add_yarn_files(
        mut self,
        yarn_files: impl IntoIterator<Item = impl Into<YarnFileSource>>,
    ) -> Self {
        self.yarn_files
            .extend(yarn_files.into_iter().map(|yarn_file| yarn_file.into()));
        self
    }

    #[must_use]
    pub fn with_localizations(mut self, localizations: impl Into<Option<Localizations>>) -> Self {
        let localizations = localizations.into();
        assert_valid_cfg(localizations.as_ref());
        self.localizations = localizations;
        self
    }
}

pub(crate) fn assert_valid_cfg(localizations: Option<&Localizations>) {
    if let Some(localizations) = localizations {
        if cfg!(any(target_arch = "wasm32", target_os = "android")) {
            assert_ne!(localizations.file_generation_mode, FileGenerationMode::Development,
                           "Failed to build Yarn Slinger plugin: File generation mode \"Development\" is not supported on this target because it does not provide a access to the filesystem.");
        }
    }
}

impl<T, U> From<T> for LoadYarnProjectEvent
where
    T: IntoIterator<Item = U>,
    U: Into<YarnFileSource>,
{
    fn from(yarn_files: T) -> Self {
        Self::with_yarn_files(yarn_files.into_iter().collect())
    }
}
