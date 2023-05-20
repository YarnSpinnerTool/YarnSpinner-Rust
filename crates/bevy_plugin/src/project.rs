use crate::prelude::*;
use bevy::prelude::*;
use bevy::utils::HashSet;

pub(crate) fn project_plugin(app: &mut App) {
    app.register_type::<CompiledYarnFiles>();
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct CompiledYarnFiles(pub HashSet<Handle<YarnFile>>);

#[derive(Debug, Resource)]
pub struct GlobalVariableStorage(pub Box<dyn VariableStorage>);

#[derive(Debug, Resource)]
pub struct GlobalTextProvider(pub Box<dyn TextProvider>);

#[derive(Debug, Resource)]
pub struct GlobalLineAssetProvider(pub Box<dyn LineAssetProvider>);
