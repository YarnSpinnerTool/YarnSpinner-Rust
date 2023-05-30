use crate::default_impl::{MemoryVariableStore, StringsFileTextProvider};
use crate::dialogue_runner::StartNode;
use crate::line_provider::SharedTextProvider;
use crate::prelude::*;
use crate::UnderlyingTextProvider;
use bevy::prelude::*;
use bevy::utils::HashMap;
use std::any::{Any, TypeId};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub(crate) fn dialogue_runner_builder_plugin(_app: &mut App) {}

pub struct DialogueRunnerBuilder {
    variable_storage: Box<dyn VariableStorage>,
    text_provider: SharedTextProvider,
    asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    library: YarnFnLibrary,
    compilation: Compilation,
    text_language: Option<Language>,
    asset_language: Option<Language>,
    localizations: Option<Localizations>,
    asset_server: AssetServer,
    run_selected_options_as_lines: bool,
    start_automatically_on_node: Option<StartNode>,
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
            run_selected_options_as_lines: false,
            start_automatically_on_node: Some(StartNode::DefaultStartNode),
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
    pub fn with_run_selected_option_as_line(mut self, run_selected_option_as_line: bool) -> Self {
        self.run_selected_options_as_lines = run_selected_option_as_line;
        self
    }

    #[must_use]
    pub fn with_start_automatically_on_node(
        mut self,
        start_automatically_on_node: impl Into<Option<StartNode>>,
    ) -> Self {
        self.start_automatically_on_node = start_automatically_on_node.into();
        self
    }

    #[must_use]
    pub fn extend_library(mut self, library: YarnFnLibrary) -> Self {
        self.library.extend(library);
        self
    }

    #[must_use]
    pub fn build(mut self) -> Result<DialogueRunner> {
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

        let will_auto_start = if let Some(start_node) = &self.start_automatically_on_node {
            match start_node {
                StartNode::DefaultStartNode => {
                    dialogue.set_node_to_start()?;
                }
                StartNode::Node(node_name) => {
                    dialogue.set_node(node_name)?;
                }
            }
            true
        } else {
            false
        };

        Ok(DialogueRunner {
            dialogue,
            text_provider,
            asset_providers: self.asset_providers,
            will_continue_in_next_update: will_auto_start,
            last_selected_option: None,
            commands: Default::default(),
            command_tasks: Vec::new(),
            is_running: false,
            run_selected_options_as_lines: self.run_selected_options_as_lines,
            start_automatically_on_node: self.start_automatically_on_node,
            just_started: false,
        })
    }
}
