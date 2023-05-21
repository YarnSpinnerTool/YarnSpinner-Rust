use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<YarnFilesInProject>()
        .register_type::<YarnFilesToLoad>()
        .init_resource::<YarnCompilation>()
        .init_resource::<YarnFilesInProject>()
        .init_resource::<YarnFilesToLoad>()
        .add_systems(
            (
                add_yarn_files_to_load_queue
                    .run_if(resource_exists_and_changed::<YarnFilesToLoad>()),
                compile_loaded_yarn_files.pipe(error),
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
