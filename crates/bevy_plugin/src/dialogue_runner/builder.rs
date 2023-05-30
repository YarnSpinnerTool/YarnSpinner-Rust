use crate::default_impl::{MemoryVariableStore, StringsFileTextProvider};
use crate::line_provider::SharedTextProvider;
use crate::prelude::*;
use crate::UnderlyingTextProvider;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::{rngs::SmallRng, Rng, SeedableRng};
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
    start_node: Option<StartNode>,
}

#[derive(Debug, Clone)]
pub enum StartNode {
    DefaultStartNode,
    Node(String),
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
            library: create_extended_standard_library(),
            compilation: yarn_project.compilation().clone(),
            text_language: None,
            asset_language: None,
            localizations: yarn_project.localizations().cloned(),
            asset_server: yarn_project.asset_server.clone(),
            run_selected_options_as_lines: false,
            start_node: Some(StartNode::DefaultStartNode),
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
        self.text_provider.set_language(language);
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
    pub fn with_start_node(mut self, start_node: impl Into<Option<StartNode>>) -> Self {
        self.start_node = start_node.into();
        self
    }

    #[must_use]
    pub fn register_function(mut self, library: YarnFnLibrary) -> Self {
        self.library.extend(library);
        self
    }

    pub fn build(mut self) -> Result<DialogueRunner> {
        let text_provider = Box::new(self.text_provider);
        let language = text_provider.get_language();
        let mut dialogue = Dialogue::new(self.variable_storage, text_provider.clone())
            .with_line_hints_enabled(true)
            .with_extended_library(self.library)
            .with_program(self.compilation.program.unwrap())
            .with_language_code(language);
        for asset_provider in self.asset_providers.values_mut() {
            if let Some(ref localizations) = self.localizations {
                asset_provider.set_localizations(localizations.clone());
            }
            asset_provider.set_asset_server(self.asset_server.clone());
        }

        if let Some(start_node) = self.start_node {
            match start_node {
                StartNode::DefaultStartNode => {
                    dialogue.set_node_to_start()?;
                }
                StartNode::Node(node) => {
                    dialogue.set_node(node)?;
                }
            }
        } else {
            info!("Dialogue has no start node, so it will need an explicitly set node to be run.");
        };

        let popped_line_hints = dialogue.pop_line_hints();

        Ok(DialogueRunner {
            dialogue,
            text_provider,
            popped_line_hints,
            asset_providers: self.asset_providers,
            run_selected_options_as_lines: self.run_selected_options_as_lines,
            commands: YarnCommandRegistrations::default_commands(),
            is_running: default(),
            command_tasks: default(),
            will_continue_in_next_update: default(),
            last_selected_option: default(),
            just_started: default(),
        })
    }
}

fn create_extended_standard_library() -> YarnFnLibrary {
    YarnFnLibrary::standard_library()
        .with_function("random", || SmallRng::from_entropy().gen_range(0.0..1.0))
        .with_function("random_range", |min: f32, max: f32| {
            if let Some(min) = min.as_int() {
                if let Some(max_inclusive) = max.as_int() {
                    return SmallRng::from_entropy().gen_range(min..=max_inclusive) as f32;
                }
            }
            SmallRng::from_entropy().gen_range(min..max)
        })
        .with_function("dice", |sides: u32| {
            if sides == 0 {
                return 1;
            }
            SmallRng::from_entropy().gen_range(1..=sides)
        })
        .with_function("round", |num: f32| num.round() as i32)
        .with_function("round_places", |num: f32, places: u32| {
            num.round_places(places)
        })
        .with_function("floor", |num: f32| num.floor() as i32)
        .with_function("ceil", |num: f32| num.ceil() as i32)
        .with_function("inc", |num: f32| {
            if let Some(num) = num.as_int() {
                num + 1
            } else {
                num.ceil() as i32
            }
        })
        .with_function("dec", |num: f32| {
            if let Some(num) = num.as_int() {
                num - 1
            } else {
                num.floor() as i32
            }
        })
        .with_function("decimal", |num: f32| num.fract())
        .with_function("int", |num: f32| num.trunc() as i32)
}

trait FloatExt: Copy {
    fn as_int(self) -> Option<i32>;
    fn round_places(self, places: u32) -> Self;
}

impl FloatExt for f32 {
    fn as_int(self) -> Option<i32> {
        (self.fract() <= f32::EPSILON).then_some(self as i32)
    }

    fn round_places(self, places: u32) -> Self {
        let factor = 10_u32.pow(places) as f32;
        (self * factor).round() / factor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rounds_places() {
        for (num, places, expected) in [
            (1.0, 0, 1.0),
            (1.2, 1, 1.2),
            (0.4, 0, 0.0),
            (43.132, 0, 43.0),
            (1.1, 2, 1.1),
            (123.123, 3, 123.123),
            (-10.3, 1, -10.3),
            (-11.99, 1, -12.0),
        ] {
            assert_eq!(expected, num.round_places(places));
        }
    }
}
