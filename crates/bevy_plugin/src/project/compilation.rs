//! Backend-agnostic part of loading a Yarn project: turns a [`LoadYarnProjectEvent`]
//! into loaded [`YarnFile`] handles. The backends in [`super::backend`] take it from there
//! and insert the [`YarnProject`].

use crate::plugin::AssetRoot;
use crate::prelude::*;
use crate::project::{CompilationSystemSet, LoadYarnProjectEvent, WatchingForChanges};
use anyhow::{Result, anyhow, bail};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use std::fmt::Debug;

/// Ordering inside [`CompilationSystemSet`].
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) enum CompilationStep {
    /// Shared: event -> [`YarnFilesBeingLoaded`].
    Queue,
    /// Backend-specific: [`YarnFilesBeingLoaded`] -> [`YarnProject`], and recompilation.
    Compile,
    /// Shared: cleanup.
    Cleanup,
}

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<YarnFilesToLoad>()
        .init_resource::<YarnFilesToLoad>()
        .init_resource::<YarnFilesBeingLoaded>()
        .configure_sets(
            Update,
            (
                CompilationStep::Queue,
                CompilationStep::Compile,
                CompilationStep::Cleanup,
            )
                .chain()
                .in_set(CompilationSystemSet)
                .in_set(YarnSpinnerSystemSet),
        )
        .add_systems(
            Update,
            (
                (
                    load_project.pipe(panic_on_err),
                    add_yarn_files_to_load_queue
                        .pipe(panic_on_err)
                        .run_if(resource_exists_and_changed::<YarnFilesToLoad>),
                )
                    .chain()
                    .in_set(CompilationStep::Queue),
                clear_temp_yarn_project
                    .run_if(resource_added::<YarnProject>)
                    .in_set(CompilationStep::Cleanup),
            ),
        )
        .add_plugins(super::backend::plugin);
}

#[derive(Debug, Resource)]
pub(crate) struct YarnProjectConfigToLoad {
    pub(crate) localizations: Option<Option<Localizations>>,
    pub(crate) watching_for_changes: bool,
    pub(crate) yarnproject: Option<bevy::asset::AssetPath<'static>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct YarnFilesToLoad(pub(crate) HashSet<YarnFileSource>);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct YarnFilesBeingLoaded(pub(crate) HashSet<Handle<YarnFile>>);

fn load_project(
    mut commands: Commands,
    mut events: ResMut<Messages<LoadYarnProjectEvent>>,
    is_watching_for_changes: Res<WatchingForChanges>,
    mut already_loaded: Local<bool>,
) -> SystemResult {
    for event in events.drain() {
        if *already_loaded {
            bail!(
                "Yarn project already loaded. Sending multiple LoadYarnProjectEvent is not allowed."
            );
        }
        assert!(
            !event.yarn_files.is_empty(),
            "Failed to load Yarn project in deferred mode: no Yarn files were specified. \
            Did run `LoadYarnProjectEvent::empty()` without adding any Yarn files with `LoadYarnProjectEvent::add_yarn_file` and `LoadYarnProjectEvent::add_yarn_files`? \
            If you wanted to load from the default directory instead, use `LoadYarnProjectEvent::default()`."
        );

        commands.insert_resource(YarnProjectConfigToLoad {
            localizations: Some(event.localizations),
            watching_for_changes: is_watching_for_changes.0,
            yarnproject: event.yarnproject,
        });
        commands.insert_resource(YarnFilesToLoad(event.yarn_files));
        *already_loaded = true;
    }
    Ok(())
}

fn add_yarn_files_to_load_queue(
    mut yarn_files_to_load: ResMut<YarnFilesToLoad>,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    mut assets: ResMut<Assets<YarnFile>>,
    asset_server: Res<AssetServer>,
    asset_root: Res<AssetRoot>,
) -> Result<()> {
    if yarn_files_to_load.0.is_empty() {
        return Ok(());
    }
    let handles: Result<Vec<_>> = yarn_files_to_load
        .0
        .drain()
        .map(|source| {
            source
                .load(&asset_server, &mut assets, &asset_root)
                .map_err(|e| anyhow!("Error loading Yarn file: {e}"))
        })
        .collect();
    let handles = handles?;
    let handles = handles.iter().flat_map(|handles| handles.iter()).cloned();

    yarn_files_being_loaded.0.extend(handles);
    Ok(())
}

fn clear_temp_yarn_project(mut commands: Commands) {
    // Done here instead of `compile_loaded_yarn_files` so that systems can access the global resources during the same frame
    commands.remove_resource::<YarnProjectConfigToLoad>();
}
