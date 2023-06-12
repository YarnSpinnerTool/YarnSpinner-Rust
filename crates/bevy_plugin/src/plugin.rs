use crate::prelude::*;
use crate::project::{assert_valid_cfg, LoadYarnProjectEvent, WatchingForChanges};
use bevy::prelude::*;
pub use yarn_file_source::YarnFileSource;

mod yarn_file_source;

#[derive(Debug)]
pub struct YarnSlingerPlugin {
    project: LoadYarnProjectEvent,
}

#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSlingerSystemSet;

impl YarnSlingerPlugin {
    /// Creates a new plugin that loads the given yarn files.
    /// All yarn files will be shared across [`DialogueRunner`]s.
    /// If [hot reloading](https://bevy-cheatbook.github.io/assets/hot-reload.html) is turned on, these yarn files will be recompiled if they change during runtime.
    ///
    /// The files can be provided in any of the following ways:
    /// - As a path to the asset (this is the most common usage)
    /// - As a handle to the asset
    /// - In memory as a [`YarnFile`]
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_yarn_slinger::prelude::*;
    /// let plugin = YarnSlingerPlugin::with_yarn_files(vec![
    ///    "some_dialogue.yarn",
    ///    "some_other_dialogue.yarn",
    /// ]);
    /// ```
    #[must_use]
    pub fn with_yarn_files(yarn_files: Vec<impl Into<YarnFileSource>>) -> Self {
        let yarn_files = yarn_files
            .into_iter()
            .map(|yarn_file| yarn_file.into())
            .collect();
        Self {
            project: LoadYarnProjectEvent {
                localizations: None,
                yarn_files,
            },
        }
    }

    /// Creates a version of the plugin that does not load anything yet and instead waits until you have sent a [`LoadYarnProjectEvent`].
    #[must_use]
    pub fn deferred() -> DeferredYarnSlingerPlugin {
        DeferredYarnSlingerPlugin
    }

    /// Adds a yarn file to the list of yarn files provided in [`YarnSlingerPlugin::with_yarn_files`].
    #[must_use]
    pub fn add_yarn_file(mut self, yarn_file: impl Into<YarnFileSource>) -> Self {
        self.project = self.project.add_yarn_file(yarn_file);
        self
    }

    /// Adds further files to the list of yarn files provided in [`YarnSlingerPlugin::with_yarn_files`].
    #[must_use]
    pub fn add_yarn_files(
        mut self,
        yarn_files: impl IntoIterator<Item = impl Into<YarnFileSource>>,
    ) -> Self {
        self.project = self.project.add_yarn_files(yarn_files);
        self
    }

    #[must_use]
    pub fn with_localizations(mut self, localizations: impl Into<Option<Localizations>>) -> Self {
        let localizations = localizations.into();
        assert_valid_cfg(localizations.as_ref());
        self.project.localizations = localizations;
        self
    }
}

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Self::deferred())
            .world
            .send_event(self.project.clone());
    }
}

/// The deferred version of [`YarnSlingerPlugin`]. Created by [`YarnSlingerPlugin::deferred`].
/// Will not load any yarn files until a [`LoadYarnProjectEvent`] is sent.
#[derive(Debug)]
#[non_exhaustive]
pub struct DeferredYarnSlingerPlugin;

impl Plugin for DeferredYarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        let watching = app.is_watching_for_changes();
        app.register_yarn_types()
            .register_sub_plugins()
            .insert_resource(WatchingForChanges(watching));
    }
}

trait YarnApp {
    fn register_yarn_types(&mut self) -> &mut Self;
    fn register_sub_plugins(&mut self) -> &mut Self;
    fn is_watching_for_changes(&self) -> bool;
}
impl YarnApp for App {
    fn register_yarn_types(&mut self) -> &mut Self {
        self.register_type::<YarnCompiler>()
            .register_type::<YarnFile>()
            .register_type::<CompilationType>()
            .register_type::<Compilation>()
            .register_type::<CompilerError>()
            .register_type::<yarn_slinger::compiler::Diagnostic>()
            .register_type::<yarn_slinger::compiler::DiagnosticSeverity>()
            .register_type::<yarn_slinger::compiler::DebugInfo>()
            .register_type::<LineInfo>()
            .register_type::<yarn_slinger::compiler::Declaration>()
            .register_type::<yarn_slinger::compiler::DeclarationSource>()
            .register_type::<StringInfo>()
            .register_type::<LineId>()
            .register_type::<yarn_slinger::core::Position>()
            .register_type::<YarnValue>()
            .register_type::<yarn_slinger::core::InvalidOpCodeError>()
            .register_type::<yarn_slinger::core::Program>()
            .register_type::<yarn_slinger::core::Node>()
            .register_type::<yarn_slinger::core::Header>()
            .register_type::<yarn_slinger::core::Instruction>()
            .register_type::<yarn_slinger::core::Type>()
            .register_type::<yarn_slinger::runtime::Command>()
            .register_type::<Dialogue>()
            .register_type::<yarn_slinger::prelude::DialogueOption>()
            .register_type::<OptionId>()
            .register_type::<Language>()
            .register_type::<DialogueEvent>()
            .register_type::<yarn_slinger::runtime::Line>()
            .register_type::<yarn_slinger::runtime::Diagnosis>()
            .register_type::<yarn_slinger::runtime::DiagnosisSeverity>()
            .register_type::<yarn_slinger::runtime::MarkupParseError>()
            .register_type::<MarkupAttribute>()
            .register_type::<MarkupValue>()
    }

    fn register_sub_plugins(&mut self) -> &mut Self {
        self.fn_plugin(crate::yarn_file_asset::yarn_slinger_asset_loader_plugin)
            .fn_plugin(crate::localization::localization_plugin)
            .fn_plugin(crate::dialogue_runner::dialogue_plugin)
            .fn_plugin(crate::line_provider::line_provider_plugin)
            .fn_plugin(crate::project::project_plugin)
            .fn_plugin(crate::commands::commands_plugin)
    }

    fn is_watching_for_changes(&self) -> bool {
        let asset_plugins: Vec<&AssetPlugin> = self.get_added_plugins();
        let asset_plugin: &AssetPlugin = asset_plugins.into_iter().next().expect("Yarn Slinger requires access to the AssetPlugin. \
        Please add the YarnSlingerPlugin after the AssetPlugin, which is commonly located in the DefaultPlugins");
        asset_plugin.watch_for_changes
    }
}
