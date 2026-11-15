use crate::default_impl::{MemoryVariableStorage, StringsFileTextProvider};
use crate::fmt_utils::SkipDebug;
use crate::line_provider::SharedTextProvider;
use crate::prelude::*;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::fmt::Debug;
use yarnspinner::core::CompiledFile;

pub(crate) fn dialogue_runner_builder_plugin(_app: &mut App) {}

/// A builder for [`DialogueRunner`]. This is instantiated for you by calling [`YarnProject::build_dialogue_runner`].
#[derive(Debug)]
pub struct DialogueRunnerBuilder {
    variable_storage: Box<dyn VariableStorage>,
    text_provider: SharedTextProvider,
    asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    library: YarnLibrary,
    commands: YarnCommands,
    compilation: CompiledFile,
    localizations: Option<Localizations>,
    asset_server: SkipDebug<AssetServer>,
}

impl DialogueRunnerBuilder {
    #[must_use]
    pub(crate) fn from_yarn_project(yarn_project: &YarnProject, commands: &mut Commands) -> Self {
        Self {
            variable_storage: Box::new(MemoryVariableStorage::new()),
            text_provider: SharedTextProvider::new(StringsFileTextProvider::from_yarn_project(
                yarn_project,
            )),
            asset_providers: HashMap::default(),
            library: YarnLibrary::standard_library(),
            commands: YarnCommands::builtin_commands(commands),
            compilation: yarn_project.compiled_file().clone(),
            localizations: yarn_project.localizations().cloned(),
            asset_server: yarn_project.asset_server.clone(),
        }
    }

    /// Replaces the [`VariableStorage`] used by the [`DialogueRunner`]. By default, this is a [`MemoryVariableStorage`].
    #[must_use]
    pub fn with_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage = storage;
        self
    }

    /// Replaces the [`TextProvider`] used by the [`DialogueRunner`]. By default, this is a [`StringsFileTextProvider`].
    #[must_use]
    pub fn with_text_provider(mut self, provider: impl TextProvider + 'static) -> Self {
        self.text_provider.replace(provider);
        self
    }

    /// Adds an [`AssetProvider`] to the [`DialogueRunner`]. By default, none are registered.
    #[must_use]
    pub fn add_asset_provider(mut self, provider: impl AssetProvider + 'static) -> Self {
        self.asset_providers
            .insert(provider.type_id(), Box::new(provider));
        self
    }

    /// Builds the [`DialogueRunner`]. See [`DialogueRunnerBuilder::try_build`] for the fallible version.
    pub fn build(self) -> DialogueRunner {
        self.try_build().unwrap_or_else(|error| {
            panic!("Failed to build DialogueRunner: {error}");
        })
    }

    /// Builds the [`DialogueRunner`].
    pub fn try_build(mut self) -> Result<DialogueRunner> {
        let text_provider = Box::new(self.text_provider);

        let mut dialogue = Dialogue::new(self.variable_storage, text_provider.clone());
        dialogue
            .set_line_hints_enabled(true)
            .library_mut()
            .extend(self.library);
        dialogue.add_program(self.compilation.program);

        for asset_provider in self.asset_providers.values_mut() {
            if let Some(ref localizations) = self.localizations {
                asset_provider.set_localizations(localizations.clone());
            }

            asset_provider.set_asset_server(self.asset_server.0.clone());
        }

        let popped_line_hints = dialogue.pop_line_hints();

        let base_language = self
            .localizations
            .as_ref()
            .map(|l| &l.base_localization.language)
            .cloned();

        let mut dialogue_runner = DialogueRunner {
            dialogue: Some(dialogue),
            text_provider,
            popped_line_hints,
            run_selected_options_as_lines: false,
            asset_providers: self.asset_providers,
            commands: self.commands,
            is_running: default(),
            command_tasks: default(),
            will_continue_in_next_update: default(),
            last_selected_option: default(),
            just_started: default(),
            unsent_events: default(),
            localizations: self.localizations,
        };

        if let Some(base_language) = base_language {
            dialogue_runner.set_language(base_language);
        }

        Ok(dialogue_runner)
    }
}
