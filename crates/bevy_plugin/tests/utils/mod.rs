#![allow(dead_code)]

use bevy::asset::LoadedUntypedAsset;
#[cfg(feature = "audio_assets")]
use bevy::audio::AudioPlugin;
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;
use std::path::{Path, PathBuf};

pub mod assertion;

pub mod prelude {
    #[allow(unused_imports)] // False positive
    pub use super::{assertion::*, *};
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    #[must_use]
    fn load_project_mut(&mut self) -> Mut<YarnProject>;

    fn load_lines(&mut self) -> &mut App;

    fn continue_dialogue_and_update(&mut self) -> &mut App;
    fn continue_dialogue_and_update_n_times(&mut self, n: usize) -> &mut App;

    #[must_use]
    fn create_dialogue_runner(&mut self) -> Entity;

    #[must_use]
    fn existing_dialogue_runner(&self, entity: Entity) -> &DialogueRunner;

    #[must_use]
    fn dialogue_runner(&mut self) -> &DialogueRunner;
    #[must_use]
    fn dialogue_runner_mut(&mut self) -> Mut<DialogueRunner>;
    #[must_use]
    fn try_dialogue_runner(&self) -> Option<&DialogueRunner>;
    #[must_use]
    fn try_dialogue_runner_mut(&mut self) -> Option<Mut<DialogueRunner>>;
    fn setup_default_plugins(&mut self) -> &mut App;
    fn setup_default_plugins_for_path(&mut self, asset_folder: impl AsRef<Path>) -> &mut App;
}

impl AppExt for App {
    fn load_project(&mut self) -> &YarnProject {
        while !self.world.contains_resource::<YarnProject>() {
            self.update();
        }
        self.world.resource::<YarnProject>()
    }

    fn load_project_mut(&mut self) -> Mut<YarnProject> {
        while !self.world.contains_resource::<YarnProject>() {
            self.update();
        }
        self.world.resource_mut::<YarnProject>()
    }

    fn load_lines(&mut self) -> &mut App {
        self.load_project();
        while !self.dialogue_runner().are_lines_available() {
            self.update();
        }
        self
    }

    fn continue_dialogue_and_update(&mut self) -> &mut App {
        self.continue_dialogue_and_update_n_times(1)
    }

    fn continue_dialogue_and_update_n_times(&mut self, n: usize) -> &mut App {
        for _ in 0..n {
            self.dialogue_runner_mut().continue_in_next_update();
            self.update();
        }
        self
    }

    fn create_dialogue_runner(&mut self) -> Entity {
        let project = self.load_project();
        let dialogue_runner = project.create_dialogue_runner();
        self.world.spawn(dialogue_runner).id()
    }

    fn existing_dialogue_runner(&self, entity: Entity) -> &DialogueRunner {
        self.world.get::<DialogueRunner>(entity).unwrap()
    }

    fn dialogue_runner(&mut self) -> &DialogueRunner {
        if self.try_dialogue_runner().is_some() {
            self.try_dialogue_runner().unwrap()
        } else {
            let entity = self.create_dialogue_runner();
            self.world.get::<DialogueRunner>(entity).unwrap()
        }
    }

    fn dialogue_runner_mut(&mut self) -> Mut<DialogueRunner> {
        if self.try_dialogue_runner().is_some() {
            self.try_dialogue_runner_mut().unwrap()
        } else {
            let entity = self.create_dialogue_runner();
            self.world.get_mut::<DialogueRunner>(entity).unwrap()
        }
    }

    fn try_dialogue_runner(&self) -> Option<&DialogueRunner> {
        self.world
            .iter_entities()
            .filter_map(|e| self.world.get::<DialogueRunner>(e.id()))
            .next()
    }

    fn try_dialogue_runner_mut(&mut self) -> Option<Mut<DialogueRunner>> {
        self.world
            .query::<&mut DialogueRunner>()
            .iter_mut(&mut self.world)
            .next()
    }

    fn setup_default_plugins(&mut self) -> &mut App {
        self.setup_default_plugins_for_path(project_root_path().join("assets"))
    }

    fn setup_default_plugins_for_path(&mut self, asset_folder: impl AsRef<Path>) -> &mut App {
        self.add_plugins(MinimalPlugins).add_plugins(AssetPlugin {
            file_path: asset_folder.as_ref().to_string_lossy().to_string(),
            ..default()
        });

        #[cfg(feature = "audio_assets")]
        self.add_plugins(AudioPlugin::default());
        self
    }
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub trait DialogueRunnerExt {
    #[must_use]
    fn get_assets_for_id(
        &self,
        line_id: &str,
        loaded_untyped_assets: &Assets<LoadedUntypedAsset>,
    ) -> LineAssets;
}

impl DialogueRunnerExt for DialogueRunner {
    fn get_assets_for_id(
        &self,
        line_id: &str,
        loaded_untyped_assets: &Assets<LoadedUntypedAsset>,
    ) -> LineAssets {
        let line_id = UnderlyingYarnLine {
            id: LineId(line_id.to_string()),
            text: String::new(),
            attributes: vec![],
        };
        self.asset_providers()
            .map(|p| p.get_assets(&line_id, loaded_untyped_assets))
            .collect()
    }
}
