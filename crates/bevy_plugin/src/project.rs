use crate::filesystem_events::UpdateAllStringsFilesForStringTableEvent;
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<YarnFilesInProject>()
        .register_type::<YarnFilesToLoad>()
        .init_resource::<YarnFilesInProject>()
        .init_resource::<YarnFilesToLoad>()
        .add_event::<RecompileLoadedYarnFilesEvent>()
        .add_systems(
            (
                add_yarn_files_to_load_queue
                    .run_if(resource_exists_and_changed::<YarnFilesToLoad>()),
                compile_loaded_yarn_files.pipe(error),
                recompile_loaded_yarn_files.pipe(error).run_if(
                    resource_exists::<YarnCompilation>()
                        .and_then(on_event::<RecompileLoadedYarnFilesEvent>()),
                ),
            )
                .chain(),
        );
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct YarnFilesToLoad(pub HashSet<YarnFileSource>);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct YarnFilesInProject(pub(crate) HashSet<Handle<YarnFile>>);
impl YarnFilesInProject {
    pub fn get(&self) -> &HashSet<Handle<YarnFile>> {
        &self.0
    }
}

impl AsRef<HashSet<Handle<YarnFile>>> for YarnFilesInProject {
    fn as_ref(&self) -> &HashSet<Handle<YarnFile>> {
        self.get()
    }
}

#[derive(Debug, Clone, PartialEq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct YarnCompilation(pub(crate) Compilation);
impl YarnCompilation {
    pub fn get(&self) -> &Compilation {
        &self.0
    }
}

impl AsRef<Compilation> for YarnCompilation {
    fn as_ref(&self) -> &Compilation {
        self.get()
    }
}

#[derive(Debug, Resource)]
pub struct GlobalVariableStorage(pub(crate) Box<dyn VariableStorage>);
impl GlobalVariableStorage {
    pub fn get(&self) -> &dyn VariableStorage {
        self.0.as_ref()
    }
}

#[derive(Debug, Resource)]
pub struct GlobalTextProvider(pub(crate) Box<dyn TextProvider>);
impl GlobalTextProvider {
    pub fn get(&self) -> &dyn TextProvider {
        self.0.as_ref()
    }
}

#[derive(Debug, Resource)]
pub struct GlobalLineAssetProvider(pub(crate) Box<dyn LineAssetProvider>);
impl GlobalLineAssetProvider {
    pub fn get(&self) -> &dyn LineAssetProvider {
        self.0.as_ref()
    }
}

fn add_yarn_files_to_load_queue(
    mut yarn_files_to_load: ResMut<YarnFilesToLoad>,
    mut yarn_files_in_project: ResMut<YarnFilesInProject>,
    asset_server: Res<AssetServer>,
) {
    if yarn_files_to_load.0.is_empty() {
        return;
    }
    let handles = yarn_files_to_load
        .0
        .drain()
        .map(|source| source.load(&asset_server));
    yarn_files_in_project.0.extend(handles);
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub struct RecompileLoadedYarnFilesEvent;

fn recompile_loaded_yarn_files(
    yarn_files_in_project: Res<YarnFilesInProject>,
    yarn_files: Res<Assets<YarnFile>>,
    localizations: Option<Res<Localizations>>,
    mut yarn_compilation: ResMut<YarnCompilation>,
) -> SystemResult {
    let Some(compilation) = compile_yarn_files(&yarn_files_in_project, yarn_files, localizations)? else {
        return Ok(());
    };
    yarn_compilation.0 = compilation;
    let file_plural = if yarn_files_in_project.0.len() == 1 {
        "file"
    } else {
        "files"
    };
    info!(
        "Successfully recompiled {} yarn {file_plural}",
        yarn_files_in_project.0.len()
    );
    Ok(())
}

fn compile_loaded_yarn_files(
    mut commands: Commands,
    yarn_files_in_project: Res<YarnFilesInProject>,
    mut yarn_compilation: Option<ResMut<YarnCompilation>>,
    yarn_files: Res<Assets<YarnFile>>,
    mut update_writer: EventWriter<UpdateAllStringsFilesForStringTableEvent>,
    mut dirty: Local<bool>,
    localizations: Option<Res<Localizations>>,
) -> SystemResult {
    if yarn_files_in_project.is_changed() {
        *dirty = true;
    }
    if !*dirty
        || yarn_files_in_project.0.is_empty()
        || !yarn_files_in_project
            .0
            .iter()
            .all(|handle| yarn_files.contains(handle))
    {
        return Ok(());
    }

    let Some(compilation) = compile_yarn_files(&yarn_files_in_project, yarn_files, localizations)? else {
        return Ok(());
    };

    if let Some(yarn_compilation) = yarn_compilation.as_mut() {
        yarn_compilation.0 = compilation;
    } else {
        update_writer.send(UpdateAllStringsFilesForStringTableEvent(
            compilation.string_table.clone(),
        ));
        commands.insert_resource(YarnCompilation(compilation));
    }

    let file_plural = if yarn_files_in_project.0.len() == 1 {
        "file"
    } else {
        "files"
    };
    info!(
        "Successfully compiled {} yarn {file_plural}",
        yarn_files_in_project.0.len()
    );

    *dirty = false;
    Ok(())
}

fn compile_yarn_files(
    yarn_files_in_project: &Res<YarnFilesInProject>,
    yarn_files: Res<Assets<YarnFile>>,
    localizations: Option<Res<Localizations>>,
) -> Result<Option<Compilation>> {
    let yarn_files = yarn_files_in_project
        .0
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
