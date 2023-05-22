use crate::prelude::*;
use bevy::prelude::*;

pub(crate) fn language_plugin(app: &mut App) {
    app.register_type::<CurrentLanguage>();
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub struct CurrentLanguage(pub Language);
