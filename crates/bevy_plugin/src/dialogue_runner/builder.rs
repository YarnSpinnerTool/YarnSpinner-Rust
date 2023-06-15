use crate::default_impl::{MemoryVariableStore, StringsFileTextProvider};
use crate::line_provider::SharedTextProvider;
use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use std::any::{Any, TypeId};
use std::fmt;
use std::fmt::{Debug, Formatter};

pub(crate) fn dialogue_runner_builder_plugin(_app: &mut App) {}

/// A builder for [`DialogueRunner`]. This is instantiated for you by calling [`YarnProject::build_dialogue_runner`].
pub struct DialogueRunnerBuilder {
    variable_storage: Box<dyn VariableStorage>,
    text_provider: SharedTextProvider,
    asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    library: YarnFnLibrary,
    commands: YarnCommandRegistrations,
    compilation: Compilation,
    text_language: Option<Option<Language>>,
    asset_language: Option<Option<Language>>,
    localizations: Option<Localizations>,
    asset_server: AssetServer,
    run_selected_options_as_lines: bool,
}

impl Debug for DialogueRunnerBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("DialogueRunnerBuilder")
            .field("variable_storage", &self.variable_storage)
            .field("text_provider", &self.text_provider)
            .field("asset_providers", &self.asset_providers)
            .field("library", &self.library)
            .field("commands", &self.commands)
            .field("compilation", &self.compilation)
            .field("text_language", &self.text_language)
            .field("asset_language", &self.asset_language)
            .field("localizations", &self.localizations)
            .field("asset_server", &())
            .field(
                "run_selected_options_as_lines",
                &self.run_selected_options_as_lines,
            )
            .finish()
    }
}

impl DialogueRunnerBuilder {
    #[must_use]
    pub(crate) fn from_yarn_project(yarn_project: &YarnProject) -> Self {
        Self {
            variable_storage: Box::new(MemoryVariableStore::new()),
            text_provider: SharedTextProvider::new(StringsFileTextProvider::from_yarn_project(
                yarn_project,
            )),
            asset_providers: HashMap::new(),
            library: create_extended_standard_library(),
            commands: YarnCommandRegistrations::builtin_commands(),
            compilation: yarn_project.compilation().clone(),
            text_language: None,
            asset_language: None,
            localizations: yarn_project.localizations().cloned(),
            asset_server: yarn_project.asset_server.clone(),
            run_selected_options_as_lines: false,
        }
    }

    /// Replaces the [`VariableStorage`] used by the [`DialogueRunner`]. By default, this is a [`MemoryVariableStore`].
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

    /// Sets the language used by both the text and asset providers.
    /// If you want to set them separately, use [`DialogueRunnerBuilder::with_text_language`] and [`DialogueRunnerBuilder::with_asset_language`].
    ///
    /// If the [`YarnProject`] has [`Localizations`], this will be the language of the base localization by default.
    /// Otherwise, this value must be left at `None`.
    #[must_use]
    pub fn set_language(self, language: impl Into<Option<Language>>) -> Self {
        let language = language.into();
        self.with_text_language(language.clone())
            .with_asset_language(language)
    }

    /// Sets the language used by the text provider.
    ///
    /// If the [`YarnProject`] has [`Localizations`], this will be the language of the base localization by default.
    /// Otherwise, this value must be left at `None`.
    #[must_use]
    pub fn with_text_language(mut self, language: impl Into<Option<Language>>) -> Self {
        let language = language.into();
        self.text_language.replace(language);
        self
    }

    /// Sets the language used by the asset providers.
    ///
    /// If the [`YarnProject`] has localizations, this will be the language of the base localization by default.
    /// Otherwise, this value must be left at `None`.
    #[must_use]
    pub fn with_asset_language(mut self, language: impl Into<Option<Language>>) -> Self {
        let language = language.into();
        self.asset_language.replace(language);
        self
    }

    /// If set, every line the user selects will emit a [`PresentLineEvent`](crate::events::PresentLineEvent). Defaults to `false`.
    #[must_use]
    pub fn with_run_selected_options_as_line(mut self, run_selected_option_as_line: bool) -> Self {
        self.run_selected_options_as_lines = run_selected_option_as_line;
        self
    }

    /// Extends the standard library with custom functions callable within Yarn files.
    #[must_use]
    pub fn extend_library(mut self, library: YarnFnLibrary) -> Self {
        self.library.extend(library);
        self
    }

    /// Extends the command registrations with custom commands callable within Yarn files.
    #[must_use]
    pub fn extend_command_registrations(mut self, commands: YarnCommandRegistrations) -> Self {
        self.commands.extend(commands);
        self
    }

    /// Builds the [`DialogueRunner`]. See [`DialogueRunner::try_build`] for the fallible version.
    pub fn build(mut self) -> DialogueRunner {
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
        dialogue.add_program(self.compilation.program.unwrap());

        for asset_provider in self.asset_providers.values_mut() {
            if let Some(ref localizations) = self.localizations {
                asset_provider.set_localizations(localizations.clone());
            }

            asset_provider.set_asset_server(self.asset_server.clone());
        }

        let popped_line_hints = dialogue.pop_line_hints();

        let base_language = self
            .localizations
            .as_ref()
            .map(|l| &l.base_localization.language)
            .cloned();

        let mut dialogue_runner = DialogueRunner {
            dialogue,
            text_provider,
            popped_line_hints,
            asset_providers: self.asset_providers,
            run_selected_options_as_lines: self.run_selected_options_as_lines,
            commands: self.commands,
            is_running: default(),
            command_tasks: default(),
            will_continue_in_next_update: default(),
            last_selected_option: default(),
            just_started: default(),
            unsent_events: default(),
            localizations: self.localizations,
        };

        let text_language = self
            .text_language
            .take()
            .unwrap_or_else(|| base_language.clone());

        if let Some(text_language) = text_language {
            dialogue_runner.set_text_language(text_language);
        }

        let asset_language = self.asset_language.take().unwrap_or(base_language);

        if let Some(asset_language) = asset_language {
            dialogue_runner.set_asset_language(asset_language);
        }

        Ok(dialogue_runner)
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
