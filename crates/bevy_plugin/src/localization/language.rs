use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentLanguage(pub(crate) Language);
