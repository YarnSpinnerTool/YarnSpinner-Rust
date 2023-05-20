use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Display;

/// IETF BCP 47 code.
/// The default is "en-US".
#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Language(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Default, Resource, Reflect, FromReflect)]
#[reflect(Debug, Resource, Default, PartialEq)]
pub(crate) struct CurrentLanguage(pub(crate) Language);

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
        Self(language.into())
    }
}

impl AsRef<str> for Language {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
