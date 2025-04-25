#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use core::fmt::Display;

/// The unique ID of a line in a Yarn script. In a Yarn script, line IDs look like this:
/// ```text
/// Darth Vader: I am your father! #line:123
/// Luke: Noooooo #line:nooooo
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
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

impl AsRef<str> for LineId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Display for LineId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.0.fmt(f)
    }
}
