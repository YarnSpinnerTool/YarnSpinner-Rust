use bevy::prelude::*;
use bevy_yarn_slinger::prelude::*;
use bevy_yarn_slinger_example_dialogue_view::prelude::*;

// For comments about the setup, see hello_world.rs
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        YarnSlingerPlugin::new(),
        ExampleYarnSlingerDialogueViewPlugin::new(),
    ))
    .add_systems(Startup, setup_camera)
    .add_systems(
        Update,
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>()),
    )
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    // Add our custom commands to the dialogue runner
    dialogue_runner
        .commands_mut()
        .add_command("insert_resource", insert_resource)
        .add_command("read_resource", read_resource);
    dialogue_runner.start_node("CustomCommand");
    commands.spawn(dialogue_runner);
}

#[derive(Resource)]
struct SomethingAddedByYarnSlinger {
    name: String,
    age: f32,
}

// Commands are valid Bevy systems with inputs (and optional outputs).
// The `In` param will determine the Yarn signature. This function can thus be called like
// `<<insert_resource "Bob" 42>>` in Yarn.
fn insert_resource(In((name, age)): In<(String, f32)>, mut commands: Commands) {
    commands.insert_resource(SomethingAddedByYarnSlinger { name, age });
}

// Commands with no inputs have the unit type (`()`) as their input.
// This function can thus be called like `<<read_resource>>` in Yarn.
fn read_resource(_: In<()>, previously_added_resource: Res<SomethingAddedByYarnSlinger>) {
    println!(
        "{} is {} years old",
        previously_added_resource.name, previously_added_resource.age
    );
}
