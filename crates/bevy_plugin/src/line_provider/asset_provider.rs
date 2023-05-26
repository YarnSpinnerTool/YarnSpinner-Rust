use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::{Asset, HandleId};
use bevy::prelude::*;
use bevy::utils::HashSet;
pub use file_extension_asset_provider::FileExtensionAssetProvider;
use std::fmt::Debug;

mod file_extension_asset_provider;

pub(crate) fn asset_provider_plugin(app: &mut App) {
    app.fn_plugin(file_extension_asset_provider::file_extension_asset_provider_plugin);
}

pub trait AssetProvider: Debug + Send + Sync {
    fn get_language(&self) -> Option<Language>;
    fn set_language(&mut self, language: Option<Language>);
    fn are_assets_available(&self) -> bool;
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineAssets(HashSet<HandleUntyped>);
impl LineAssets {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn get_handle<T>(&self) -> Option<Handle<T>>
    where
        T: Asset,
    {
        self.0.iter().find_map(|handle| {
            if let HandleId::Id(type_uuid, _) = handle.id() {
                (T::TYPE_UUID == type_uuid).then(|| handle.clone().typed())
            } else {
                None
            }
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<HashSet<HandleUntyped>> for LineAssets {
    fn from(h: HashSet<HandleUntyped>) -> Self {
        Self(h)
    }
}

impl IntoIterator for LineAssets {
    type Item = <HashSet<HandleUntyped> as IntoIterator>::Item;
    type IntoIter = <HashSet<HandleUntyped> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<HandleUntyped> for LineAssets {
    fn extend<T: IntoIterator<Item = HandleUntyped>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}
