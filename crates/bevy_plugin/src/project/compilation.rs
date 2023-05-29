use crate::localization::UpdateAllStringsFilesForStringTableEvent;
use crate::prelude::*;
use crate::project::{CompilationSystemSet, LoadYarnProjectEvent};
use anyhow::bail;
use bevy::prelude::*;
use bevy::utils::HashSet;
use std::fmt::Debug;

pub(crate) fn project_compilation_plugin(app: &mut App) {
    app.register_type::<YarnFilesToLoad>()
        .init_resource::<YarnFilesToLoad>()
        .init_resource::<YarnFilesBeingLoaded>()
        .add_event::<RecompileLoadedYarnFilesEvent>()
        .add_systems(
            (
                load_project.pipe(panic_on_err),
                add_yarn_files_to_load_queue
                    .run_if(resource_exists_and_changed::<YarnFilesToLoad>()),
                compile_loaded_yarn_files
                    .pipe(panic_on_err)
                    .run_if(resource_exists::<YarnFilesToLoad>()),
                recompile_loaded_yarn_files
                    .pipe(error)
                    .run_if(on_event::<RecompileLoadedYarnFilesEvent>()),
                clear_temp_yarn_project.run_if(resource_added::<YarnProject>()),
            )
                .chain()
                .in_set(CompilationSystemSet),
        );
}

#[derive(Debug, Resource)]
pub(crate) struct YarnProjectConfigToLoad {
    pub(crate) localizations: Option<Option<Localizations>>,
}

impl YarnProjectConfigToLoad {
    pub(crate) fn localizations(&self) -> Option<&Localizations> {
        self.localizations.as_ref().unwrap().as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct YarnFilesToLoad(pub(crate) HashSet<YarnFileSource>);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct YarnFilesBeingLoaded(pub(crate) HashSet<Handle<YarnFile>>);

fn load_project(
    mut commands: Commands,
    mut events: ResMut<Events<LoadYarnProjectEvent>>,
    mut already_loaded: Local<bool>,
) -> SystemResult {
    for event in events.drain() {
        if *already_loaded {
            bail!("Yarn project already loaded. Sending multiple LoadYarnProjectEvent is not allowed.");
        }
        commands.insert_resource(YarnProjectConfigToLoad {
            localizations: Some(event.localizations),
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
) {
    if yarn_files_to_load.0.is_empty() {
        return;
    }
    let handles = yarn_files_to_load
        .0
        .drain()
        .map(|source| source.load(&asset_server, &mut assets));
    yarn_files_being_loaded.0.extend(handles);
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub(crate) struct RecompileLoadedYarnFilesEvent;

fn recompile_loaded_yarn_files(
    yarn_files: Res<Assets<YarnFile>>,
    yarn_project: Option<ResMut<YarnProject>>,
    mut dialogue_runners: Query<&mut DialogueRunner>,
) -> SystemResult {
    let Some(mut yarn_project) = yarn_project else {
        return Ok(());
    };
    let Some(compilation) = compile_yarn_files(&yarn_project.yarn_files, &yarn_files, yarn_project.localizations.as_ref())? else {
        return Ok(());
    };
    yarn_project.compilation = compilation;
    for mut dialogue_runner in dialogue_runners.iter_mut() {
        dialogue_runner
            .data_providers_mut()
            .text_provider_mut()
            .set_base_string_table(yarn_project.compilation.string_table.clone());
    }
    let file_count = yarn_project.yarn_files.len();
    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully recompiled {file_count} yarn {file_plural}");
    Ok(())
}

fn compile_loaded_yarn_files(
    mut commands: Commands,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    yarn_files: Res<Assets<YarnFile>>,
    mut update_strings_files_writer: EventWriter<UpdateAllStringsFilesForStringTableEvent>,
    mut dirty: Local<bool>,
    yarn_project_config_to_load: Option<Res<YarnProjectConfigToLoad>>,
    asset_server: Res<AssetServer>,
) -> SystemResult {
    if yarn_files_being_loaded.is_changed() {
        *dirty = true;
    }
    if yarn_files_being_loaded.0.is_empty() {
        *dirty = false;
    }
    if !*dirty
        || !yarn_files_being_loaded
            .0
            .iter()
            .all(|handle| yarn_files.contains(handle))
    {
        return Ok(());
    }

    let localizations = yarn_project_config_to_load
        .as_ref()
        .map(|c| c.localizations.as_ref().unwrap().as_ref())
        .unwrap();
    let Some(compilation) = compile_yarn_files(&yarn_files_being_loaded.0, &yarn_files, localizations)? else {
        return Ok(());
    };
    let file_count = yarn_files_being_loaded.0.len();

    let yarn_project_config_to_load = yarn_project_config_to_load.as_ref().unwrap();
    if yarn_project_config_to_load
        .localizations()
        .map(|l| l.file_generation_mode == FileGenerationMode::Development)
        .unwrap_or_default()
    {
        update_strings_files_writer.send(UpdateAllStringsFilesForStringTableEvent(
            compilation.string_table.clone(),
        ));
        if let Some(localizations) = yarn_project_config_to_load.localizations.as_ref().unwrap() {
            for localization in &localizations.translations {
                let path = localization.strings_file.as_path();
                if asset_server.asset_io().is_file(path) {
                    return Ok(());
                }
                let strings_file = StringsFile::from_string_table(
                    localization.language.clone(),
                    &compilation.string_table,
                )
                .unwrap_or_default();

                strings_file.write_asset(&asset_server, path)?;
                info!(
                    "Generated \"{}\" (lang: {}).",
                    path.display(),
                    localization.language
                );
            }
        }
    }

    commands.insert_resource(YarnProject {
        yarn_files: std::mem::take(&mut yarn_files_being_loaded.0),
        compilation,
        localizations: yarn_project_config_to_load.localizations.clone().unwrap(),
        asset_server: asset_server.clone(),
    });

    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully compiled {file_count} yarn {file_plural}");

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
) -> Result<Option<Compilation>> {
    let yarn_files = yarn_file_handles
        .iter()
        .map(|handle| yarn_files.get(handle).unwrap());
    if let Some(localizations) = localizations.as_ref() {
        if let Some(untagged_file) = yarn_files
            .clone()
            .find(|file| file.string_table.values().any(|v| v.is_implicit_tag))
        {
            if localizations.file_generation_mode == FileGenerationMode::Development {
                info!(
                    "Waiting with compilation until \"{}\" is automatically tagged",
                    untagged_file.file.file_name
                );
                return Ok(None);
            } else {
                bail!("Failed to compile yarn files: Localization mode is on, but \"{}\" is not tagged. \
                    Cannot tag it automatically either because we are not in `FileGenerationMode::Development`",
                    untagged_file.file.file_name);
            }
        }
    }
    let inner_yarn_files = yarn_files.map(|file| file.file.clone());
    let compilation = YarnCompiler::new().add_files(inner_yarn_files).compile()?;
    Ok(Some(compilation))
}
