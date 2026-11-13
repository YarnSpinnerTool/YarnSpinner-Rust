use crate::prelude::*;
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use yarnspinner::core::{YscLines as YscLinesInner, YscMetadata as YscMetadataInner};

/// A compiled Yarn file, `*.yarnc`.
#[derive(Debug, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Serialize, Deserialize)]
pub struct YscProgram(pub(crate) YarnProgram);

/// Line data for a compiled Yarn file, `*-Lines.csv`
#[derive(Debug, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Serialize, Deserialize)]
pub struct YscLines(pub(crate) YscLinesInner);

/// Metadata for a compiled Yarn file, `*-Metadata.csv`
#[derive(Debug, Clone, PartialEq, Reflect, Asset, Serialize, Deserialize)]
#[reflect(Debug, PartialEq, Serialize, Deserialize)]
pub struct YscMetadata(pub(crate) YscMetadataInner);

macro_rules! add_loader {
    ($name:ident, $outer:ident, $inner:ty, $ext:expr, $doc:literal) => {
        #[doc=$doc]
        #[derive(Debug, Default, TypePath)]
        pub struct $name;
        impl AssetLoader for $name {
            type Asset = $outer;
            // TODO: Possibly specify ysc version here?
            type Settings = ();
            type Error = anyhow::Error;

            async fn load(
                &self,
                reader: &mut dyn Reader,
                _settings: &(),
                _load_context: &mut LoadContext<'_>,
            ) -> Result<Self::Asset, Self::Error> {
                let mut bytes = Vec::new();
                reader.read_to_end(&mut bytes).await?;
                let file = <$inner>::from_bytes(&bytes)?;
                Ok($outer(file))
            }

            fn extensions(&self) -> &[&str] {
                &$ext
            }
        }
    };
}

add_loader!(
    YscProgramLoader,
    YscProgram,
    YarnProgram,
    ["yarnc"],
    "[AssetLoader] for a [YscProgram]."
);
// These are CSV files, but they're notated with a dash instead of a dot,
// e.g. `-Lines.csv`. Can't claim the csv extension so we leave it blank.
add_loader!(
    YscLinesLoader,
    YscLines,
    YscLinesInner,
    [],
    "[AssetLoader] for a [YscLines] file."
);
add_loader!(
    YscMetadataLoader,
    YscMetadata,
    YscMetadataInner,
    [],
    "[AssetLoader] for a [YscMetadata] file."
);

pub(crate) fn plugin(app: &mut App) {
    app.init_asset::<YscProgram>()
        .register_asset_reflect::<YscProgram>()
        .init_asset_loader::<YscProgramLoader>();
    app.init_asset::<YscLines>()
        .register_asset_reflect::<YscLines>()
        .init_asset_loader::<YscLinesLoader>();
    app.init_asset::<YscMetadata>()
        .register_asset_reflect::<YscMetadata>()
        .init_asset_loader::<YscMetadataLoader>();
}
