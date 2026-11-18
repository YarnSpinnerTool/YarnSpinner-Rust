use std::path::{Path, PathBuf};

use crate::prelude::*;
use anyhow::ensure;
use bevy::{
    asset::{
        AssetLoader, AssetPath, LoadContext,
        io::{ErasedAssetReader, Reader, embedded::GetAssetServer},
    },
    platform::collections::HashSet,
    prelude::*,
    tasks::futures_lite::StreamExt,
};
use yarnspinner::compiler::{CURRENT_PROJECT_FILE_VERSION, Project};

/// A .yarnproject file.
#[derive(Debug, Clone, PartialEq, Reflect, Asset)]
#[reflect(Debug, PartialEq)]
pub struct YarnProjectFile {
    pub(crate) project: Project,
    #[dependency]
    pub(crate) files: HashSet<Handle<YarnFile>>,
}

impl YarnProjectFile {
    /// Gets the asset's underlying [`Project`]
    pub fn project(&self) -> &Project {
        &self.project
    }

    /// Gets all the associated [`YarnFile`]s for this project.
    pub fn files(&self) -> &HashSet<Handle<YarnFile>> {
        &self.files
    }
}

#[derive(TypePath)]
pub(crate) struct YarnProjectFileLoader {
    asset_server: AssetServer,
}
impl FromWorld for YarnProjectFileLoader {
    fn from_world(world: &mut World) -> Self {
        Self {
            asset_server: world.get_asset_server().clone(),
        }
    }
}

impl AssetLoader for YarnProjectFileLoader {
    type Asset = YarnProjectFile;
    type Settings = ();
    type Error = anyhow::Error;
    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let project: Project = serde_json::from_slice(&bytes)?;
        ensure!(
            project.project_file_version == CURRENT_PROJECT_FILE_VERSION,
            "Project file version must be {}",
            CURRENT_PROJECT_FILE_VERSION
        );
        let asset_path = load_context.path().clone();
        let project_dir = asset_path
            .path()
            .parent()
            .unwrap_or(Path::new(""))
            .to_path_buf();
        let source = self
            .asset_server
            .get_source(asset_path.source().clone_owned())?;

        let mut found = Vec::new();
        walk(source.reader(), &project_dir, &mut found).await?;

        let files = found
            .into_iter()
            .filter(|p| {
                p.strip_prefix(&project_dir)
                    .is_ok_and(|rel| project.is_matching_path_relative(rel))
            })
            .map(|p| {
                let path =
                    AssetPath::from_path_buf(p).with_source(asset_path.source().clone_owned());
                load_context.load::<YarnFile>(path)
            })
            .collect();

        Ok(YarnProjectFile {
            project: project.with_path(asset_path.path().to_path_buf()),
            files,
        })
    }
    fn extensions(&self) -> &[&str] {
        &["yarnproject"]
    }
}

async fn walk(
    reader: &dyn ErasedAssetReader,
    dir: &Path,
    out: &mut Vec<PathBuf>,
) -> Result<(), Error> {
    let mut stack = vec![dir.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let mut entries = reader.read_directory(&dir).await?;
        while let Some(path) = entries.next().await {
            if reader.is_directory(&path).await? {
                stack.push(path);
            } else {
                out.push(path);
            }
        }
    }
    Ok(())
}

pub fn plugin(app: &mut App) {
    app.init_asset::<YarnProjectFile>()
        .register_asset_reflect::<YarnProjectFile>()
        .init_asset_loader::<YarnProjectFileLoader>();
}
