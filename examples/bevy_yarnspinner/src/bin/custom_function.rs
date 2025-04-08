use bevy::prelude::*;
use bevy_yarnspinner::prelude::*;
use bevy_yarnspinner_example_dialogue_view::prelude::*;

// For comments about the setup, see hello_world.rs
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        YarnSpinnerPlugin::new(),
        ExampleYarnSpinnerDialogueViewPlugin::new(),
    ))
    .insert_resource(Counter(42))
    .add_systems(Startup, setup_camera)
    .add_systems(
        Update,
        spawn_dialogue_runner.run_if(resource_added::<YarnProject>),
    )
    .run();
}

#[derive(Resource)]
struct Counter(u32);

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    // Add our custom function to the dialogue runner
    dialogue_runner.library_mut().add_function("pow", pow);
    dialogue_runner
        .library_mut()
        .add_function("get_counter", get_counter);
    dialogue_runner.start_node("CustomFunction");
    commands.spawn(dialogue_runner);
}

fn pow(base: f32, exponent: f32) -> f32 {
    base.powf(exponent)
}

fn get_counter(_: In<()>, mut counter: ResMut<Counter>) -> u32 {
    counter.0 += 1;
    counter.0
}
