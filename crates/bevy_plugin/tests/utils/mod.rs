#![allow(dead_code)]

use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;
use std::path::PathBuf;

mod assertations;

pub mod prelude {
    pub use super::*;
    pub use assertations::*;
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    #[must_use]
    fn load_project_mut(&mut self) -> Mut<YarnProject>;

    fn load_lines(&mut self) -> &mut App;

    fn continue_dialogue_and_update(&mut self) -> &mut App;
    fn continue_dialogue_and_update_n_times(&mut self, n: usize) -> &mut App;

    #[must_use]
    fn dialogue_runner(&mut self) -> &DialogueRunner;
    #[must_use]
    fn dialogue_runner_mut(&mut self) -> Mut<DialogueRunner>;
    #[must_use]
    fn try_dialogue_runner(&self) -> Option<&DialogueRunner>;
    #[must_use]
    fn try_dialogue_runner_mut(&mut self) -> Option<Mut<DialogueRunner>>;
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

    fn dialogue_runner(&mut self) -> &DialogueRunner {
        if self.try_dialogue_runner().is_some() {
            self.try_dialogue_runner().unwrap()
        } else {
            let project = self.load_project();
            let dialogue_runner = project.default_dialogue_runner().unwrap();
            let entity = self.world.spawn(dialogue_runner).id();
            self.world.get::<DialogueRunner>(entity).unwrap()
        }
    }

    fn dialogue_runner_mut(&mut self) -> Mut<DialogueRunner> {
        if self.try_dialogue_runner().is_some() {
            self.try_dialogue_runner_mut().unwrap()
        } else {
            let project = self.load_project();
            let dialogue_runner = project.default_dialogue_runner().unwrap();
            let entity = self.world.spawn(dialogue_runner).id();
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
