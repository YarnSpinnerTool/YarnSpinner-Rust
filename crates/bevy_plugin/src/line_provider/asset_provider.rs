use crate::prelude::*;
use crate::UnderlyingYarnLine;
#[cfg(feature = "audio_assets")]
pub use audio_asset_provider_plugin::AudioAssetProvider;
use bevy::asset::{Asset, LoadedUntypedAsset};
use bevy::prelude::*;
use bevy::utils::HashMap;
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
    /// ```
    /// # use std::any::Any;
    /// # use bevy::asset::AssetServer;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # use bevy_yarn_slinger::UnderlyingYarnLine;
    /// # #[derive(Debug)]
    /// # struct Foo;
    /// # impl AssetProvider for Foo {
    /// fn as_any(&self) -> &dyn Any {
    ///    self
    /// }
    /// #
    /// # fn as_any_mut(&mut self) -> &mut dyn Any {
    /// #        unreachable!()
    /// #    }
    /// #
    /// # fn get_language(&self) -> Option<Language> {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_language(&mut self, language: Option<Language>) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_localizations(&mut self, localizations: Localizations) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_asset_server(&mut self, asset_server: AssetServer) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn are_assets_available(&self) -> bool {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn accept_line_hints(&mut self, line_ids: &[LineId]) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets {
    /// #          unreachable!()
    /// #      }
    /// # }
    /// ```
    fn as_any(&self) -> &dyn Any;

    /// Returns the type as a mutable [`dyn Any`]. Used for polymorphism. Should be implemented like this:
    /// ```
    /// # use std::any::Any;
    /// # use bevy::asset::AssetServer;
    /// # use bevy_yarn_slinger::prelude::*;
    /// # use bevy_yarn_slinger::UnderlyingYarnLine;
    /// # #[derive(Debug)]
    /// # struct Foo;
    /// # impl AssetProvider for Foo {
    /// # fn as_any(&self) -> &dyn Any {
    /// #    self
    /// # }
    /// #
    /// fn as_any_mut(&mut self) -> &mut dyn Any {
    ///     self
    /// }
    /// #
    /// # fn get_language(&self) -> Option<Language> {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_language(&mut self, language: Option<Language>) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_localizations(&mut self, localizations: Localizations) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn set_asset_server(&mut self, asset_server: AssetServer) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn are_assets_available(&self) -> bool {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn accept_line_hints(&mut self, line_ids: &[LineId]) {
    /// #          unreachable!()
    /// #      }
    /// #
    /// #  fn get_assets(&self, line: &UnderlyingYarnLine) -> LineAssets {
    /// #          unreachable!()
    /// #      }
    /// # }
    /// ```
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
pub struct LineAssets(HashMap<&'static str, Handle<LoadedUntypedAsset>>);
impl LineAssets {
    /// Creates a new empty [`LineAssets`] struct.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Creates a new [`LineAssets`] struct from an iterator of untyped [`Handle`]s and the [`TypePath::type_path`] of the [`Asset`] they reference.
    pub fn with_assets(
        handles: impl IntoIterator<Item = (&'static str, Handle<LoadedUntypedAsset>)>,
    ) -> Self {
        Self(handles.into_iter().collect())
    }

    /// Gets the [`Handle`] for the given [`Asset`] type, if available.
    pub fn get_handle<T>(&self) -> Option<Handle<T>>
    where
        T: Asset,
    {
        self.0.iter().find_map(|(type_id, handle)| {
            (T::type_path() == *type_id).then(|| handle.clone().untyped().typed())
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

impl From<HashMap<&'static str, Handle<LoadedUntypedAsset>>> for LineAssets {
    fn from(h: HashMap<&'static str, Handle<LoadedUntypedAsset>>) -> Self {
        Self(h)
    }
}

impl IntoIterator for LineAssets {
    type Item = <HashMap<&'static str, Handle<LoadedUntypedAsset>> as IntoIterator>::Item;
    type IntoIter = <HashMap<&'static str, Handle<LoadedUntypedAsset>> as IntoIterator>::IntoIter;

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

impl Extend<(&'static str, Handle<LoadedUntypedAsset>)> for LineAssets {
    fn extend<T: IntoIterator<Item = (&'static str, Handle<LoadedUntypedAsset>)>>(
        &mut self,
        iter: T,
    ) {
        self.0.extend(iter)
    }
}

impl FromIterator<(&'static str, Handle<LoadedUntypedAsset>)> for LineAssets {
    fn from_iter<T: IntoIterator<Item = (&'static str, Handle<LoadedUntypedAsset>)>>(
        iter: T,
    ) -> Self {
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
