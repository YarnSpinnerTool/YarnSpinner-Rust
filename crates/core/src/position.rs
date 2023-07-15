#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;

/// Represents a position in a multi-line string.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect,))]
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
    /// Careful: This represents a unicode code point, not a byte, i.e. what you'd get with `string.chars().nth(character)`.
    pub character: usize,
}
