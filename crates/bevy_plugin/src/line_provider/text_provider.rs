use crate::line_provider::LineProviderSystemSet;
use crate::prelude::*;
use crate::UnderlyingTextProvider;
use bevy::prelude::*;
pub(crate) use shared_text_provider::SharedTextProvider;
use std::any::Any;
use std::collections::HashMap;
pub use strings_file_text_provider::StringsFileTextProvider;

mod shared_text_provider;
mod strings_file_text_provider;

pub(crate) fn text_provider_plugin(app: &mut App) {
    app.fn_plugin(shared_text_provider::shared_text_provider_plugin)
        .fn_plugin(strings_file_text_provider::strings_file_text_provider_plugin)
        .add_system(
            fetch_resources
                .in_set(LineProviderSystemSet)
                .in_set(YarnSlingerSystemSet),
        );
}

/// Trait for the provider the [`DialogueRunner`]s text. By default, this is a [`StringsFileTextProvider`].
/// You can override this with [`DialogueRunnerBuilder::with_text_provider`] if you want a custom localization strategy.
/// For most users however, the default is fine.
pub trait TextProvider: UnderlyingTextProvider {
    fn set_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);
    fn extend_base_string_table(&mut self, string_table: HashMap<LineId, StringInfo>);
    fn take_fetched_assets(&mut self, asset: Box<dyn Any>);
    fn fetch_assets(&self, world: &World) -> Option<Box<dyn Any + 'static>>;
}

pub(crate) fn fetch_resources(world: &mut World) {
    let dialogue_runner_entities: Vec<_> = world
        .iter_entities()
        .map(|entity| entity.id())
        .filter(|entity| world.get::<DialogueRunner>(*entity).is_some())
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
