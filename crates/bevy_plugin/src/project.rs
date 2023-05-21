use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<YarnFilesInProject>()
        .register_type::<YarnFilesToLoad>()
        .init_resource::<YarnCompilation>()
        .add_systems(
            (
                add_yarn_files_to_load_queue
                    .run_if(resource_exists_and_changed::<YarnFilesToLoad>()),
                compile_loaded_yarn_files.pipe(error),
                set_dialogue_programs,
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

fn compile_loaded_yarn_files(
    yarn_files_in_project: Res<YarnFilesInProject>,
    mut yarn_compilation: ResMut<YarnCompilation>,
    yarn_files: Res<Assets<YarnFile>>,
    mut dirty: Local<bool>,
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

    let yarn_files = yarn_files_in_project
        .0
        .iter()
        .map(|handle| yarn_files.get(handle).unwrap().file.clone());
    let compilation = YarnCompiler::new().add_files(yarn_files).compile()?;
    yarn_compilation.0 = compilation;
    *dirty = false;
    Ok(())
}

fn set_dialogue_programs(
    mut dialogue_runners: Query<&mut DialogueRunner>,
    global_variable_storage: Res<GlobalVariableStorage>,
    global_text_provider: Res<GlobalTextProvider>,
    global_line_asset_provider: Option<Res<GlobalLineAssetProvider>>,
    yarn_compilation: Res<YarnCompilation>,
) {
    let compilation_changed = yarn_compilation.is_changed();
    let dialogue_runners = dialogue_runners
        .iter_mut()
        .filter(|runner| compilation_changed || runner.dialogue.is_none());
    for mut dialogue_runner in dialogue_runners {
        let dialogue = if let Some(dialogue) = &mut dialogue_runner.dialogue {
            dialogue
        } else {
            let text_provider = dialogue_runner
                .text_provider_override
                .as_ref()
                .map(|provider| provider.clone_shallow())
                .unwrap_or_else(|| global_text_provider.0.clone_shallow());
            let variable_storage = dialogue_runner
                .variable_storage_override
                .as_ref()
                .map(|storage| storage.clone_shallow())
                .unwrap_or_else(|| global_variable_storage.0.clone_shallow());
            if dialogue_runner.line_asset_provider_override.is_none() {
                let line_asset_provider = global_line_asset_provider
                    .as_ref()
                    .map(|provider| provider.0.clone_shallow());
                dialogue_runner.line_asset_provider_override = Some(line_asset_provider);
            }
            dialogue_runner.dialogue = Some(Dialogue::new(variable_storage, text_provider));
            dialogue_runner.dialogue.as_mut().unwrap()
        };
        dialogue.replace_program(yarn_compilation.0.program.clone().unwrap());
    }
}
