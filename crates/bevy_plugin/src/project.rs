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
                    .pipe(error)
                    .run_if(resource_exists::<YarnFilesToLoad>()),
                recompile_loaded_yarn_files
                    .pipe(error)
                    .run_if(on_event::<RecompileLoadedYarnFilesEvent>()),
            )
                .chain(),
        );
}

#[derive(Debug, Resource)]
pub struct YarnProject {
    pub yarn_files: HashSet<Handle<YarnFile>>,
    pub(crate) compilation: Compilation,
    pub variable_storage: Box<dyn VariableStorage>,
    pub text_provider: Box<dyn TextProvider>,
    pub line_asset_provider: Option<Box<dyn LineAssetProvider>>,
    pub library: YarnFnLibrary,
    pub localizations: Option<Localizations>,
}

impl YarnProject {
    pub fn create_dialogue_runner(&self) -> DialogueRunner {
        DialogueRunnerBuilder::with_yarn_project(self).build()
    }

    pub fn build_dialogue_runner(&self) -> DialogueRunnerBuilder {
        DialogueRunnerBuilder::with_yarn_project(self)
    }

    pub fn set_text_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        self.text_provider.set_language_code(language.into());
        self
    }

    pub fn set_line_asset_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        if let Some(line_asset_provider) = self.line_asset_provider.as_mut() {
            line_asset_provider.set_language(language.into());
        }
        self
    }

    pub fn set_global_language(&mut self, language: impl Into<Option<Language>>) -> &mut Self {
        let language = language.into();
        self.set_text_language(language.clone())
            .set_line_asset_language(language)
    }

    pub fn text_language(&self) -> Option<Language> {
        self.text_provider.get_language_code()
    }

    pub fn line_asset_language(&self) -> Option<Language> {
        self.line_asset_provider
            .as_ref()
            .and_then(|p| p.get_language())
    }
}

#[derive(Debug, Resource)]
pub(crate) struct YarnProjectConfigToLoad {
    pub(crate) variable_storage: Option<Box<dyn VariableStorage>>,
    pub(crate) text_provider: Option<Box<dyn TextProvider>>,
    pub(crate) line_asset_provider: Option<Option<Box<dyn LineAssetProvider>>>,
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
    mut events: EventReader<RecompileLoadedYarnFilesEvent>,
    yarn_files: Res<Assets<YarnFile>>,
    yarn_project: Option<ResMut<YarnProject>>,
) -> SystemResult {
    events.clear();
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
    mut yarn_project_config_to_load: ResMut<YarnProjectConfigToLoad>,
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

    let Some(compilation) = compile_yarn_files(&yarn_files_being_loaded.0, &yarn_files, yarn_project_config_to_load.localizations())? else {
        return Ok(());
    };
    let file_count = yarn_files_being_loaded.0.len();

    if let Some(yarn_project) = yarn_project.as_mut() {
        yarn_project.compilation = compilation;
        yarn_files_being_loaded.0.clear();
    } else {
        if yarn_project_config_to_load
            .localizations()
            .map(|l| l.file_generation_mode == FileGenerationMode::Development)
            .unwrap_or_default()
        {
            update_strings_files_writer.send(UpdateAllStringsFilesForStringTableEvent(
                compilation.string_table.clone(),
            ));
            update_text_writer.send((&compilation.string_table).into())
        }
        commands.insert_resource(YarnProject {
            yarn_files: std::mem::take(&mut yarn_files_being_loaded.0),
            compilation,
            variable_storage: yarn_project_config_to_load.variable_storage.take().unwrap(),
            text_provider: yarn_project_config_to_load.text_provider.take().unwrap(),
            line_asset_provider: yarn_project_config_to_load
                .line_asset_provider
                .take()
                .unwrap(),
            library: yarn_project_config_to_load.library.take().unwrap(),
            localizations: yarn_project_config_to_load.localizations.take().unwrap(),
        });
    }

    let file_plural = if file_count == 1 { "file" } else { "files" };
    info!("Successfully compiled {file_count} yarn {file_plural}");

    *dirty = false;
    Ok(())
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
