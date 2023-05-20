#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect,))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct LineId(pub String);

impl<T> From<T> for LineId
where
    String: From<T>,
{
    fn from(s: T) -> Self {
        Self(s.into())
    }
}

impl Display for LineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
