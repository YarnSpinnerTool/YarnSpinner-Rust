pub use self::events::{
    DialogueCompleteEvent, ExecuteCommandEvent, LineHintsEvent, NodeCompleteEvent, NodeStartEvent,
    PresentLineEvent, PresentOptionsEvent,
};
pub use self::{dialogue_option::DialogueOption, localized_line::LocalizedLine};
use crate::asset_provider::Assets;
use crate::prelude::*;
use crate::UnderlyingYarnLine;
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

    pub(crate) fn get_assets(&self, line_id: &UnderlyingYarnLine) -> Assets {
        self.asset_provider
            .as_ref()
            .map(|p| p.get_assets(line_id))
            .unwrap_or_default()
    }
}

impl DialogueConfigurator for DialogueRunner {
    fn text_provider(&self) -> &dyn TextProvider {
        self.dialogue.text_provider()
    }

    fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.dialogue.text_provider_mut()
    }

    fn asset_provider(&self) -> Option<&dyn AssetProvider> {
        self.asset_provider.as_deref()
    }

    fn asset_provider_mut(&mut self) -> Option<&mut dyn AssetProvider> {
        // Source: <https://stackoverflow.com/a/55866511/5903309>
        self.asset_provider
            .as_mut()
            .map(|x| &mut **x as &mut dyn AssetProvider)
    }

    fn variable_storage(&self) -> &dyn VariableStorage {
        self.dialogue.variable_storage()
    }

    fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.dialogue.variable_storage_mut()
    }

    fn library(&self) -> &YarnFnLibrary {
        self.dialogue.library()
    }

    fn library_mut(&mut self) -> &mut YarnFnLibrary {
        self.dialogue.library_mut()
    }
}

#[derive(Debug)]
pub struct DialogueRunnerBuilder<'a> {
    pub(crate) variable_storage_override: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider_override: Option<Box<dyn TextProvider>>,
    pub(crate) asset_provider_override: Option<Option<Box<dyn AssetProvider>>>,
    pub(crate) yarn_project: &'a YarnProject,
}

impl<'a> DialogueRunnerBuilder<'a> {
    #[must_use]
    pub fn with_yarn_project(yarn_project: &'a YarnProject) -> Self {
        Self {
            variable_storage_override: None,
            text_provider_override: None,
            asset_provider_override: None,
            yarn_project,
        }
    }

    #[must_use]
    pub fn override_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage_override = Some(storage);
        self
    }

    #[must_use]
    pub fn override_text_provider(mut self, provider: Box<dyn TextProvider>) -> Self {
        self.text_provider_override = Some(provider);
        self
    }

    #[must_use]
    pub fn override_asset_provider(mut self, provider: Option<Box<dyn AssetProvider>>) -> Self {
        self.asset_provider_override = Some(provider);
        self
    }

    #[must_use]
    pub fn build(self) -> DialogueRunner {
        let variable_storage = self
            .variable_storage_override
            .unwrap_or_else(|| self.yarn_project.variable_storage.clone());
        let text_provider = self
            .text_provider_override
            .unwrap_or_else(|| self.yarn_project.text_provider.clone());
        let asset_provider = self
            .asset_provider_override
            .unwrap_or_else(|| self.yarn_project.asset_provider.clone());
        let mut dialogue =
            Dialogue::new(variable_storage, text_provider).with_line_hints_enabled(true);
        if let Some(language) = dialogue.text_provider().get_language() {
            dialogue.set_language_code(language).unwrap();
        }
        if dialogue.set_node_to_start().is_err() {
            info!("Dialogue has no start node, so it will need an explicitly set node to be run.");
        }

        DialogueRunner {
            dialogue,
            asset_provider,
            continue_: false,
            run_selected_options_as_lines: false,
            last_selected_option: None,
        }
    }
}
