use crate::fmt_utils::SkipDebug;
use crate::localization::{LineIdUpdateSystemSet, UpdateAllStringsFilesForStringTableEvent};
use crate::plugin::AssetRoot;
use crate::prelude::*;
use crate::project::{CompilationSystemSet, LoadYarnProjectEvent, WatchingForChanges};
use anyhow::{Result, anyhow, bail};
use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use std::fmt::Debug;

pub(crate) fn project_compilation_plugin(app: &mut App) {
    app.register_type::<YarnFilesToLoad>()
        .init_resource::<YarnFilesToLoad>()
        .init_resource::<YarnFilesBeingLoaded>()
        .add_message::<RecompileLoadedYarnFilesEvent>()
        .add_systems(
            Update,
            (
                load_project.pipe(panic_on_err),
                add_yarn_files_to_load_queue
                    .pipe(panic_on_err)
                    .run_if(resource_exists_and_changed::<YarnFilesToLoad>),
                compile_loaded_yarn_files
                    .pipe(panic_on_err)
                    .run_if(resource_exists::<YarnFilesToLoad>),
                recompile_loaded_yarn_files
                    .pipe(log_error)
                    .run_if(events_in_queue::<RecompileLoadedYarnFilesEvent>()),
                clear_temp_yarn_project.run_if(resource_added::<YarnProject>),
            )
                .chain()
                .after(LineIdUpdateSystemSet)
                .in_set(CompilationSystemSet)
                .in_set(YarnSpinnerSystemSet),
        );
}

#[derive(Debug, Resource)]
pub(crate) struct YarnProjectConfigToLoad {
    pub(crate) localizations: Option<Option<Localizations>>,
    pub(crate) watching_for_changes: bool,
    pub(crate) development_file_generation: DevelopmentFileGeneration,
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
        if event.development_file_generation == DevelopmentFileGeneration::Full
            && !is_watching_for_changes.0
        {
            warn!(
                "Development file generation mode is set to `Full`, but hot reloading is not turned on. \
                For an optimal development experience, we recommend turning on hot reloading by activating the \"file_watcher\" feature of Bevy"
            );
        }

        commands.insert_resource(YarnProjectConfigToLoad {
            localizations: Some(event.localizations),
            watching_for_changes: is_watching_for_changes.0,
            development_file_generation: event.development_file_generation,
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

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, Message)]
#[reflect(Debug, Default, PartialEq)]
pub(crate) struct RecompileLoadedYarnFilesEvent;

fn recompile_loaded_yarn_files(
    yarn_files: Res<Assets<YarnFile>>,
    yarn_project: Option<ResMut<YarnProject>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
    mut events: ResMut<Messages<RecompileLoadedYarnFilesEvent>>,
) -> SystemResult {
    let Some(mut yarn_project) = yarn_project else {
        return Ok(());
    };
    let Some(compilation) = compile_yarn_files(
        &yarn_project.yarn_files,
        &yarn_files,
        yarn_project.localizations.as_ref(),
        yarn_project.development_file_generation,
    )?
    else {
        return Ok(());
    };
    let metadata = compilation
        .string_table
        .iter()
        .map(|(line_id, string_info)| (line_id.clone(), string_info.metadata.clone()))
        .collect();
    yarn_project.compilation = compilation;
    yarn_project.metadata = metadata;
    let program = yarn_project.compilation.program.clone().unwrap();
    for mut dialogue_runner in dialogue_runners.iter_mut() {
        let current_node = dialogue_runner.current_node();
        dialogue_runner
            .inner_mut()
            .0
            .replace_program(program.clone());
        dialogue_runner
            .text_provider
            .set_base_string_table(yarn_project.compilation.string_table.clone());
        if let Some(current_node) = current_node {
            dialogue_runner
                .stop()
                .try_start_node(current_node)
                .map(|_| ())
                .ok()
                .unwrap_or_else(|| {
                    dialogue_runner.start_node("Start");
                });
        }
    }
    events.clear();
    info!("Successfully recompiled Yarn project because of changes in Yarn files.");
    Ok(())
}

fn compile_loaded_yarn_files(
    mut commands: Commands,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    yarn_files: Res<Assets<YarnFile>>,
    mut update_strings_files_writer: MessageWriter<UpdateAllStringsFilesForStringTableEvent>,
    mut dirty: Local<bool>,
    yarn_project_config_to_load: Option<Res<YarnProjectConfigToLoad>>,
    asset_server: Res<AssetServer>,
    asset_root: Res<AssetRoot>,
) -> SystemResult {
    if yarn_files_being_loaded.is_changed() {
        *dirty = true;
    }
    if yarn_files_being_loaded.0.is_empty() {
        *dirty = false;
    }

    let all_files_finished_loading = || {
        yarn_files_being_loaded
            .0
            .iter()
            .all(|handle| yarn_files.contains(handle))
    };
    if !(*dirty && all_files_finished_loading()) {
        return Ok(());
    }

    let yarn_project_config_to_load = yarn_project_config_to_load.unwrap();
    let localizations = yarn_project_config_to_load
        .localizations
        .as_ref()
        .unwrap()
        .as_ref();
    let development_file_generation = yarn_project_config_to_load.development_file_generation;
    let Some(compilation) = compile_yarn_files(
        &yarn_files_being_loaded.0,
        &yarn_files,
        localizations,
        development_file_generation,
    )?
    else {
        return Ok(());
    };
    let file_count = yarn_files_being_loaded.0.len();

    if development_file_generation == DevelopmentFileGeneration::Full
        && let Some(localizations) = yarn_project_config_to_load.localizations.as_ref().unwrap()
    {
        update_strings_files_writer.write(UpdateAllStringsFilesForStringTableEvent(
            compilation.string_table.clone(),
        ));
        for localization in &localizations.translations {
            let path = localization.strings_file.as_path();
            let path = asset_root.0.join(path);

            if path.is_file() {
                continue;
            }
            let strings_file = StringsFile::from_string_table(
                localization.language.clone(),
                compilation.string_table.clone(),
            )
            .unwrap_or_default();

            strings_file.write_asset(&path)?;
            info!(
                "Generated \"{}\" (lang: {}).",
                path.display(),
                localization.language
            );
        }
    }

    let metadata = compilation
        .string_table
        .iter()
        .map(|(line_id, string_info)| (line_id.clone(), string_info.metadata.clone()))
        .collect();
    commands.insert_resource(YarnProject {
        yarn_files: std::mem::take(&mut yarn_files_being_loaded.0),
        compilation,
        localizations: yarn_project_config_to_load.localizations.clone().unwrap(),
        asset_server: SkipDebug(asset_server.clone()),
        watching_for_changes: yarn_project_config_to_load.watching_for_changes,
        development_file_generation,
        metadata,
    });

    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully compiled {file_count} Yarn {file_plural}");

    *dirty = false;
    Ok(())
}

fn clear_temp_yarn_project(mut commands: Commands) {
    // Done here instead of `compile_loaded_yarn_files` so that systems can access the global resources during the same frame
    commands.remove_resource::<YarnProjectConfigToLoad>();
}

fn compile_yarn_files(
    yarn_file_handles: &HashSet<Handle<YarnFile>>,
    yarn_files: &Res<Assets<YarnFile>>,
    localizations: Option<&Localizations>,
    development_file_generation: DevelopmentFileGeneration,
) -> Result<Option<Compilation>> {
    let yarn_files = yarn_file_handles
        .iter()
        .map(|handle| yarn_files.get(handle).unwrap());
    if localizations.is_some()
        && let Some(untagged_file) = yarn_files
            .clone()
            .find(|file| file.string_table.values().any(|v| v.is_implicit_tag))
    {
        if development_file_generation == DevelopmentFileGeneration::Full {
            info!(
                "Waiting with compilation until \"{}\" gets its line IDs generated",
                untagged_file.file.file_name
            );
            return Ok(None);
        } else {
            bail!(
                "Failed to compile Yarn files: Localization mode is on, but \"{}\" is not does not have full line IDs. \
                    Cannot generate the line IDs automatically either because we are not in `DevelopmentFileGeneration::Full`",
                untagged_file.file.file_name
            );
        }
    }
    let inner_yarn_files = yarn_files.map(|file| file.file.clone());
    let compilation = YarnCompiler::new().add_files(inner_yarn_files).compile()?;
    Ok(Some(compilation))
}
