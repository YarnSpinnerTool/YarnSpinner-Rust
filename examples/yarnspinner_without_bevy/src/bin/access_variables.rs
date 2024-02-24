use yarnspinner::core::YarnValue;
use yarnspinner_without_bevy_examples::TuiDialogueRunner;

fn main() -> anyhow::Result<()> {
    // See lib.rs for more details on how this works!
    let mut runner =
        TuiDialogueRunner::new("./assets/dialogue/access_variables.yarn", "AccessVariables")?;

    runner.set_variable("$foo", YarnValue::Number(0.0))?;

    println!(
        "Value of $foo before dialogue: {:?}",
        runner.get_variable("$foo")?
    );

    runner.run()?;

    println!(
        "Value of $foo after dialogue: {:?}",
        runner.get_variable("$foo")?
    );

    Ok(())
}
