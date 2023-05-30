#![allow(dead_code)]
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger::UnderlyingYarnLine;

pub mod prelude {
    pub use super::*;
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    #[must_use]
    fn load_project_mut(&mut self) -> Mut<YarnProject>;
    fn load_texts(&mut self) -> &mut App;
    fn load_assets(&mut self) -> &mut App;
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

    fn load_texts(&mut self) -> &mut App {
        self.load_project();
        while !self
            .dialogue_runner()
            .data_providers()
            .are_texts_available()
        {
            self.update();
        }
        self
    }

    fn load_assets(&mut self) -> &mut App {
        self.load_project();
        while !self
            .dialogue_runner()
            .data_providers()
            .are_assets_available()
        {
            self.update();
        }
        self
    }

    fn try_dialogue_runner(&self) -> Option<&DialogueRunner> {
        self.world
            .iter_entities()
            .filter_map(|e| self.world.get::<DialogueRunner>(e.id()))
            .next()
    }

    fn try_dialogue_runner_mut(&mut self) -> Option<Mut<DialogueRunner>> {
        let entity = self
            .world
            .iter_entities()
            .map(|e| e.id())
            .find(|e| self.world.get::<DialogueRunner>(*e).is_some())
            .unwrap();
        self.world.get_mut::<DialogueRunner>(entity)
    }
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
        self.data_providers()
            .asset_providers()
            .map(|p| p.get_assets(&line_id))
            .collect()
    }
}
