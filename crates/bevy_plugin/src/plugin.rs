use crate::prelude::*;
use crate::project::LoadYarnProjectEvent;
use bevy::prelude::*;
pub use yarn_file_source::YarnFileSource;

mod yarn_file_source;

#[derive(Debug, Default)]
pub struct YarnSlingerPlugin {
    project: LoadYarnProjectEvent,
}

impl YarnSlingerPlugin {
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

    #[must_use]
    pub fn add_yarn_file(mut self, yarn_file: impl Into<YarnFileSource>) -> Self {
        self.project = self.project.add_yarn_file(yarn_file);
        self
    }

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
        if let Some(localizations) = localizations.as_ref() {
            if cfg!(target_arch = "wasm32") {
                assert_ne!(localizations.file_generation_mode, FileGenerationMode::Development,
                           "Failed to build Yarn Slinger plugin: File generation mode \"Development\" is not supported on Wasm because this target does not provide a access to the filesystem.");
            }
        }
        self.project.localizations = localizations;
        self
    }
}

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.register_yarn_types()
            .register_sub_plugins()
            .world
            .send_event(self.project.clone());
    }
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct DeferredYarnSlingerPlugin;

impl DeferredYarnSlingerPlugin {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl Plugin for DeferredYarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.register_yarn_types().register_sub_plugins();
    }
}

trait YarnApp {
    fn register_yarn_types(&mut self) -> &mut Self;
    fn register_sub_plugins(&mut self) -> &mut Self;
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
}
