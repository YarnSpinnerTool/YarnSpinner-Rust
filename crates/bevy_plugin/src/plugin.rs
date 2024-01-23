use crate::prelude::*;
use crate::project::{LoadYarnProjectEvent, WatchingForChanges};
use bevy::prelude::*;
pub use yarn_file_source::YarnFileSource;

mod yarn_file_source;

/// The plugin that provides all Yarn Slinger functionality.
/// In general, you'll want to create this by searching for Yarn files in "assets/dialogue", which [`YarnSlingerPlugin::new`] does under the hood.
/// You can also provide a list of Yarn files to load via [`YarnSlingerPlugin::with_yarn_sources`].
/// If you however do not know the paths to any files nor have them in-memory at the start of the program,
/// use [`YarnSlingerPlugin::deferred`] instead to later load the files by sending a [`LoadYarnProjectEvent`].
///
/// Needs to be added after the [`AssetPlugin`] which is usually added as part of the [`DefaultPlugins`].
///
/// ## Example
///
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_yarn_slinger::prelude::*;
///
/// App::new()
///     .add_plugins(DefaultPlugins)
///     // Load all Yarn files from the "assets/dialogue" folder by default.
///     .add_plugins(YarnSlingerPlugin::new());
/// ```
///
/// Note that the above does not work on Wasm or Android, since Bevy cannot query folders on these platforms. See [`YarnSlingerPlugin::new`] for more information.
///
/// For more information on how this plugin interacts with the rest of the crate, see the crate-level documentation.
#[derive(Debug, Default)]
pub struct YarnSlingerPlugin {
    project: LoadYarnProjectEvent,
}

/// The [`SystemSet`] containing all systems used by the [`YarnSlingerPlugin`].
#[derive(Debug, Default, Clone, Copy, SystemSet, Eq, PartialEq, Hash)]
pub struct YarnSlingerSystemSet;

impl YarnSlingerPlugin {
    /// Creates a new plugin that loads Yarn files from the folder "assets/dialogue" when not on Wasm or Android.
    /// Otherwise this panics since Bevy cannot query folders on these platforms.
    /// Use [`YarnSlingerPlugin::with_yarn_source`] or [`YarnSlingerPlugin::with_yarn_sources`] there instead.
    ///
    /// All Yarn files will be shared across [`DialogueRunner`]s.
    /// If [hot reloading](https://bevy-cheatbook.github.io/assets/hot-reload.html) is turned on,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// Calling this is equivalent to calling [`YarnSlingerPlugin::with_yarn_source`] with a [`YarnFileSource::folder`] of `"dialogue"`.
    #[must_use]
    pub fn new() -> Self {
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            Self::default()
        }
        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            panic!(
                "YarnSlingerPlugin::new() is not supported on this platform because it tries to load files from the \"dialogue\" directory in the assets folder. \
                However, this platform does not allow loading a file without naming it explicitly. \
                Use `YarnSlingerPlugin::with_yarn_source` or `YarnSlingerPlugin::with_yarn_sources` instead.")
        }
    }

    /// Creates a new plugin that loads Yarn files from the given sources.
    /// All Yarn files will be shared across [`DialogueRunner`]s.
    /// If [hot reloading](https://bevy-cheatbook.github.io/assets/hot-reload.html) is turned on,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// See [`YarnFileSource`] for more information on where Yarn files can be loaded from.
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_yarn_slinger::prelude::*;
    /// let plugin = YarnSlingerPlugin::with_yarn_sources([
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
    /// If [hot reloading](https://bevy-cheatbook.github.io/assets/hot-reload.html) is turned on,
    /// these Yarn files will be recompiled if they change during runtime.
    ///
    /// See [`YarnFileSource`] for more information on where Yarn files can be loaded from.
    ///
    /// Calling this with [`YarnFileSource::folder`] and passing `"dialogue"` is equivalent to calling [`YarnSlingerPlugin::new`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use bevy_yarn_slinger::prelude::*;
    /// let plugin = YarnSlingerPlugin::with_yarn_source(YarnFileSource::folder("yarn_files"));
    /// ```
    #[must_use]
    pub fn with_yarn_source(yarn_file_source: impl Into<YarnFileSource>) -> Self {
        Self {
            project: LoadYarnProjectEvent::with_yarn_source(yarn_file_source),
        }
    }

    /// Creates a version of the plugin that does not load anything yet and instead waits until you have sent a [`LoadYarnProjectEvent`].
    #[must_use]
    pub fn deferred() -> DeferredYarnSlingerPlugin {
        DeferredYarnSlingerPlugin
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

    /// Sets the development file generation mode, which determines how aggressively Yarn Slinger will generate files that aid in development.
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

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        assert!(!self.project.yarn_files.is_empty(), "Cannot initialize Yarn Slinger plugin because no Yarn files were specified. \
        Did you call `YarnSlingerPlugin::with_yarn_files()` without any Yarn file sources? \
        If you really want to load no Yarn files right now and do that later, use `YarnSlingerPlugin::deferred()` instead.\
        If you wanted to load from the default directory instead, use `YarnSlingerPlugin::default()`.");
        app.add_plugins(Self::deferred())
            .world
            .send_event(self.project.clone());
    }
}

/// The deferred version of [`YarnSlingerPlugin`]. Created by [`YarnSlingerPlugin::deferred`].
/// Will not load any Yarn files until a [`LoadYarnProjectEvent`] is sent.
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
            .register_type::<yarn_slinger::prelude::DialogueOption>()
            .register_type::<OptionId>()
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
            .fn_plugin(crate::development_file_generation::development_file_generation_plugin)
    }

    fn is_watching_for_changes(&self) -> bool {
        let on_by_default = cfg!(all(not(target_arch = "wasm32"), not(target_os = "android")));

        let asset_plugins: Vec<&AssetPlugin> = self.get_added_plugins();
        let asset_plugin: &AssetPlugin = asset_plugins.into_iter().next().expect("Yarn Slinger requires access to the Bevy asset plugin. \
        Please add `YarnSlingerPlugin` after `AssetPlugin`, which is commonly added as part of the `DefaultPlugins`");

        asset_plugin
            .watch_for_changes_override
            .unwrap_or(on_by_default)
    }
}
