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
