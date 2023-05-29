pub use self::events::{
    DialogueCompleteEvent, ExecuteCommandEvent, LineHintsEvent, NodeCompleteEvent, NodeStartEvent,
    PresentLineEvent, PresentOptionsEvent,
};
pub use self::{dialogue_option::DialogueOption, localized_line::LocalizedLine};
use crate::default_impl::MemoryVariableStore;
use crate::line_provider::{LineAssets, SharedTextProvider, StringsFileTextProvider};
use crate::prelude::*;
use crate::{UnderlyingTextProvider, UnderlyingYarnLine};
use bevy::prelude::*;
use bevy::tasks::Task;
use bevy::utils::HashMap;
pub(crate) use runtime_interaction::DialogueExecutionSystemSet;
use std::any::{Any, TypeId};
use std::fmt;
use std::fmt::{Debug, Formatter};

mod dialogue_option;
mod events;
mod localized_line;
mod runtime_interaction;

pub(crate) fn dialogue_plugin(app: &mut App) {
    app.fn_plugin(runtime_interaction::runtime_interaction_plugin)
        .fn_plugin(localized_line::localized_line_plugin)
        .fn_plugin(events::dialogue_runner_events_plugin)
        .fn_plugin(dialogue_option::dialogue_option_plugin);
}

#[derive(Debug, Component)]
pub struct DialogueRunner {
    pub(crate) dialogue: Dialogue,
    text_provider: Box<dyn TextProvider>,
    asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    continue_: bool,
    pub(crate) last_selected_option: Option<OptionId>,
    pub(crate) commands: YarnCommandRegistrations,
    command_tasks: Vec<Task<()>>,
    config: DialogueRunnerConfig,
}

#[derive(Debug)]
pub enum StartNode {
    DefaultStartNode,
    Node(String),
}

impl DialogueRunner {
    pub const DEFAULT_START_NODE_NAME: &'static str = Dialogue::DEFAULT_START_NODE_NAME;

    pub fn continue_in_next_update(&mut self) -> &mut Self {
        self.continue_ = true;
        self
    }

    #[must_use]
    pub fn will_continue_in_next_update(&self) -> bool {
        self.continue_
    }

    pub fn select_option(&mut self, option: OptionId) -> Result<&mut Self> {
        self.last_selected_option.replace(option);
        self.dialogue
            .set_selected_option(option)
            .map_err(Error::from)?;
        Ok(self)
    }

    pub fn set_node(&mut self, name: impl Into<String>) -> Result<&mut Self> {
        self.dialogue.set_node(name)?;
        Ok(self)
    }

    pub fn set_node_to_start(&mut self) -> Result<&mut Self> {
        self.dialogue.set_node_to_start()?;
        Ok(self)
    }

    #[must_use]
    pub fn node_names(&self) -> Vec<String> {
        self.dialogue.node_names().unwrap()
    }

    #[must_use]
    pub fn get_line_id_for_node(&self, node_name: &str) -> Option<LineId> {
        self.dialogue.get_line_id_for_node(node_name)
    }

    #[must_use]
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.dialogue.get_tags_for_node(node_name)
    }

    #[must_use]
    pub fn node_exists(&self, node_name: &str) -> bool {
        self.dialogue.node_exists(node_name)
    }

    #[must_use]
    pub fn current_node(&self) -> Option<String> {
        self.dialogue.current_node()
    }

    #[must_use]
    pub fn analyse(&self, context: &mut YarnAnalysisContext) -> &Self {
        self.dialogue.analyse(context);
        self
    }

    #[must_use]
    pub fn config(&self) -> &DialogueRunnerConfig {
        &self.config
    }

    #[must_use]
    pub fn config_mut(&mut self) -> &mut DialogueRunnerConfig {
        &mut self.config
    }

    #[must_use]
    pub fn data_providers(&self) -> DialogueRunnerDataProviders {
        DialogueRunnerDataProviders(self)
    }

    #[must_use]
    pub fn data_providers_mut(&mut self) -> DialogueRunnerDataProvidersMut {
        DialogueRunnerDataProvidersMut(self)
    }

    #[must_use]
    pub fn are_texts_available(&self) -> bool {
        self.text_provider.are_lines_available()
    }

    #[must_use]
    pub fn are_assets_available(&self) -> bool {
        self.asset_providers
            .values()
            .all(|provider| provider.are_assets_available())
    }

    #[must_use]
    pub fn are_lines_available(&self) -> bool {
        self.are_texts_available() && self.are_assets_available()
    }

    pub fn set_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        let language = language.into();
        self.set_text_language(language.clone())
            .set_asset_language(language)
    }

    pub fn set_text_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        let language = language.into();
        self.text_provider.set_language(language.into());
        self
    }

    pub fn set_asset_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        let language = language.into();
        for asset_provider in self.asset_providers.values_mut() {
            asset_provider.set_language(language.clone());
        }
        self
    }

    #[must_use]
    pub fn text_language(&self) -> Option<Language> {
        self.text_provider.get_language()
    }

    #[must_use]
    pub(crate) fn get_assets(&self, line_id: &UnderlyingYarnLine) -> LineAssets {
        self.asset_providers
            .values()
            .map(|p| p.get_assets(line_id))
            .collect()
    }

    pub(crate) fn add_command_task(&mut self, task: Task<()>) -> &mut Self {
        self.command_tasks.push(task);
        self
    }

    #[must_use]
    pub(crate) fn poll_tasks_and_check_if_done(&mut self) -> bool {
        self.command_tasks.retain(|task| !task.is_finished());
        self.command_tasks.is_empty()
    }
}

pub struct DialogueRunnerDataProviders<'a>(&'a DialogueRunner);

pub struct DialogueRunnerDataProvidersMut<'a>(&'a mut DialogueRunner);

impl DialogueRunnerDataProviders<'_> {
    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.0.text_provider.as_ref()
    }

    #[must_use]
    pub fn asset_providers(&self) -> impl Iterator<Item = &dyn AssetProvider> {
        self.0
            .asset_providers
            .values()
            .map(|provider| provider.as_ref())
    }

    #[must_use]
    pub fn asset_provider<T: 'static>(&self) -> Option<&T> {
        self.0
            .asset_providers
            .get(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any().downcast_ref())
    }

    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.0.dialogue.variable_storage()
    }
}

impl DialogueRunnerDataProvidersMut<'_> {
    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.0.text_provider.as_ref()
    }

    #[must_use]
    pub fn asset_providers(&self) -> impl Iterator<Item = &dyn AssetProvider> {
        self.0
            .asset_providers
            .values()
            .map(|provider| provider.as_ref())
    }

    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.0.dialogue.variable_storage()
    }

    #[must_use]
    pub fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.0.text_provider.as_mut()
    }

    #[must_use]
    pub fn asset_providers_mut(&mut self) -> impl Iterator<Item = &mut dyn AssetProvider> {
        self.0
            .asset_providers
            .values_mut()
            // Source: <https://stackoverflow.com/a/55866511/5903309>
            .map(|x| &mut **x as &mut dyn AssetProvider)
    }

    #[must_use]
    pub fn asset_provider<T: 'static>(&self) -> Option<&T> {
        self.0
            .asset_providers
            .get(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any().downcast_ref())
    }

    #[must_use]
    pub fn asset_provider_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.0
            .asset_providers
            .get_mut(&TypeId::of::<T>())
            .and_then(|provider| provider.as_any_mut().downcast_mut())
    }

    #[must_use]
    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.0.dialogue.variable_storage_mut()
    }
}

#[derive(Debug, Default)]
pub struct DialogueRunnerConfig {
    pub run_selected_options_as_lines: bool,
    pub start_automatically_on_node: Option<StartNode>,
}

pub struct DialogueRunnerBuilder {
    pub(crate) variable_storage: Box<dyn VariableStorage>,
    pub(crate) text_provider: SharedTextProvider,
    pub(crate) asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    pub(crate) library: YarnFnLibrary,
    pub(crate) compilation: Compilation,
    pub(crate) text_language: Option<Language>,
    pub(crate) asset_language: Option<Language>,
    pub(crate) localizations: Option<Localizations>,
    pub(crate) asset_server: AssetServer,
}

impl Debug for DialogueRunnerBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DialogueRunnerBuilder")
            .field("variable_storage", &self.variable_storage)
            .field("text_provider", &self.text_provider)
            .field("asset_providers", &self.asset_providers)
            .field("library", &self.library)
            .field("compilation", &self.compilation)
            .field("text_language", &self.text_language)
            .field("asset_language", &self.asset_language)
            .field("localizations", &self.localizations)
            .field("asset_server", &())
            .finish()
    }
}

impl DialogueRunnerBuilder {
    #[must_use]
    pub fn from_yarn_project(yarn_project: &YarnProject) -> Self {
        Self {
            variable_storage: Box::new(MemoryVariableStore::new()),
            text_provider: SharedTextProvider::new(StringsFileTextProvider::from_yarn_project(
                yarn_project,
            )),
            asset_providers: HashMap::new(),
            library: YarnFnLibrary::new(),
            compilation: yarn_project.compilation().clone(),
            text_language: None,
            asset_language: None,
            localizations: yarn_project.localizations().cloned(),
            asset_server: yarn_project.asset_server.clone(),
        }
    }

    #[must_use]
    pub fn with_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage = storage;
        self
    }

    #[must_use]
    pub fn with_text_provider(mut self, provider: impl TextProvider + 'static) -> Self {
        self.text_provider.replace(provider);
        if let Some(language) = self.text_language.take() {
            self.text_provider.set_language(Some(language));
        }
        self
    }

    #[must_use]
    pub fn add_asset_provider(mut self, mut provider: impl AssetProvider + 'static) -> Self {
        if let Some(language) = self.asset_language.as_ref() {
            provider.set_language(Some(language.clone()));
        }
        self.asset_providers
            .insert(provider.type_id(), Box::new(provider));
        self
    }

    #[must_use]
    pub fn with_text_language(mut self, language: impl Into<Option<Language>>) -> Self {
        let language = language.into();
        self.text_provider.set_language(language.into());
        self
    }

    #[must_use]
    pub fn with_asset_language(mut self, language: impl Into<Option<Language>>) -> Self {
        let language = language.into();
        for asset_provider in self.asset_providers.values_mut() {
            asset_provider.set_language(language.clone());
        }
        self
    }

    #[must_use]
    pub fn extend_library(mut self, library: YarnFnLibrary) -> Self {
        self.library.extend(library);
        self
    }

    #[must_use]
    pub fn build(mut self) -> DialogueRunner {
        let text_provider = Box::new(self.text_provider);
        let language = text_provider.get_language();
        let mut dialogue = Dialogue::new(self.variable_storage, text_provider.clone())
            .with_line_hints_enabled(true)
            .with_extended_library(self.library)
            .with_program(self.compilation.program.unwrap())
            .with_language_code(language);
        if dialogue.set_node_to_start().is_err() {
            info!("Dialogue has no start node, so it will need an explicitly set node to be run.");
        }
        for asset_provider in self.asset_providers.values_mut() {
            if let Some(ref localizations) = self.localizations {
                asset_provider.set_localizations(localizations.clone());
            }
            asset_provider.set_asset_server(self.asset_server.clone());
        }

        DialogueRunner {
            dialogue,
            text_provider,
            asset_providers: self.asset_providers,
            continue_: false,
            last_selected_option: None,
            commands: Default::default(),
            command_tasks: Vec::new(),
            config: Default::default(),
        }
    }
}
