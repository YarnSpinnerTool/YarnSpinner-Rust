use crate::prelude::*;
use crate::project::{LoadYarnProjectEvent, WatchingForChanges};
use bevy::prelude::*;
use std::path::PathBuf;
pub use yarn_file_source::YarnFileSource;

mod yarn_file_source;

/// The plugin that provides all Yarn Spinner functionality.
/// In general, you'll want to create this by searching for Yarn files in "assets/dialogue", which [`YarnSpinnerPlugin::new`] does under the hood.
/// You can also provide a list of Yarn files to load via [`YarnSpinnerPlugin::with_yarn_sources`].
/// If you however do not know the paths to any files nor have them in-memory at the start of the program,
/// use [`YarnSpinnerPlugin::deferred`] instead to later load the files by sending a [`LoadYarnProjectEvent`].
///
/// Needs to be added after the [`AssetPlugin`] which is usually added as part of the [`DefaultPlugins`].
///
/// ## Example
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_yarnspinner::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     // Load all Yarn files from the "assets/dialogue" folder by default.
///     .add_plugins(YarnSpinnerPlugin::new());
/// ```
///
/// Note that the above does not work on Wasm or Android, since Bevy cannot query folders on these platforms. See [`YarnSpinnerPlugin::new`] for more information.
///
/// For more information on how this plugin interacts with the rest of the crate, see the crate-level documentation.
#[derive(Debug, Default)]
pub struct YarnSpinnerPlugin {
    project: LoadYarnProjectEvent,
}

/// The [`SystemSet`] containing all systems used by the [`YarnSpinnerPlugin`].
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSpinnerSystemSet;

impl YarnSpinnerPlugin {
    /// Creates a new plugin that loads Yarn files from the folder "assets/dialogue" when not on Wasm or Android.
    /// Otherwise this panics since Bevy cannot query folders on these platforms.
    /// Use [`YarnSpinnerPlugin::with_yarn_source`] or [`YarnSpinnerPlugin::with_yarn_sources`] there instead.
    ///
    /// All Yarn files will be shared across [`DialogueRunner`]s.
    /// If hot reloading is turned on via the \"file_watcher\" feature of Bevy,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// Calling this is equivalent to calling [`YarnSpinnerPlugin::with_yarn_source`] with a [`YarnFileSource::folder`] of `"dialogue"`.
    #[must_use]
    pub fn new() -> Self {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::default()
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            panic!(
                "YarnSpinnerPlugin::new() is not supported on this platform because it tries to load files from the \"dialogue\" directory in the assets folder. \
                However, this platform does not allow loading a file without naming it explicitly. \
                Use `YarnSpinnerPlugin::with_yarn_source` or `YarnSpinnerPlugin::with_yarn_sources` instead.")
        }
    }

    /// Creates a new plugin that loads Yarn files from the given sources.
    /// All Yarn files will be shared across [`DialogueRunner`]s.
    /// If hot reloading is turned on via the \"file_watcher\" feature of Bevy,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// See [`YarnFileSource`] for more information on where Yarn files can be loaded from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_yarnspinner::prelude::*;
    /// let plugin = YarnSpinnerPlugin::with_yarn_sources([
    ///    YarnFileSource::file("some_dialogue.yarn"),
    ///    YarnFileSource::file("some_other_dialogue.yarn"),
    /// ]);
    /// ```
    #[must_use]
    pub fn with_yarn_sources<T, U>(yarn_files: T) -> Self
    where
        T: IntoIterator<Item = U>,
        U: Into<YarnFileSource>,
    {
        Self {
            project: LoadYarnProjectEvent::with_yarn_sources(yarn_files),
        }
    }

    /// Creates a new plugin that loads Yarn files from the given source.
    /// All Yarn files will be shared across [`DialogueRunner`]s.
    /// If hot reloading is turned on via the \"file_watcher\" feature of Bevy,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// See [`YarnFileSource`] for more information on where Yarn files can be loaded from.
    ///
    /// Calling this with [`YarnFileSource::folder`] and passing `"dialogue"` is equivalent to calling [`YarnSpinnerPlugin::new`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_yarnspinner::prelude::*;
    /// let plugin = YarnSpinnerPlugin::with_yarn_source(YarnFileSource::folder("yarn_files"));
    /// ```
    #[must_use]
    pub fn with_yarn_source(yarn_file_source: impl Into<YarnFileSource>) -> Self {
        Self {
            project: LoadYarnProjectEvent::with_yarn_source(yarn_file_source),
        }
    }

    /// Creates a version of the plugin that does not load anything yet and instead waits until you have sent a [`LoadYarnProjectEvent`].
    #[must_use]
    pub fn deferred() -> DeferredYarnSpinnerPlugin {
        DeferredYarnSpinnerPlugin
    }

    /// Adds a Yarn file source to the files that will be loaded and compiled.
    #[must_use]
    pub fn add_yarn_source(mut self, yarn_file: impl Into<YarnFileSource>) -> Self {
        self.project = self.project.add_yarn_source(yarn_file);
        self
    }

    /// Adds multiple Yarn file source to the files that will be loaded and compiled.
    #[must_use]
    pub fn add_yarn_sources(
        mut self,
        yarn_files: impl IntoIterator<Item = impl Into<YarnFileSource>>,
    ) -> Self {
        self.project = self.project.add_yarn_sources(yarn_files);
        self
    }

    /// Sets supported localizations. See [`Localizations`] for more information about the format.
    /// By default, no localizations are used.
    #[must_use]
    pub fn with_localizations(mut self, localizations: impl Into<Option<Localizations>>) -> Self {
        self.project = self.project.with_localizations(localizations);
        self
    }

    /// Sets the development file generation mode, which determines how aggressively Yarn Spinner will generate files that aid in development.
    /// Defaults to [`DevelopmentFileGeneration::TRY_FULL`] in debug builds, [`DevelopmentFileGeneration::None`] otherwise.
    #[must_use]
    pub fn with_development_file_generation(
        mut self,
        development_file_generation: DevelopmentFileGeneration,
    ) -> Self {
        self.project = self
            .project
            .with_development_file_generation(development_file_generation);
        self
    }
}

impl Plugin for YarnSpinnerPlugin {
    fn build(&self, app: &mut App) {
        assert!(!self.project.yarn_files.is_empty(), "Cannot initialize Yarn Spinner plugin because no Yarn files were specified. \
        Did you call `YarnSpinnerPlugin::with_yarn_files()` without any Yarn file sources? \
        If you really want to load no Yarn files right now and do that later, use `YarnSpinnerPlugin::deferred()` instead.\
        If you wanted to load from the default directory instead, use `YarnSpinnerPlugin::default()`.");
        app.add_plugins(Self::deferred())
            .world
            .send_event(self.project.clone());
    }
}

/// The deferred version of [`YarnSpinnerPlugin`]. Created by [`YarnSpinnerPlugin::deferred`].
/// Will not load any Yarn files until a [`LoadYarnProjectEvent`] is sent.
#[derive(Debug)]
#[non_exhaustive]
pub struct DeferredYarnSpinnerPlugin;

impl Plugin for DeferredYarnSpinnerPlugin {
    fn build(&self, app: &mut App) {
        app.register_yarn_types()
            .register_sub_plugins()
            .register_watching_for_changes()
            .register_asset_root();
    }
}

trait YarnApp {
    fn register_yarn_types(&mut self) -> &mut Self;
    fn register_sub_plugins(&mut self) -> &mut Self;
    fn register_watching_for_changes(&mut self) -> &mut Self;
    fn register_asset_root(&mut self) -> &mut Self;
}
impl YarnApp for App {
    fn register_yarn_types(&mut self) -> &mut Self {
        self.register_type::<YarnCompiler>()
            .register_type::<YarnFile>()
            .register_type::<CompilationType>()
            .register_type::<Compilation>()
            .register_type::<CompilerError>()
            .register_type::<yarnspinner::compiler::Diagnostic>()
            .register_type::<yarnspinner::compiler::DiagnosticSeverity>()
            .register_type::<yarnspinner::compiler::DebugInfo>()
            .register_type::<LineInfo>()
            .register_type::<yarnspinner::compiler::Declaration>()
            .register_type::<yarnspinner::compiler::DeclarationSource>()
            .register_type::<StringInfo>()
            .register_type::<LineId>()
            .register_type::<yarnspinner::core::Position>()
            .register_type::<YarnValue>()
            .register_type::<yarnspinner::core::InvalidOpCodeError>()
            .register_type::<yarnspinner::core::Program>()
            .register_type::<yarnspinner::core::Node>()
            .register_type::<yarnspinner::core::Header>()
            .register_type::<yarnspinner::core::Instruction>()
            .register_type::<yarnspinner::core::Type>()
            .register_type::<yarnspinner::runtime::Command>()
            .register_type::<yarnspinner::prelude::DialogueOption>()
            .register_type::<OptionId>()
            .register_type::<DialogueEvent>()
            .register_type::<yarnspinner::runtime::Line>()
            .register_type::<yarnspinner::runtime::Diagnosis>()
            .register_type::<yarnspinner::runtime::DiagnosisSeverity>()
            .register_type::<yarnspinner::runtime::MarkupParseError>()
            .register_type::<MarkupAttribute>()
            .register_type::<MarkupValue>()
    }

    fn register_sub_plugins(&mut self) -> &mut Self {
        self.add_plugins(crate::yarn_file_asset::yarnspinner_asset_loader_plugin)
            .add_plugins(crate::localization::localization_plugin)
            .add_plugins(crate::dialogue_runner::dialogue_plugin)
            .add_plugins(crate::line_provider::line_provider_plugin)
            .add_plugins(crate::project::project_plugin)
            .add_plugins(crate::commands::commands_plugin)
            .add_plugins(crate::development_file_generation::development_file_generation_plugin)
    }

    fn register_watching_for_changes(&mut self) -> &mut Self {
        let asset_server = self.world.get_resource::<AssetServer>().expect(ASSET_ERROR);

        let watching_for_changes = asset_server.watching_for_changes();
        self.insert_resource(WatchingForChanges(watching_for_changes))
    }

    fn register_asset_root(&mut self) -> &mut Self {
        let asset_plugin = get_asset_plugin(self);
        let path_str = asset_plugin.file_path.clone();
        let path = PathBuf::from(path_str);
        self.insert_resource(AssetRoot(path))
    }
}

fn get_asset_plugin(app: &App) -> &AssetPlugin {
    let asset_plugins: Vec<&AssetPlugin> = app.get_added_plugins();
    asset_plugins.into_iter().next().expect(ASSET_ERROR)
}

const ASSET_ERROR: &str = "Yarn Spinner requires access to the Bevy asset plugin. \
    Please add `YarnSpinnerPlugin` after `AssetPlugin`, which is commonly added as part of the `DefaultPlugins`";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub(crate) struct AssetRoot(pub(crate) PathBuf);
