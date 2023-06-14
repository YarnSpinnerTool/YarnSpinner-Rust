use crate::prelude::*;
use crate::UnderlyingYarnLine;
#[cfg(feature = "audio_assets")]
pub use audio_asset_provider_plugin::AudioAssetProvider;
use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::utils::{HashMap, Uuid};
pub use file_extension_asset_provider_plugin::{file_extensions, FileExtensionAssetProvider};
use std::any::Any;
use std::fmt::Debug;

#[cfg(feature = "audio_assets")]
mod audio_asset_provider_plugin;
mod file_extension_asset_provider_plugin;

pub(crate) fn asset_provider_plugin(app: &mut App) {
    app.fn_plugin(file_extension_asset_provider_plugin::file_extension_asset_provider_plugin);

    #[cfg(feature = "audio_assets")]
    app.fn_plugin(audio_asset_provider_plugin::audio_asset_provider_plugin);
}

/// Trait for providing assets for lines, e.g. audio files or character portraits.
/// If the `audio_assets` feature is enabled, you can use the bundled [`AudioAssetProvider`] struct to retrieve audio files.
/// You can also fetch assets in a similar way by using the [`FileExtensionAssetProvider`] struct.
pub trait AssetProvider: Debug + Send + Sync {
    /// Returns the type as a [`dyn Any`]. Used for polymorphism. Should be implemented like this:
    /// ```ignore
    /// fn as_any(&self) -> &dyn Any {
    ///    self
    /// }
    fn as_any(&self) -> &dyn Any;

    /// Returns the type as a mutable [`dyn Any`]. Used for polymorphism. Should be implemented like this:
    /// ```ignore
    /// fn as_any_mut(&mut self) -> &mut dyn Any {
    ///   self
    /// }
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Returns the [`Language`] that this [`AssetProvider`] is currently using. If there are no [`Localizations`] set, this returns [`None`].
    fn get_language(&self) -> Option<Language>;

    /// Sets the [`Language`] that this [`AssetProvider`] should use. If there are [`Localizations`] available, this should only be called with [`Some`].
    /// Since this method can be called by the user, implementors should check if the [`Language`] is available via [`Localizations::supports_language`] and panic if it isn't.
    fn set_language(&mut self, language: Option<Language>);

    /// Sets the available [`Localizations`].
    fn set_localizations(&mut self, localizations: Localizations);

    /// Sets the [`AssetServer`] used. This is a simple clone of the one found in the Bevy ECS [`World`].
    fn set_asset_server(&mut self, asset_server: AssetServer);

    /// Returns whether the assets for all lines announced by [`AssetProvider::accept_line_hints`] are available, i.e. have been loaded and are ready to be used.
    fn are_assets_available(&self) -> bool;

    /// Passes the [`LineId`]s that this [`AssetProvider`] should soon provide assets for. These are the [`LineId`]s that are contained in the current node and are not required to be actually reached.
    fn accept_line_hints(&mut self, line_ids: &[LineId]);

    /// Returns the [`LineAssets`] for the given [`UnderlyingYarnLine`]. Will only be called if [`AssetProvider::are_assets_available`] returns `true`,
    /// so an implementor is expected to panic if the assets are not available.
    fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets;
}

/// Assets that were provided by one or more [`AssetProvider`]s. Stores them in the form of [`Handle`]s.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LineAssets(HashMap<Uuid, HandleUntyped>);
impl LineAssets {
    /// Creates a new empty [`LineAssets`] struct.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a new [`LineAssets`] struct from an iterator of untyped [`Handle`]s and the [`Uuid`] of the [`Asset`] they reference.
    pub fn with_assets(handles: impl IntoIterator<Item = (Uuid, HandleUntyped)>) -> Self {
        Self(handles.into_iter().collect())
    }

    /// Gets the [`Handle`] for the given [`Asset`] type, if available.
    pub fn get_handle<T>(&self) -> Option<Handle<T>>
    where
        T: Asset,
    {
        self.0.iter().find_map(|(type_id, handle)| {
            (T::TYPE_UUID == *type_id).then(|| handle.clone().typed())
        })
    }

    /// Gets the number of assets provided.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns whether there are no assets provided.
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
