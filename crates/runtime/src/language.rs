#[cfg(any(feature = "bevy", feature = "serde"))]
use crate::prelude::*;
use core::fmt::Display;
use icu_locid::LanguageIdentifier;

/// IETF BCP 47 code.
/// The default is "en-US".
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct Language(pub(crate) LanguageIdentifier);
impl Language {
    /// Creates a new `Language` from a string. Panics if the string is not a valid IETF BCP 47 code.
    pub fn new(language: impl Into<String>) -> Self {
        let language = language.into();
        Self(language.parse().unwrap())
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Default for Language {
    fn default() -> Self {
        "en-US".into()
    }
}

impl<T> From<T> for Language
where
    String: From<T>,
{
    fn from(language: T) -> Self {
        let language: String = language.into();
        Self::new(language)
    }
}
