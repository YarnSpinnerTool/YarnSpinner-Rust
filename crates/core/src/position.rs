use crate::prelude::*;

/// Represents a position in a multi-line string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect, FromReflect,))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy", reflect(Debug, PartialEq, Hash))]
#[cfg_attr(
    all(feature = "bevy", feature = "serde"),
    reflect(Serialize, Deserialize)
)]
pub struct Position {
    /// The zero-indexed line of this position.
    pub line: usize,

    /// The zero-indexed character number of this position.
    pub character: usize,
}
