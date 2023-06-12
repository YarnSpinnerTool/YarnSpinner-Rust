use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_dialogue_view::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Get the `hello_world.yarn` file from the `assets` folder.
        // This starts the compilation process for the Yarn project.
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec!["hello_world.yarn"]))
        // Initialize the bundled example UI
        .add_plugin(ExampleYarnSlingerDialogueViewPlugin::new())
        .add_systems((
            setup_camera.on_startup(),
            // Spawn dialogue runner once the Yarn project has finished compiling
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    // Create a dialogue runner from the project
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Immediately start showing the dialogue to the player
    dialogue_runner.start();
    commands.spawn(dialogue_runner);
}
