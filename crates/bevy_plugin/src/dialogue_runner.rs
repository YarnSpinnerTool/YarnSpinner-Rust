pub use self::events::{
    DialogueCompleteEvent, DialogueStartEvent, ExecuteCommandEvent, LineHintsEvent,
    NodeCompleteEvent, NodeStartEvent, PresentLineEvent, PresentOptionsEvent,
};
pub use self::{
    builder::DialogueRunnerBuilder,
    dialogue_option::DialogueOption,
    inner::{InnerDialogue, InnerDialogueMut},
    localized_line::LocalizedLine,
};
use crate::commands::TaskFinishedIndicator;
use crate::line_provider::LineAssets;
use crate::prelude::*;
use crate::UnderlyingYarnLine;
use anyhow::{anyhow, bail, Result};
use bevy::asset::LoadedUntypedAsset;
use bevy::platform_support::collections::HashSet;
use bevy::{platform_support::collections::HashMap, prelude::*};
pub(crate) use runtime_interaction::DialogueExecutionSystemSet;
use std::any::TypeId;
use std::fmt::Debug;
use yarnspinner::core::Library;

mod builder;
mod dialogue_option;
mod events;
mod inner;
mod localized_line;
mod runtime_interaction;

pub(crate) fn dialogue_plugin(app: &mut App) {
    app.add_plugins(runtime_interaction::runtime_interaction_plugin)
        .add_plugins(localized_line::localized_line_plugin)
        .add_plugins(events::dialogue_runner_events_plugin)
        .add_plugins(dialogue_option::dialogue_option_plugin)
        .add_plugins(builder::dialogue_runner_builder_plugin)
        .add_plugins(inner::inner_dialogue_runner_plugin);
}

const DIALOGUE_MISSING_MESSAGE: &str = "Dialogue missing from DialogueRunner. \
                                  This is a bug. Please report it at https://github.com/YarnSpinnerTool/YarnSpinner-Rust/issues/new";
/// The main type to interact with the dialogue system.
/// Created by calling either [`YarnProject::create_dialogue_runner`] or [`YarnProject::build_dialogue_runner`].
#[derive(Debug, Component)]
pub struct DialogueRunner {
    pub(crate) dialogue: Option<Dialogue>,
    pub(crate) text_provider: Box<dyn TextProvider>,
    asset_providers: HashMap<TypeId, Box<dyn AssetProvider>>,
    pub(crate) will_continue_in_next_update: bool,
    pub(crate) last_selected_option: Option<OptionId>,
    pub(crate) commands: YarnCommands,
    command_tasks: Vec<Box<dyn TaskFinishedIndicator>>,
    localizations: Option<Localizations>,
    pub(crate) is_running: bool,
    run_selected_options_as_lines: bool,
    pub(crate) just_started: bool,
    pub(crate) popped_line_hints: Option<Vec<LineId>>,
    pub(crate) unsent_events: Vec<DialogueEvent>,
}

impl DialogueRunner {
    /// Tells the dialogue runner to try to advance the dialogue in the next update.
    /// This method must be called by the dialogue view when the user clicks on a button to show the next line.
    ///
    /// Note that the actual advancement of the dialogue will be postponed until the following conditions are met:
    /// - The text provider has finished loading its lines, indicated by [`TextProvider::are_lines_available`](yarnspinner::prelude::TextProvider::are_lines_available) returning `true`.
    /// - The asset providers have finished loading their assets, indicated by all [`AssetProvider::update_asset_availability`] calls returning `true`.
    /// - All previously called [`YarnCommand`]s are finished, indicated by their return type's [`TaskFinishedIndicator::is_finished`] returning `true`.
    pub fn continue_in_next_update(&mut self) -> &mut Self {
        if !self.is_running {
            panic!("Can't continue dialogue that isn't running. Please call `DialogueRunner::start_node()` before calling `DialogueRunner::continue_in_next_update()`.");
        }
        self.will_continue_in_next_update = true;
        self
    }

    /// Returns whether the dialogue runner will try to advance the dialogue in the next update.
    /// This can return `true` multiple updates in a row if the conditions mentioned in [`DialogueRunner::continue_in_next_update`] are not yet met.
    #[must_use]
    pub fn will_continue_in_next_update(&self) -> bool {
        self.will_continue_in_next_update
    }

    /// If the dialogue is currently waiting for the user to select an option, this method will select the option with the given id.
    /// Implies [`DialogueRunner::continue_in_next_update`].
    pub fn select_option(&mut self, option: OptionId) -> Result<&mut Self> {
        if !self.is_running {
            bail!("Can't select option {option}: the dialogue is currently not running. Please call `DialogueRunner::continue_in_next_update()` only after receiving a `PresentOptionsEvent`.")
        }
        self.inner_mut()
            .0
            .set_selected_option(option)
            .map_err(Error::from)?;
        self.last_selected_option.replace(option);
        self.continue_in_next_update();
        Ok(self)
    }

    /// Returns whether the dialogue runner is currently running. Returns `false` if:
    /// - The dialogue has not yet been started via [`DialogueRunner::start_node`]
    /// - The dialogue has been stopped via [`DialogueRunner::stop`]
    /// - The dialogue has finished running through all nodes
    #[must_use]
    pub fn is_running(&self) -> bool {
        self.is_running
    }

    /// Returns whether the dialogue runner is currently waiting for the user to select an option.
    /// If this is true, [`DialogueRunner::select_option`] must be called before the dialogue can continue.
    /// Calling [`DialogueRunner::continue_in_next_update`] will panic in this case.
    #[must_use]
    pub fn is_waiting_for_option_selection(&self) -> bool {
        self.inner().0.is_waiting_for_option_selection()
    }

    /// If set, every line the user selects will emit a [`PresentLineEvent`]. Defaults to `false`.
    pub fn run_selected_options_as_lines(
        &mut self,
        run_selected_options_as_lines: bool,
    ) -> &mut Self {
        self.run_selected_options_as_lines = run_selected_options_as_lines;
        self
    }

    /// If set, every line the user selects will emit a [`PresentLineEvent`]. Defaults to `false`.
    #[must_use]
    pub fn runs_selected_options_as_lines(&self) -> bool {
        self.run_selected_options_as_lines
    }

    /// Stops the execution of the dialogue. Any pending dialogue events will still be sent in the next update, including a [`DialogueCompleteEvent`].
    /// After this, [`DialogueRunner::start_node`] must be called before the dialogue can be advanced again.
    pub fn stop(&mut self) -> &mut Self {
        self.is_running = false;
        self.last_selected_option = None;
        self.popped_line_hints = None;
        self.will_continue_in_next_update = false;
        self.just_started = false;
        let stop_events = self.inner_mut().0.stop();
        self.unsent_events.extend(stop_events);
        self
    }

    /// Starts the dialogue at the given node.
    /// This method must be called after creation or after calling [`DialogueRunner::stop`] before the dialogue can be advanced. Implies [`DialogueRunner::continue_in_next_update`].
    /// If the dialogue was already running, this method will panic.
    ///
    /// See [`DialogueRunner::try_start_node`] for a fallible version of this method.
    pub fn start_node(&mut self, node_name: impl AsRef<str>) -> &mut Self {
        self.try_start_node(node_name)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    /// Fallible version of [`DialogueRunner::start_node`].
    pub fn try_start_node(&mut self, node_name: impl AsRef<str>) -> Result<&mut Self> {
        let node_name = node_name.as_ref();
        if self.is_running {
            bail!("Can't start dialogue from node {node_name}: the dialogue is currently in the middle of running. Stop the dialogue first.");
        }
        self.is_running = true;
        self.just_started = true;
        self.inner_mut()
            .0
            .set_node(node_name)
            .map_err(|e| anyhow!("Can't start dialogue from node {node_name}: {e}"))?;
        self.popped_line_hints = self.inner_mut().0.pop_line_hints();
        self.continue_in_next_update();
        Ok(self)
    }

    /// Returns the tags for the node `node_name`.
    ///
    /// The tags for a node are defined by setting the `tags` header in
    /// the node's source code. This header must be a space-separated list
    ///
    /// Returns [`None`] if the node is not present in the program.
    #[must_use]
    pub fn get_tags_for_node(&self, node_name: &str) -> Option<Vec<String>> {
        self.inner().0.get_tags_for_node(node_name)
    }

    /// Gets a value indicating whether a specified node exists in the Yarn files.
    #[must_use]
    pub fn node_exists(&self, node_name: &str) -> bool {
        self.inner().0.node_exists(node_name)
    }

    /// Gets the name of the node that this Dialogue is currently executing.
    /// This is [`None`] if [`DialogueRunner::is_running`] is `false`.
    #[must_use]
    pub fn current_node(&self) -> Option<String> {
        self.inner().0.current_node()
    }

    /// Returns a shallow clone of the registered [`VariableStorage`]. The storage used can be overridden by calling [`DialogueRunnerBuilder::with_variable_storage`].
    #[must_use]
    pub fn variable_storage(&self) -> &dyn VariableStorage {
        self.inner().0.variable_storage()
    }

    /// Returns a shallow mutable clone of the registered [`VariableStorage`]. The storage used can be overridden by calling [`DialogueRunnerBuilder::with_variable_storage`].
    #[must_use]
    pub fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.inner_mut().0.variable_storage_mut()
    }

    /// Returns whether both the text and asset providers have loaded all their lines.
    #[must_use]
    pub fn update_line_availability(
        &mut self,
        loaded_untyped_assets: &Assets<LoadedUntypedAsset>,
    ) -> bool {
        self.are_texts_available() && self.update_asset_availability(loaded_untyped_assets)
    }

    /// Returns whether the text provider has loaded all its lines.
    #[must_use]
    fn are_texts_available(&self) -> bool {
        self.text_provider.are_lines_available()
    }

    /// Returns whether all asset providers have loaded all their assets.
    /// If no asset providers where added via [`DialogueRunnerBuilder::add_asset_provider`], this will always return `true`.
    #[must_use]
    fn update_asset_availability(
        &mut self,
        loaded_untyped_assets: &Assets<LoadedUntypedAsset>,
    ) -> bool {
        self.asset_providers
            .values_mut()
            .all(|provider| provider.update_asset_availability(loaded_untyped_assets))
    }

    /// Sets the language of both the text and asset providers. Same as calling [`DialogueRunner::set_text_language`] and [`DialogueRunner::set_asset_language`].
    pub fn set_language(&mut self, language: impl Into<Language>) -> &mut Self {
        let language = language.into();
        self.set_text_language(language.clone())
            .set_asset_language(language)
    }

    /// Sets the language of the text provider.
    pub fn set_text_language(&mut self, language: impl Into<Language>) -> &mut Self {
        let language = language.into();
        self.assert_localizations_available_for_language(&language);
        self.inner_mut().0.set_language_code(language);
        self
    }

    /// Sets the language of all asset providers. If no asset providers where added via [`DialogueRunnerBuilder::add_asset_provider`], this will do nothing.
    pub fn set_asset_language(&mut self, language: impl Into<Language>) -> &mut Self {
        let language = language.into();
        self.assert_localizations_available_for_language(&language);
        for asset_provider in self.asset_providers.values_mut() {
            asset_provider.set_language(language.clone().into());
        }
        self
    }

    fn assert_localizations_available_for_language(&self, language: &Language) {
        let localizations = self.localizations.as_ref().expect(
            "Tried to set language, but no localizations are available. \
            Did you forget to call `YarnSpinnerApp::with_localizations(..)` on the plugin setup?",
        );
        assert!(
            localizations.supports_language(language),
            "Tried to set language to {language}, but no localizations are available for that language."
        );
    }

    /// Returns the library of functions that can be called from Yarn files.
    #[must_use]
    pub fn library(&self) -> &Library {
        self.inner().0.library()
    }

    /// Mutably returns the library of functions that can be called from Yarn files.
    #[must_use]
    pub fn library_mut(&mut self) -> &mut Library {
        self.inner_mut().0.library_mut()
    }

    /// Returns the command registrations that can be called from Yarn files.
    #[must_use]
    pub fn commands(&self) -> &YarnCommands {
        &self.commands
    }

    /// Mutably returns the command registrations that can be called from Yarn files.
    #[must_use]
    pub fn commands_mut(&mut self) -> &mut YarnCommands {
        &mut self.commands
    }

    /// Returns the language used by the [`TextProvider`]. If there are no [`Localizations`] available, this will return [`None`].
    #[must_use]
    pub fn text_language(&self) -> Option<Language> {
        self.inner().0.language_code().cloned()
    }

    /// Returns the language used by the [`AssetProvider`]s. If there are no [`Localizations`] available, this will return [`None`].
    /// Panics if the asset providers have different languages.
    #[must_use]
    pub fn asset_language(&self) -> Option<Language> {
        let languages: HashSet<_> = self
            .asset_providers
            .values()
            .map(|provider| provider.get_language())
            .collect();
        assert!(
            languages.len() <= 1,
            "Asset providers have different languages"
        );
        languages.into_iter().next().flatten()
    }

    /// Returns a struct that can be used to access a portion of the underlying [`Dialogue`]. This is advanced functionality.
    #[must_use]
    pub fn inner(&self) -> InnerDialogue {
        InnerDialogue(self.dialogue.as_ref().expect(DIALOGUE_MISSING_MESSAGE))
    }

    /// Mutably returns a struct that can be used to access a portion of the underlying [`Dialogue`]. This is advanced functionality.
    #[must_use]
    pub fn inner_mut(&mut self) -> InnerDialogueMut {
        InnerDialogueMut(self.dialogue.as_mut().expect(DIALOGUE_MISSING_MESSAGE))
    }

    /// Returns the registered [`TextProvider`]. By default, this is a [`StringsFileTextProvider`](crate::default_impl::StringsFileTextProvider).
    #[must_use]
    pub fn text_provider(&self) -> &dyn TextProvider {
        self.text_provider.as_ref()
    }

    /// Returns the registered [`AssetProvider`] of the given type if it was previously registered with [`DialogueRunnerBuilder::add_asset_provider`].
    #[must_use]
    pub fn asset_provider<T: 'static>(&self) -> Option<&T> {
        self.asset_providers
            .values()
            .filter_map(|p| p.as_any().downcast_ref())
            .next()
    }

    /// Iterates over all registered [`AssetProvider`]s.
    pub fn asset_providers(&self) -> impl Iterator<Item = &dyn AssetProvider> {
        self.asset_providers.values().map(|p| p.as_ref())
    }

    #[must_use]
    pub(crate) fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets {
        self.asset_providers
            .values()
            .map(|p| p.get_assets(line))
            .collect()
    }

    pub(crate) fn add_command_task(&mut self, task: Box<dyn TaskFinishedIndicator>) -> &mut Self {
        self.command_tasks.push(task);
        self
    }

    #[must_use]
    pub(crate) fn poll_tasks_and_check_if_done(&mut self) -> bool {
        self.command_tasks.retain(|task| !task.is_finished());
        self.command_tasks.is_empty()
    }
}
