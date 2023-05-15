use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineId(pub String);

impl From<String> for LineId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for LineId {
    fn from(s: &str) -> Self {
        Self(s.to_owned())
    }
}

impl Display for LineId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}