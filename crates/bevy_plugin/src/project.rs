use crate::fmt_utils::SkipDebug;
use crate::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
pub(crate) use compilation::{
    RecompileLoadedYarnFilesEvent, YarnFilesBeingLoaded, YarnProjectConfigToLoad,
};
use std::fmt::Debug;
use std::iter;

mod compilation;

pub(crate) fn project_plugin(app: &mut App) {
    app.add_plugins(compilation::project_compilation_plugin)
        .add_message::<LoadYarnProjectEvent>();
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct CompilationSystemSet;

/// The compiled Yarn project built from the Yarn files passed to the [`YarnSpinnerPlugin`], or, in the deferred loading case, the Yarn files passed to the [`LoadYarnProjectEvent`].
/// This [`Resource`](bevy::prelude::Resource) is inserted into the world automatically for you once all files have been loaded and compiled. You can react to this by configuring a system like this:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_yarnspinner::prelude::*;
/// # let mut app = App::new();
/// app.add_systems(Update, setup_dialogue_runners.run_if(resource_added::<YarnProject>));
///
/// fn setup_dialogue_runners(mut commands: Commands, project: Res<YarnProject>) {
///    let dialogue_runner = project.create_dialogue_runner(&mut commands);
///    commands.spawn(dialogue_runner);
/// }
/// ```
#[derive(Resource, Debug)]
pub struct YarnProject {
    pub(crate) yarn_files: HashSet<Handle<YarnFile>>,
    pub(crate) compilation: Compilation,
    pub(crate) localizations: Option<Localizations>,
    pub(crate) asset_server: SkipDebug<AssetServer>,
    pub(crate) metadata: HashMap<LineId, Vec<String>>,
    pub(crate) watching_for_changes: bool,
    pub(crate) development_file_generation: DevelopmentFileGeneration,
}

impl YarnProject {
    /// Iterates over the [`YarnFile`]s that were used to compile this project. These will be the files passed to the [`YarnSpinnerPlugin`] or the [`LoadYarnProjectEvent`].
    pub fn yarn_files(&self) -> impl Iterator<Item = &Handle<YarnFile>> {
        self.yarn_files.iter()
    }

    /// Returns the underlying [`Compilation`] of this project. This is advanced functionality.
    pub fn compilation(&self) -> &Compilation {
        &self.compilation
    }

    /// Returns the [`Localizations`] of this project, if any. These come from [`YarnSpinnerPlugin::with_localizations`] or [`LoadYarnProjectEvent::with_localizations`].
    pub fn localizations(&self) -> Option<&Localizations> {
        self.localizations.as_ref()
    }

    /// Constructs a [`DialogueRunner`] from this project using all defaults of [`DialogueRunnerBuilder`] .
    /// This is a convenience method for calling [`DialogueRunnerBuilder::build`] on an unconfigured builder returned by [`YarnProject::build_dialogue_runner`].
    pub fn create_dialogue_runner(&self, commands: &mut Commands) -> DialogueRunner {
        self.build_dialogue_runner(commands).build()
    }

    /// Constructs a [`DialogueRunnerBuilder`] that can be used to customize the construction of a [`DialogueRunner`] from this project.
    pub fn build_dialogue_runner(&self, commands: &mut Commands) -> DialogueRunnerBuilder {
        DialogueRunnerBuilder::from_yarn_project(self, commands)
    }

    /// Returns the metadata associated with the given [`LineId`], if any. This can also be accessed on a given [`LocalizedLine`] via its `metadata` field.
    pub fn line_metadata(&self, line_id: &LineId) -> Option<&[String]> {
        self.metadata.get(line_id).map(|v| v.as_slice())
    }

    /// Returns the headers associated with the given node, if it exists.
    pub fn headers_for_node(&self, node_name: &str) -> Option<HashMap<&str, Vec<&str>>> {
        self.compilation
            .program
            .as_ref()
            .unwrap()
            .nodes
            .get(node_name)?
            .headers
            .iter()
            .fold(HashMap::default(), |mut map: HashMap<_, Vec<_>>, header| {
                map.entry(header.key.as_str())
                    .or_default()
                    .push(header.value.as_str());
                map
            })
            .into()
    }
}

/// Used to late initialize a [`YarnProject`] with a set of Yarn files when using [`YarnSpinnerPlugin::deferred`].
/// If you know the Yarn files at the start of the game, you should use [`YarnSpinnerPlugin::with_yarn_sources`] instead.
#[derive(Debug, Clone, PartialEq, Eq, Message)]
pub struct LoadYarnProjectEvent {
    pub(crate) localizations: Option<Localizations>,
    pub(crate) yarn_files: HashSet<YarnFileSource>,
    pub(crate) development_file_generation: DevelopmentFileGeneration,
}

#[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
impl Default for LoadYarnProjectEvent {
    fn default() -> Self {
        Self {
            localizations: None,
            yarn_files: HashSet::from_iter([YarnFileSource::Folder(DEFAULT_ASSET_DIR.into())]),
            development_file_generation: default(),
        }
    }
}

impl LoadYarnProjectEvent {
    /// See [`YarnSpinnerPlugin::new`]. Shares the same limitations regarding platform support.
    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// See [`YarnSpinnerPlugin::with_yarn_sources`].
    #[must_use]
    pub fn with_yarn_sources<T, U>(yarn_files: T) -> Self
    where
        T: IntoIterator<Item = U>,
        U: Into<YarnFileSource>,
    {
        let yarn_files = yarn_files
            .into_iter()
            .map(|yarn_file| yarn_file.into())
            .collect();
        Self {
            localizations: None,
            yarn_files,
            development_file_generation: default(),
        }
    }

    /// See [`YarnSpinnerPlugin::with_yarn_source`].
    #[must_use]
    pub fn with_yarn_source(yarn_file_source: impl Into<YarnFileSource>) -> Self {
        Self::with_yarn_sources(iter::once(yarn_file_source))
    }

    /// See [`YarnSpinnerPlugin::add_yarn_source`].
    #[must_use]
    pub fn add_yarn_source(mut self, yarn_file: impl Into<YarnFileSource>) -> Self {
        self.yarn_files.insert(yarn_file.into());
        self
    }

    /// See [`YarnSpinnerPlugin::add_yarn_sources`].
    #[must_use]
    pub fn add_yarn_sources(
        mut self,
        yarn_files: impl IntoIterator<Item = impl Into<YarnFileSource>>,
    ) -> Self {
        self.yarn_files
            .extend(yarn_files.into_iter().map(|yarn_file| yarn_file.into()));
        self
    }

    /// See [`YarnSpinnerPlugin::with_localizations`].
    #[must_use]
    pub fn with_localizations(mut self, localizations: impl Into<Option<Localizations>>) -> Self {
        let localizations = localizations.into();
        self.localizations = localizations;
        self
    }

    /// See [`YarnSpinnerPlugin::with_development_file_generation`].
    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    #[must_use]
    pub fn with_development_file_generation(
        mut self,
        development_file_generation: DevelopmentFileGeneration,
    ) -> Self {
        self.development_file_generation = development_file_generation;
        self
    }
}

impl<T, U> From<T> for LoadYarnProjectEvent
where
    T: IntoIterator<Item = U>,
    U: Into<YarnFileSource>,
{
    fn from(yarn_files: T) -> Self {
        Self::with_yarn_sources(yarn_files)
    }
}

#[derive(Debug, Clone, Resource, Default)]
pub(crate) struct WatchingForChanges(pub(crate) bool);

pub(crate) const DEFAULT_ASSET_DIR: &str = "dialogue";
