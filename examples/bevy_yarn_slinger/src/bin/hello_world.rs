use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_dialogue_view::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        // Register the Yarn Slinger plugin using its default settings, which will look for Yarn files in the "dialogue" folder
        .add_plugin(YarnSlingerPlugin::new())
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
    // Create a dialogue runner from the project.
    // If you have a node named "Start", you can skip the builder and just use "project.default_dialogue_runner()" instead
    let mut dialogue_runner = project.build_dialogue_runner().with_start_node("HelloWorld").build();
    // Immediately start showing the dialogue to the player
    dialogue_runner.start();
    commands.spawn(dialogue_runner);
}
