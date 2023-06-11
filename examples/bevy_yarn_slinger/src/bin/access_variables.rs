use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_ui::prelude::*;

// For comments about the setup, see hello_world.rs
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(YarnSlingerPlugin::with_yarn_files(vec![
            "access_variables.yarn",
        ]))
        .add_plugin(ExampleYarnSlingerUiPlugin::new())
        .add_systems((
            setup_camera.on_startup(),
            spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
            print_yarn_variable.run_if(any_with_component::<DialogueRunner>()),
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.default_dialogue_runner().unwrap();
    dialogue_runner.start();
    commands.spawn(dialogue_runner);
}

fn print_yarn_variable(dialogue_runner: Query<&DialogueRunner>) {
    let dialogue_runner = dialogue_runner.single();
    let value = dialogue_runner.variable_storage().get("$foo");
    if let Ok(value) = value {
        println!("foo: {value}");
    }
}
