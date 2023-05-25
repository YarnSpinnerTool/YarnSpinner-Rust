pub use self::events::{
    DialogueCompleteEvent, ExecuteCommandEvent, LineHintsEvent, NodeCompleteEvent, NodeStartEvent,
    PresentLineEvent, PresentOptionsEvent,
};
pub use self::{dialogue_option::DialogueOption, localized_line::LocalizedLine};
use crate::asset_provider::{LineAssets, SharedTextProvider, StringsFileTextProvider};
use crate::default_impl::MemoryVariableStore;
use crate::prelude::*;
use crate::{UnderlyingTextProvider, UnderlyingYarnLine};
use bevy::prelude::*;
use std::fmt::Debug;

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
    pub(crate) text_provider: Box<dyn TextProvider>,
    pub(crate) asset_provider: Option<Box<dyn AssetProvider>>,
    pub(crate) continue_: bool,
    pub(crate) run_selected_options_as_lines: bool,
    pub(crate) last_selected_option: Option<OptionId>,
}

impl DialogueRunner {
    pub fn continue_in_next_update(&mut self) -> &mut Self {
        self.continue_ = true;
        self
    }

    pub fn select_option(&mut self, option: OptionId) -> Result<&mut Self> {
        self.last_selected_option.replace(option.clone());
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
    pub fn treats_selected_options_as_lines(&mut self) -> bool {
        self.run_selected_options_as_lines
    }

    #[must_use]
    pub fn treats_selected_options_as_lines_mut(&mut self) -> &mut bool {
        &mut self.run_selected_options_as_lines
    }

    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.text_provider.as_ref()
    }

    #[must_use]
    pub fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.text_provider.as_mut()
    }

    #[must_use]
    pub fn asset_provider(&self) -> Option<&dyn AssetProvider> {
        self.asset_provider.as_deref()
    }

    #[must_use]
    pub fn asset_provider_mut(&mut self) -> Option<&mut dyn AssetProvider> {
        // Source: <https://stackoverflow.com/a/55866511/5903309>
        self.asset_provider
            .as_mut()
            .map(|x| &mut **x as &mut dyn AssetProvider)
    }

    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.dialogue.variable_storage()
    }

    #[must_use]
    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.dialogue.variable_storage_mut()
    }

    #[must_use]
    pub fn library(&self) -> &YarnFnLibrary {
        self.dialogue.library()
    }

    #[must_use]
    pub fn library_mut(&mut self) -> &mut YarnFnLibrary {
        self.dialogue.library_mut()
    }

    #[must_use]
    pub fn are_line_texts_available(&self) -> bool {
        self.text_provider.are_lines_available()
    }

    #[must_use]
    pub fn are_line_assets_available(&self) -> bool {
        self.asset_provider.is_some()
    }

    #[must_use]
    pub fn are_lines_available(&self) -> bool {
        self.are_line_texts_available() && self.are_line_assets_available()
    }

    pub fn set_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        let language = language.into();
        self.set_text_language(language.clone());
        self.set_asset_language(language);
        self
    }

    pub fn set_text_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        self.text_provider.set_language(language.into());
        self
    }

    pub fn set_asset_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        self.asset_provider
            .as_mut()
            .map(|p| p.set_language(language.into()));
        self
    }

    pub(crate) fn get_assets(&self, line_id: &UnderlyingYarnLine) -> LineAssets {
        self.asset_provider
            .as_ref()
            .map(|p| p.get_assets(line_id))
            .unwrap_or_default()
    }
}

#[derive(Debug)]
pub struct DialogueRunnerBuilder {
    pub(crate) variable_storage: Box<dyn VariableStorage>,
    pub(crate) text_provider: SharedTextProvider,
    pub(crate) asset_provider: Option<Box<dyn AssetProvider>>,
    pub(crate) library: YarnFnLibrary,
}

impl DialogueRunnerBuilder {
    #[must_use]
    pub fn from_yarn_project(yarn_project: &YarnProject) -> Self {
        Self {
            variable_storage: Box::new(MemoryVariableStore::new()),
            text_provider: SharedTextProvider::new(StringsFileTextProvider::from_yarn_project(
                yarn_project,
            )),
            asset_provider: None,
            library: YarnFnLibrary::new(),
        }
    }

    #[must_use]
    pub fn with_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage = storage;
        self
    }

    #[must_use]
    pub fn with_text_provider(mut self, provider: impl TextProvider + 'static) -> Self {
        self.text_provider = SharedTextProvider::new(provider);
        self
    }

    #[must_use]
    pub fn with_asset_provider(mut self, provider: impl AssetProvider + 'static) -> Self {
        self.asset_provider.replace(Box::new(provider));
        self
    }

    #[must_use]
    pub fn extend_library(mut self, library: YarnFnLibrary) -> Self {
        self.library.extend(library);
        self
    }

    #[must_use]
    pub fn build(self) -> DialogueRunner {
        let text_provider = Box::new(self.text_provider);
        let language = text_provider.get_language();
        let mut dialogue = Dialogue::new(self.variable_storage, text_provider.clone())
            .with_line_hints_enabled(true)
            .with_extended_library(self.library)
            .with_language_code(language);
        if dialogue.set_node_to_start().is_err() {
            info!("Dialogue has no start node, so it will need an explicitly set node to be run.");
        }

        DialogueRunner {
            dialogue,
            text_provider,
            asset_provider: self.asset_provider,
            continue_: false,
            run_selected_options_as_lines: false,
            last_selected_option: None,
        }
    }
}
