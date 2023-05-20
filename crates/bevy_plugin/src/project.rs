use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<LoadedYarnFiles>()
        .register_type::<YarnFileSources>();
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct YarnFileSources(pub(crate) HashSet<YarnFileSource>);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct LoadedYarnFiles(pub(crate) HashSet<Handle<YarnFile>>);
impl LoadedYarnFiles {
    pub fn get(&self) -> &HashSet<Handle<YarnFile>> {
        &self.0
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
