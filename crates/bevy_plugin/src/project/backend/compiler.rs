use crate::localization::LineIdUpdateSystemSet;
use crate::project::{CompilationStep, apply_compiled_file};
use crate::{
    fmt_utils::SkipDebug,
    localization::UpdateAllStringsFilesForStringTableEvent,
    plugin::AssetRoot,
    prelude::*,
    project::{YarnFilesBeingLoaded, YarnProjectConfigToLoad},
};
use anyhow::bail;
use bevy::{platform::collections::HashSet, prelude::*};

pub(crate) fn plugin(app: &mut App) {
    app.add_message::<RecompileLoadedYarnFilesEvent>()
        .add_systems(
            Update,
            (
                compile_loaded_yarn_files.pipe(panic_on_err),
                recompile_loaded_yarn_files
                    .pipe(log_error)
                    .run_if(events_in_queue::<RecompileLoadedYarnFilesEvent>()),
            )
                .chain()
                .after(LineIdUpdateSystemSet)
                .in_set(CompilationStep::Compile),
        );
}

pub(crate) fn generate_yarn_project(
    yarn_files_being_loaded: &mut YarnFilesBeingLoaded,
    yarn_project_config_to_load: &YarnProjectConfigToLoad,
    yarn_files: &Res<Assets<YarnFile>>,
    localizations: Option<&Localizations>,
    development_file_generation: DevelopmentFileGeneration,
    update_strings_files_writer: &mut MessageWriter<UpdateAllStringsFilesForStringTableEvent>,
    asset_root: &AssetRoot,
    asset_server: &AssetServer,
) -> anyhow::Result<Option<YarnProject>> {
    let Some(compilation) = compile_yarn_files(
        &yarn_files_being_loaded.0,
        yarn_files,
        localizations,
        development_file_generation,
    )?
    else {
        return Ok(None);
    };
    let file_count = yarn_files_being_loaded.0.len();

    // generates metadata tags inline.
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

    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully compiled {file_count} Yarn {file_plural}");

    Ok(Some(YarnProject {
        yarn_files: std::mem::take(&mut yarn_files_being_loaded.0),
        compilation: CompiledFile::from_compilation(compilation),
        localizations: yarn_project_config_to_load.localizations.clone().unwrap(),
        asset_server: SkipDebug(asset_server.clone()),
        watching_for_changes: yarn_project_config_to_load.watching_for_changes,
        development_file_generation,
    }))
}

pub(crate) fn compile_yarn_files(
    yarn_file_handles: &HashSet<Handle<YarnFile>>,
    yarn_files: &Res<Assets<YarnFile>>,
    localizations: Option<&Localizations>,
    development_file_generation: DevelopmentFileGeneration,
) -> anyhow::Result<Option<Compilation>> {
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

    apply_compiled_file(
        &mut yarn_project,
        CompiledFile::from_compilation(compilation),
        &mut dialogue_runners,
    );
    events.clear();
    info!("Successfully recompiled Yarn project because of changes in Yarn files.");
    Ok(())
}

/// Compiles the loaded yarn files, then emits [`UpdateAllStringsFilesForStringTableEvent`]
/// and inserts the [`YarnProject`] resource.
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
    // guard clause: early exit if we don't need to update anything
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

    // load the project config
    let yarn_project_config_to_load = yarn_project_config_to_load.unwrap();
    let localizations = yarn_project_config_to_load
        .localizations
        .as_ref()
        .unwrap()
        .as_ref();
    let development_file_generation = yarn_project_config_to_load.development_file_generation;

    // TODO: Modify this when we support multiple projects.
    let res = generate_yarn_project(
        &mut yarn_files_being_loaded,
        &yarn_project_config_to_load,
        &yarn_files,
        localizations,
        development_file_generation,
        &mut update_strings_files_writer,
        &asset_root,
        &asset_server,
    )?;

    if let Some(yarn_project) = res {
        commands.insert_resource(yarn_project);
    } else {
        return Ok(());
    }

    *dirty = false;
    Ok(())
}
