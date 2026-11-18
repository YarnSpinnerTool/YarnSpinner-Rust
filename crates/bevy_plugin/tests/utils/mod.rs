#![allow(dead_code)]

use bevy::asset::LoadedUntypedAsset;
#[cfg(feature = "audio_assets")]
use bevy::audio::AudioPlugin;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_yarnspinner::UnderlyingYarnLine;
use bevy_yarnspinner::prelude::*;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::process::Command;

pub mod assertion;

pub mod prelude {
    #[allow(unused_imports)] // False positive
    pub use super::{assertion::*, *};
}

pub const DEFAULT_COMPILED_DIR: &str = "dialogue/compiled";

pub const STANDALONE_YARN_FILES: &[&str] = &[
    "commands.yarn",
    "functions.yarn",
    "lines.yarn",
    "lines_with_ids.yarn",
    "options.yarn",
    "wait.yarn",
    "folder/folder.yarn",
];

pub fn compile_yarn(asset_root: impl AsRef<Path>, inputs: &[&str], output_dir: impl AsRef<Path>) {
    let asset_root = asset_root.as_ref();
    let output_dir = asset_root.join(output_dir);
    let inputs: Vec<_> = inputs.iter().map(|input| asset_root.join(input)).collect();

    let name = match inputs.as_slice() {
        [input] => input.file_stem().unwrap().to_string_lossy().into_owned(),
        _ => "Output".to_owned(),
    };
    let outputs = [".yarnc", "-Lines.csv", "-Metadata.csv"]
        .map(|suffix| output_dir.join(format!("{name}{suffix}")));
    if outputs.iter().all(|output| output.is_file()) {
        return;
    }

    let staging = tempfile::tempdir_in(asset_root).unwrap();
    let output = Command::new("ysc")
        .arg("compile")
        .args(&inputs)
        .arg("-o")
        .arg(staging.path())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "ysc failed to compile {inputs:?}:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    std::fs::create_dir_all(&output_dir).unwrap();
    for entry in std::fs::read_dir(staging.path()).unwrap() {
        let entry = entry.unwrap();
        std::fs::rename(entry.path(), output_dir.join(entry.file_name())).unwrap();
    }
}

pub struct TimeoutPlugin(pub std::time::Duration);

impl Default for TimeoutPlugin {
    fn default() -> Self {
        Self(std::time::Duration::from_secs(30))
    }
}

impl Plugin for TimeoutPlugin {
    fn build(&self, app: &mut App) {
        let timeout = self.0;
        app.add_systems(
            Last,
            move |mut start: Local<Option<std::time::Instant>>, mut commands: Commands| {
                let start = *start.get_or_insert_with(std::time::Instant::now);
                if start.elapsed() > timeout {
                    error!("Timed out after {timeout:?}");
                    commands.write_message(AppExit::error());
                }
            },
        );
    }
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    #[must_use]
    fn load_project_mut(&mut self) -> Mut<'_, YarnProject>;

    fn load_lines(&mut self) -> &mut App;

    fn load_project_and_get_dialogue_builder(&mut self) -> DialogueRunnerBuilder;

    fn clear_and_assert_event<T: Event + Clone + Debug>(&mut self);

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
    fn setup_default_plugins_for_path_with_watching(
        &mut self,
        asset_folder: impl AsRef<Path>,
        watch_for_changes: bool,
    ) -> &mut App;

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
        let (mut commands, yarn_project) = system_state.get_mut(self.world_mut()).unwrap();
        yarn_project.build_dialogue_runner(&mut commands)
    }

    fn clear_and_assert_event<T: Event + Clone + Debug>(&mut self) {
        let resource = self
            .world_mut()
            .remove_resource::<assertion::EventAsserter<T>>();
        assert!(
            resource.is_some(),
            "No EventAsserter with type {} registered",
            stringify!(T)
        );
        let resource = resource.unwrap();
        assert_eq!(
            resource.expected_calls,
            resource.actual_calls,
            "Expected {} events of type {}, but found {}",
            resource.expected_calls,
            stringify!(T),
            resource.actual_calls
        );
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
            .world_mut()
            .query_filtered::<Entity, With<DialogueRunner>>()
            .iter(self.world())
            .next();
        if let Some(entity) = existing_entity {
            entity
        } else {
            self.load_project();
            let mut system_state: SystemState<(Commands, Res<YarnProject>)> =
                SystemState::new(self.world_mut());
            let (mut commands, yarn_project) = system_state.get_mut(self.world_mut()).unwrap();
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
        let assets = project_root_path().join("assets");
        for file in STANDALONE_YARN_FILES {
            compile_yarn(&assets, &[file], DEFAULT_COMPILED_DIR);
        }
        self.setup_default_plugins_for_path(assets)
    }

    fn setup_default_plugins_for_path(&mut self, asset_folder: impl AsRef<Path>) -> &mut App {
        self.setup_default_plugins_for_path_with_watching(asset_folder, false)
    }

    fn setup_default_plugins_for_path_with_watching(
        &mut self,
        asset_folder: impl AsRef<Path>,
        watch_for_changes: bool,
    ) -> &mut App {
        self.add_plugins(MinimalPlugins)
            .add_plugins(AssetPlugin {
                file_path: asset_folder.as_ref().to_string_lossy().to_string(),
                watch_for_changes_override: Some(watch_for_changes),
                ..default()
            })
            .add_plugins(TimeoutPlugin::default());

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

/// signature: (commands: Commands, condition: bool, message: format str)
/// Will send an AppExit::error() and early exit on failure.
#[macro_export]
macro_rules! cmd_assert {
    ($commands:expr, $condition:expr, $message:expr) => {
        if !$condition {
            error!($message);
            $commands.write_message(AppExit::error());
            return;
        }
    };
}
