use crate::prelude::*;
use crate::UnderlyingYarnLine;
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};
pub use file_extension_asset_provider::FileExtensionAssetProvider;
use std::any::Any;
use std::fmt::Debug;

mod file_extension_asset_provider;

pub(crate) fn asset_provider_plugin(app: &mut App) {
    app.fn_plugin(file_extension_asset_provider::file_extension_asset_provider_plugin);
}

pub trait AssetProvider: Debug + Send + Sync {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn get_language(&self) -> Option<Language>;
    fn set_language(&mut self, language: Option<Language>);
    fn set_localizations(&mut self, localizations: Localizations);
    fn set_asset_server(&mut self, asset_server: AssetServer);
    fn are_assets_available(&self) -> bool;
    fn accept_line_hints(&mut self, line_ids: &[LineId]);
    fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineAssets(HashMap<Uuid, HandleUntyped>);
impl LineAssets {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn with_assets(handles: impl IntoIterator<Item = (Uuid, HandleUntyped)>) -> Self {
        Self(handles.into_iter().collect())
    }

    pub fn get_handle<T>(&self) -> Option<Handle<T>>
    where
        T: Asset,
    {
        self.0.iter().find_map(|(type_id, handle)| {
            (T::TYPE_UUID == *type_id).then(|| handle.clone().typed())
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<HashMap<Uuid, HandleUntyped>> for LineAssets {
    fn from(h: HashMap<Uuid, HandleUntyped>) -> Self {
        Self(h)
    }
}

impl IntoIterator for LineAssets {
    type Item = <HashMap<Uuid, HandleUntyped> as IntoIterator>::Item;
    type IntoIter = <HashMap<Uuid, HandleUntyped> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Extend<LineAssets> for LineAssets {
    fn extend<T: IntoIterator<Item = LineAssets>>(&mut self, iter: T) {
        self.0.extend(
            iter.into_iter()
                .flat_map(|line_assets| line_assets.0.into_iter()),
        )
    }
}

impl Extend<(Uuid, HandleUntyped)> for LineAssets {
    fn extend<T: IntoIterator<Item = (Uuid, HandleUntyped)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl FromIterator<(Uuid, HandleUntyped)> for LineAssets {
    fn from_iter<T: IntoIterator<Item = (Uuid, HandleUntyped)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter))
    }
}

impl FromIterator<LineAssets> for LineAssets {
    fn from_iter<T: IntoIterator<Item = LineAssets>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .flat_map(|line_assets| line_assets.0.into_iter())
                .collect(),
        )
    }
}
