#![allow(dead_code)]

use bevy::asset::LoadedUntypedAsset;
#[cfg(feature = "audio_assets")]
use bevy::audio::AudioPlugin;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner::UnderlyingYarnLine;
use std::path::{Path, PathBuf};

pub mod assertion;

pub mod prelude {
    #[allow(unused_imports)] // False positive
    pub use super::{assertion::*, *};
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    #[must_use]
    fn load_project_mut(&mut self) -> Mut<'_, YarnProject>;

    fn load_lines(&mut self) -> &mut App;

    fn load_project_and_get_dialogue_builder(&mut self) -> DialogueRunnerBuilder;

    fn continue_dialogue_and_update(&mut self) -> &mut App;
    fn continue_dialogue_and_update_n_times(&mut self, n: usize) -> &mut App;

    #[must_use]
    fn dialogue_runner_entity(&mut self) -> Entity;

    #[must_use]
    fn dialogue_runner(&mut self) -> &DialogueRunner;
    #[must_use]
    fn dialogue_runner_mut(&mut self) -> Mut<'_, DialogueRunner>;
    fn setup_default_plugins(&mut self) -> &mut App;
    fn setup_default_plugins_for_path(&mut self, asset_folder: impl AsRef<Path>) -> &mut App;

    #[must_use]
    fn clone_loaded_untyped_assets(&self) -> Assets<LoadedUntypedAsset>;
}

impl AppExt for App {
    fn load_project(&mut self) -> &YarnProject {
        while !self.world().contains_resource::<YarnProject>() {
            self.update();
        }
        self.world().resource::<YarnProject>()
    }

    fn load_project_mut(&mut self) -> Mut<'_, YarnProject> {
        while !self.world().contains_resource::<YarnProject>() {
            self.update();
        }
        self.world_mut().resource_mut::<YarnProject>()
    }

    fn load_lines(&mut self) -> &mut App {
        self.load_project();
        loop {
            let assets = self.clone_loaded_untyped_assets();
            if self.dialogue_runner_mut().update_line_availability(&assets) {
                break;
            }
            self.update();
        }
        self
    }

    fn load_project_and_get_dialogue_builder(&mut self) -> DialogueRunnerBuilder {
        while !self.world().contains_resource::<YarnProject>() {
            self.update();
        }
        let mut system_state: SystemState<(Commands, Res<YarnProject>)> =
            SystemState::new(self.world_mut());
        let (mut commands, yarn_project) = system_state.get_mut(self.world_mut());
        yarn_project.build_dialogue_runner(&mut commands)
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

    fn dialogue_runner_entity(&mut self) -> Entity {
        let existing_entity = self
            .world()
            .iter_entities()
            .filter(|e| self.world().get::<DialogueRunner>(e.id()).is_some())
            .map(|e| e.id())
            .next();
        if let Some(entity) = existing_entity {
            entity
        } else {
            self.load_project();
            let mut system_state: SystemState<(Commands, Res<YarnProject>)> =
                SystemState::new(self.world_mut());
            let (mut commands, yarn_project) = system_state.get_mut(self.world_mut());
            let dialogue_runner = yarn_project.create_dialogue_runner(&mut commands);
            system_state.apply(self.world_mut());
            self.world_mut().spawn(dialogue_runner).id()
        }
    }

    fn dialogue_runner(&mut self) -> &DialogueRunner {
        let entity = self.dialogue_runner_entity();
        self.world().get::<DialogueRunner>(entity).unwrap()
    }

    fn dialogue_runner_mut(&mut self) -> Mut<'_, DialogueRunner> {
        let entity = self.dialogue_runner_entity();
        self.world_mut().get_mut::<DialogueRunner>(entity).unwrap()
    }

    fn setup_default_plugins(&mut self) -> &mut App {
        self.setup_default_plugins_for_path(project_root_path().join("assets"))
    }

    fn setup_default_plugins_for_path(&mut self, asset_folder: impl AsRef<Path>) -> &mut App {
        self.add_plugins(MinimalPlugins).add_plugins(AssetPlugin {
            file_path: asset_folder.as_ref().to_string_lossy().to_string(),
            watch_for_changes_override: Some(false),
            ..default()
        });

        #[cfg(feature = "audio_assets")]
        self.add_plugins(AudioPlugin::default());
        self
    }

    fn clone_loaded_untyped_assets(&self) -> Assets<LoadedUntypedAsset> {
        self.world()
            .resource::<Assets<LoadedUntypedAsset>>()
            .iter()
            .map(|(_handle, asset)| LoadedUntypedAsset {
                handle: asset.handle.clone(),
            })
            .fold(Assets::default(), |mut assets, asset| {
                assets.add(asset);
                assets
            })
    }
}

pub fn project_root_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub trait DialogueRunnerExt {
    #[must_use]
    fn get_assets_for_id(&self, line_id: &str) -> LineAssets;
}

impl DialogueRunnerExt for DialogueRunner {
    fn get_assets_for_id(&self, line_id: &str) -> LineAssets {
        let line_id = UnderlyingYarnLine {
            id: LineId(line_id.to_string()),
            text: String::new(),
            attributes: vec![],
        };
        self.asset_providers()
            .map(|p| p.get_assets(&line_id))
            .collect()
    }
}
