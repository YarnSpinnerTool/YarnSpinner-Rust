use anyhow::Result;
use bevy::prelude::*;
use bevy_yarnspinner::{events::*, prelude::*};
use utils::prelude::*;

mod utils;

#[test]
fn basic_functions() -> Result<()> {
    let mut app = App::new();
    let mut asserter = EventAsserter::new();
    let mut dialogue_runner = app.setup_dialogue_runner();
    dialogue_runner.start_node("Start");
    app.update();
    assert_events!(asserter, app contains [
        PresentLineEvent with |event| event.line.text == "Data = Initial",
    ]);
    app.continue_dialogue_and_update();
    assert_events!(asserter, app contains [
        PresentLineEvent with |event| event.line.text == "New Data = After Swap",
    ]);
    app.continue_dialogue_and_update();
    assert_events!(asserter, app contains [
        PresentLineEvent with |event| event.line.text == "Picky, picky: true",
    ]);

    Ok(())
}

#[derive(Debug, Resource)]
struct Data(String);

trait FunctionAppExt {
    fn setup_dialogue_runner(&mut self) -> Mut<DialogueRunner>;
}

impl FunctionAppExt for App {
    fn setup_dialogue_runner(&mut self) -> Mut<DialogueRunner> {
        self.insert_resource(Data("Initial".to_string()));

        let swap_data =
            self.register_system(|In(param): In<String>, mut data: ResMut<Data>| -> String {
                let old = data.0.clone();
                data.0 = param;
                old
            });

        self.add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Name::new("Tweedledee"));
        });

        let picky_function = self.register_system(|_: Single<&Name>| -> bool { true });

        let mut dialogue_runner = self
            .setup_default_plugins()
            .add_plugins(YarnSpinnerPlugin::with_yarn_source(YarnFileSource::file(
                "functions.yarn",
            )))
            .dialogue_runner_mut();
        dialogue_runner
            .library_mut()
            .add_function("swap_data", swap_data);
        dialogue_runner
            .library_mut()
            .add_function("picky_function", picky_function);
        dialogue_runner
    }
}
