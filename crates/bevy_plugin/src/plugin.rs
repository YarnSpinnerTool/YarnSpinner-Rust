use crate::default_impl::{MemoryVariableStore, StringTableTextProvider};
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

mod yarn_file_source;
use crate::project::YarnFilesToLoad;
pub use yarn_file_source::YarnFileSource;

#[derive(Debug)]
#[non_exhaustive]
pub struct YarnSlingerPlugin {
    pub localizations: Option<Localizations>,
    pub line_asset_provider: Option<Box<dyn LineAssetProvider>>,
    pub yarn_files: HashSet<YarnFileSource>,
    pub advanced: AdvancedPluginConfig,
}

impl YarnSlingerPlugin {
    pub fn with_localizations(localizations: impl Into<Option<Localizations>>) -> Self {
        let localizations = localizations.into();
        if let Some(localizations) = localizations.as_ref() {
            if cfg!(target_arch = "wasm32") {
                assert_ne!(localizations.file_generation_mode, FileGenerationMode::Development,
                           "Failed to build Yarn Slinger plugin: File generation mode \"Development\" is not supported on Wasm because this target does not provide a access to the filesystem.");
            }
        }

        Self {
            localizations,
            advanced: Default::default(),
            line_asset_provider: None,
            yarn_files: HashSet::default(),
        }
    }

    pub fn with_yarn_files(mut self, yarn_files: Vec<impl Into<YarnFileSource>>) -> Self {
        let yarn_files = yarn_files.into_iter().map(|yarn_file| yarn_file.into());
        self.yarn_files.extend(yarn_files);
        self
    }

    pub fn with_asset_provider(
        mut self,
        asset_provider: impl Into<Option<Box<dyn LineAssetProvider>>>,
    ) -> Self {
        self.line_asset_provider = asset_provider.into();
        self
    }

    pub fn advanced(
        mut self,
        config: impl Fn(AdvancedPluginConfig) -> AdvancedPluginConfig,
    ) -> Self {
        self.advanced = config(self.advanced);
        self
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub struct AdvancedPluginConfig {
    pub variable_storage: Box<dyn VariableStorage>,
    pub text_provider: Box<dyn TextProvider>,
}

impl Default for AdvancedPluginConfig {
    fn default() -> Self {
        Self {
            variable_storage: Box::new(MemoryVariableStore::default()),
            text_provider: Box::new(StringTableTextProvider::default()),
        }
    }
}

impl AdvancedPluginConfig {
    pub fn with_variable_storage(mut self, variable_storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage = variable_storage;
        self
    }

    pub fn with_text_provider(mut self, text_provider: Box<dyn TextProvider>) -> Self {
        self.text_provider = text_provider;
        self
    }
}

impl Plugin for YarnSlingerPlugin {
    fn build(&self, app: &mut App) {
        app.register_yarn_types()
            .init_resources(self)
            .register_sub_plugins();
    }
}

trait YarnApp {
    fn register_yarn_types(&mut self) -> &mut Self;
    fn init_resources(&mut self, plugin: &YarnSlingerPlugin) -> &mut Self;
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
            .register_type::<DialogueOption>()
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

    fn init_resources(&mut self, plugin: &YarnSlingerPlugin) -> &mut Self {
        if let Some(localizations) = plugin.localizations.clone() {
            self.insert_resource(CurrentLanguage(
                localizations.base_language.language.clone(),
            ))
            .insert_resource(localizations);
        }
        if let Some(line_asset_provider) = &plugin.line_asset_provider {
            self.insert_resource(GlobalLineAssetProvider(line_asset_provider.clone_shallow()));
        }
        self.insert_resource(YarnFilesToLoad(plugin.yarn_files.clone()))
            .insert_resource(GlobalTextProvider(
                plugin.advanced.text_provider.clone_shallow(),
            ))
            .insert_resource(GlobalVariableStorage(
                plugin.advanced.variable_storage.clone_shallow(),
            ))
    }

    fn register_sub_plugins(&mut self) -> &mut Self {
        self.fn_plugin(crate::yarn_file_asset::yarn_slinger_asset_loader_plugin)
            .fn_plugin(crate::localization::localization_plugin)
            .fn_plugin(crate::dialogue_runner::dialogue_plugin)
            .fn_plugin(crate::line_provider::line_provider_plugin)
            .fn_plugin(crate::project::project_plugin)
    }
}
