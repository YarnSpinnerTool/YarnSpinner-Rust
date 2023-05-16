#[cfg(all(feature = "bevy", feature = "serde"))]
pub(crate) use bevy::prelude::{ReflectDeserialize, ReflectSerialize};
#[cfg(feature = "bevy")]
pub(crate) use bevy::reflect::{std_traits::ReflectDefault, FromReflect, Reflect};
#[cfg(feature = "serde")]
pub(crate) use serde::{Deserialize, Serialize};
