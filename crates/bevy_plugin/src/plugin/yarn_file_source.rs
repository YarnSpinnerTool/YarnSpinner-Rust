use crate::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum YarnFileSource {
    Handle(Handle<YarnFile>),
    Path(String),
}

impl From<Handle<YarnFile>> for YarnFileSource {
    fn from(handle: Handle<YarnFile>) -> Self {
        Self::Handle(handle)
    }
}

impl From<String> for YarnFileSource {
    fn from(path: String) -> Self {
        Self::Path(path)
    }
}

impl From<&str> for YarnFileSource {
    fn from(path: &str) -> Self {
        Self::Path(path.to_owned())
    }
}

impl YarnFileSource {
    pub fn load(&self, asset_server: &AssetServer) -> Handle<YarnFile> {
        match self {
            Self::Handle(handle) => handle.clone(),
            Self::Path(path) => asset_server.load(path),
        }
    }
}
