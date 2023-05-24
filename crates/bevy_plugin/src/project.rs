use crate::localization::{
    UpdateAllStringsFilesForStringTableEvent, UpdateBaseLanguageTextProviderForStringTableEvent,
};
use crate::prelude::*;
use anyhow::bail;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<YarnFilesToLoad>()
        .init_resource::<YarnFilesToLoad>()
        .init_resource::<YarnFilesBeingLoaded>()
        .add_event::<RecompileLoadedYarnFilesEvent>()
        .add_systems(
            (
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

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, SystemSet)]
pub(crate) struct CompilationSystemSet;

#[derive(Debug, Resource)]
pub struct YarnProject {
    pub(crate) yarn_files: HashSet<Handle<YarnFile>>,
    pub(crate) compilation: Compilation,
    pub(crate) variable_storage: Box<dyn VariableStorage>,
    pub(crate) text_provider: Box<dyn TextProvider>,
    pub(crate) asset_provider: Option<Box<dyn AssetProvider>>,
    pub(crate) library: YarnFnLibrary,
    pub localizations: Option<Localizations>,
}

impl YarnProject {
    pub fn create_dialogue_runner(&self) -> DialogueRunner {
        DialogueRunnerBuilder::with_yarn_project(self).build()
    }

    pub fn build_dialogue_runner(&self) -> DialogueRunnerBuilder {
        DialogueRunnerBuilder::with_yarn_project(self)
    }

    pub fn compilation(&self) -> &Compilation {
        &self.compilation
    }

    pub fn yarn_files(&self) -> impl Iterator<Item = &Handle<YarnFile>> {
        self.yarn_files.iter()
    }
}

impl DialogueConfigurator for YarnProject {
    fn text_provider(&self) -> &dyn TextProvider {
        self.text_provider.as_ref()
    }

    fn text_provider_mut(&mut self) -> &mut dyn TextProvider {
        self.text_provider.as_mut()
    }

    fn asset_provider(&self) -> Option<&dyn AssetProvider> {
        self.asset_provider.as_deref()
    }

    fn asset_provider_mut(&mut self) -> Option<&mut dyn AssetProvider> {
        // Source: <https://stackoverflow.com/a/55866511/5903309>
        self.asset_provider
            .as_mut()
            .map(|x| &mut **x as &mut dyn AssetProvider)
    }

    fn variable_storage(&self) -> &dyn VariableStorage {
        self.variable_storage.as_ref()
    }

    fn variable_storage_mut(&mut self) -> &mut dyn VariableStorage {
        self.variable_storage.as_mut()
    }

    fn library(&self) -> &YarnFnLibrary {
        &self.library
    }

    fn library_mut(&mut self) -> &mut YarnFnLibrary {
        &mut self.library
    }
}

#[derive(Debug, Resource)]
pub(crate) struct YarnProjectConfigToLoad {
    pub(crate) variable_storage: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider: Option<Box<dyn TextProvider>>,
    pub(crate) asset_provider: Option<Option<Box<dyn AssetProvider>>>,
    pub(crate) library: Option<YarnFnLibrary>,
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

fn add_yarn_files_to_load_queue(
    mut yarn_files_to_load: ResMut<YarnFilesToLoad>,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    asset_server: Res<AssetServer>,
) {
    if yarn_files_to_load.0.is_empty() {
        return;
    }
    let handles = yarn_files_to_load
        .0
        .drain()
        .map(|source| source.load(&asset_server));
    yarn_files_being_loaded.0.extend(handles);
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Reflect, FromReflect)]
#[reflect(Debug, Default, PartialEq)]
pub struct RecompileLoadedYarnFilesEvent;

fn recompile_loaded_yarn_files(
    yarn_files: Res<Assets<YarnFile>>,
    yarn_project: Option<ResMut<YarnProject>>,
) -> SystemResult {
    let Some(mut yarn_project) = yarn_project else {
        return Ok(());
    };
    let Some(compilation) = compile_yarn_files(&yarn_project.yarn_files, &yarn_files, yarn_project.localizations.as_ref())? else {
        return Ok(());
    };
    yarn_project.compilation = compilation;
    let file_count = yarn_project.yarn_files.len();
    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully recompiled {file_count} yarn {file_plural}");
    Ok(())
}

fn compile_loaded_yarn_files(
    mut commands: Commands,
    mut yarn_files_being_loaded: ResMut<YarnFilesBeingLoaded>,
    mut yarn_project: Option<ResMut<YarnProject>>,
    yarn_files: Res<Assets<YarnFile>>,
    mut update_strings_files_writer: EventWriter<UpdateAllStringsFilesForStringTableEvent>,
    mut update_text_writer: EventWriter<UpdateBaseLanguageTextProviderForStringTableEvent>,
    mut dirty: Local<bool>,
    yarn_project_config_to_load: Option<Res<YarnProjectConfigToLoad>>,
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

    let localizations = yarn_project
        .as_ref()
        .map(|p| p.localizations.as_ref())
        .unwrap_or_else(|| {
            yarn_project_config_to_load
                .as_ref()
                .map(|c| c.localizations.as_ref().unwrap().as_ref())
                .unwrap()
        });
    let Some(compilation) = compile_yarn_files(&yarn_files_being_loaded.0, &yarn_files, localizations)? else {
        return Ok(());
    };
    let file_count = yarn_files_being_loaded.0.len();

    update_text_writer.send((&compilation.string_table).into());
    if let Some(yarn_project) = yarn_project.as_mut() {
        yarn_project.compilation = compilation;
        yarn_files_being_loaded.0.clear();
    } else {
        let yarn_project_config_to_load = yarn_project_config_to_load.as_ref().unwrap();
        if yarn_project_config_to_load
            .localizations()
            .map(|l| l.file_generation_mode == FileGenerationMode::Development)
            .unwrap_or_default()
        {
            update_strings_files_writer.send(UpdateAllStringsFilesForStringTableEvent(
                compilation.string_table.clone(),
            ));
        }
        commands.insert_resource(YarnProject {
            yarn_files: std::mem::take(&mut yarn_files_being_loaded.0),
            compilation,
            variable_storage: yarn_project_config_to_load
                .variable_storage
                .clone()
                .unwrap(),
            text_provider: yarn_project_config_to_load.text_provider.clone().unwrap(),
            asset_provider: yarn_project_config_to_load.asset_provider.clone().unwrap(),
            library: yarn_project_config_to_load.library.clone().unwrap(),
            localizations: yarn_project_config_to_load.localizations.clone().unwrap(),
        });
    }

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
