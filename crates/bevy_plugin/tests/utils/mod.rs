#![allow(dead_code)]
use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;

pub mod prelude {
    pub use super::*;
}

pub trait AppExt {
    fn load_project(&mut self) -> &YarnProject;
    fn load_project_mut(&mut self) -> Mut<YarnProject>;
    fn load_lines(&mut self) -> &mut App;
    fn load_assets(&mut self) -> &mut App;
}

impl AppExt for App {
    fn load_project(&mut self) -> &YarnProject {
        while !self.world.contains_resource::<YarnProject>() {
            self.update();
        }
        self.world.resource::<YarnProject>()
    }

    #[must_use]
    fn load_project_mut(&mut self) -> Mut<YarnProject> {
        while !self.world.contains_resource::<YarnProject>() {
            self.update();
        }
        self.world.resource_mut::<YarnProject>()
    }

    fn load_lines(&mut self) -> &mut App {
        self.load_project();
        while self.world.is_resource_changed::<YarnProject>() {
            self.update();
        }

        while !self.world.resource::<YarnProject>().lines_available() {
            self.update();
        }
        self
    }

    fn load_assets(&mut self) -> &mut App {
        self.load_project();
        while self.world.is_resource_changed::<YarnProject>() {
            self.update();
        }

        while !self.world.resource::<YarnProject>().assets_available() {
            self.update();
        }
        self
    }
}
