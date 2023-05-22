use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Debug;

mod runtime_interaction;

pub(crate) fn dialogue_plugin(app: &mut App) {
    app.fn_plugin(runtime_interaction::runtime_interaction_plugin);
}

#[derive(Debug, Component)]
pub struct DialogueRunner {
    pub(crate) dialogue: Dialogue,
    pub line_asset_provider_override: Option<Box<dyn LineAssetProvider>>,
    pub continue_: bool,
}

impl DialogueRunner {
    pub fn with_library(mut self, library: YarnFnLibrary) -> Self {
        self.extend_library(library);
        self
    }

    pub fn extend_library(&mut self, library: YarnFnLibrary) -> &mut Self {
        self.dialogue.library_mut().extend(library);
        self
    }

    pub fn continue_(&mut self) -> &mut Self {
        self.continue_ = true;
        self
    }

    pub fn select_option(&mut self, option: OptionId) -> Result<&mut Self> {
        self.dialogue
            .set_selected_option(option)
            .map_err(Error::from)?;
        Ok(self)
    }
}

#[derive(Debug)]
pub struct DialogueRunnerBuilder<'a> {
    pub(crate) variable_storage_override: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider_override: Option<Box<dyn TextProvider>>,
    pub(crate) line_asset_provider_override: Option<Option<Box<dyn LineAssetProvider>>>,
    pub(crate) yarn_project: &'a YarnProject,
}

impl<'a> DialogueRunnerBuilder<'a> {
    pub fn with_yarn_project(yarn_project: &'a YarnProject) -> Self {
        Self {
            variable_storage_override: None,
            text_provider_override: None,
            line_asset_provider_override: None,
            yarn_project,
        }
    }

    pub fn override_variable_storage(mut self, storage: Box<dyn VariableStorage>) -> Self {
        self.variable_storage_override = Some(storage);
        self
    }

    pub fn override_text_provider(mut self, provider: Box<dyn TextProvider>) -> Self {
        self.text_provider_override = Some(provider);
        self
    }

    pub fn override_line_asset_provider(
        mut self,
        provider: Option<Box<dyn LineAssetProvider>>,
    ) -> Self {
        self.line_asset_provider_override = Some(provider);
        self
    }

    pub fn build(self) -> DialogueRunner {
        let variable_storage = self
            .variable_storage_override
            .unwrap_or_else(|| self.yarn_project.variable_storage.clone());
        let text_provider = self
            .text_provider_override
            .unwrap_or_else(|| self.yarn_project.text_provider.clone());
        let line_asset_provider = self
            .line_asset_provider_override
            .unwrap_or_else(|| self.yarn_project.line_asset_provider.clone());
        let mut dialogue =
            Dialogue::new(variable_storage, text_provider).with_line_hints_enabled(true);
        if let Some(language) = dialogue.text_provider().get_language_code() {
            dialogue.set_language_code(language).unwrap();
        }

        DialogueRunner {
            dialogue,
            line_asset_provider_override: line_asset_provider,
            continue_: false,
        }
    }
}
