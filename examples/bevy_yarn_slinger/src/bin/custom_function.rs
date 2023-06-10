use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_ui::prelude::*;

// For comments about the setup, see hello_world.rs
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "custom_function.yarn",
        ]))
        .add_plugin(ExampleYarnSlingerUiPlugin::new())
        .add_systems((
            setup_camera.on_startup(),
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    // Add our custom function to the dialogue runner
    dialogue_runner.library_mut().register_function("pow", pow);
    dialogue_runner.start();
    commands.spawn(dialogue_runner);
}

fn pow(base: f32, exponent: f32) -> f32 {
    base.powf(exponent)
}
