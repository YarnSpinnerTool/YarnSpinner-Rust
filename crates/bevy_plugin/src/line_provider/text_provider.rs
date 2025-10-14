use crate::UnderlyingTextProvider;
use crate::line_provider::LineProviderSystemSet;
use crate::prelude::*;
use bevy::prelude::*;
pub(crate) use shared_text_provider::SharedTextProvider;
use std::any::Any;
use std::collections::HashMap;
pub use strings_file_text_provider::StringsFileTextProvider;

mod shared_text_provider;
mod strings_file_text_provider;

pub(crate) fn text_provider_plugin(app: &mut App) {
    app.add_plugins(shared_text_provider::shared_text_provider_plugin)
        .add_plugins(strings_file_text_provider::strings_file_text_provider_plugin)
        .add_systems(
            Update,
            fetch_resources
                .in_set(LineProviderSystemSet)
                .in_set(YarnSpinnerSystemSet),
        );
}

/// Trait for the provider the [`DialogueRunner`]s text. By default, this is a [`StringsFileTextProvider`].
/// You can override this with [`DialogueRunnerBuilder::with_text_provider`] if you want a custom localization strategy.
/// For most users however, the default is fine.
pub trait TextProvider: UnderlyingTextProvider {
    /// Stores a string table containing the base language strings, i.e. the strings found in the Yarn files themselves.
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);

    /// Extends the string table set by [`TextProvider::set_base_string_table`] with additional strings.
    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);

    /// Stores the assets fetched by [`TextProvider::fetch_assets`].
    /// This functionality is split into two functions because [`TextProvider::take_fetched_assets`] is mutable,
    /// so we lose access to the [`World`] when calling it since it contains this very [`TextProvider`].
    ///
    fn take_fetched_assets(&mut self, asset: Box<dyn Any>);
    /// Fetches any necessary assets. The returned value is then passed to [`TextProvider::take_fetched_assets`].
    /// This functionality is split into two functions because [`TextProvider::take_fetched_assets`] is mutable,
    /// so we lose access to the [`World`] when calling it since it contains this very [`TextProvider`].
    fn fetch_assets(&self, world: &World) -> Option<Box<dyn Any + 'static>>;
}

pub(crate) fn fetch_resources(world: &mut World) {
    let dialogue_runner_entities: Vec<_> = world
        .query_filtered::<Entity, With<DialogueRunner>>()
        .iter(world)
        .collect();

    for entity in dialogue_runner_entities {
        let assets = {
            let dialogue_runner = world.get::<DialogueRunner>(entity).unwrap();
            dialogue_runner.text_provider.fetch_assets(world)
        };
        if let Some(assets) = assets {
            let mut dialogue_runner = world.get_mut::<DialogueRunner>(entity).unwrap();
            dialogue_runner.text_provider.take_fetched_assets(assets)
        }
    }
}
