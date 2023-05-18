use crate::prelude::*;
use bevy::prelude::*;
use std::fmt::Display;

/// IETF BCP 47 code.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Debug, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Language(pub String);

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

impl From<String> for Language {
    fn from(language: String) -> Self {
        Self(language)
    }
}

impl From<&str> for Language {
    fn from(language: &str) -> Self {
        Self(language.to_owned())
    }
}
