use yarnspinner_without_bevy_examples::TuiDialogueRunner;

fn main() -> anyhow::Result<()> {
    // See lib.rs for more details on how this works!
    let mut runner =
        TuiDialogueRunner::new("./assets/dialogue/custom_function.yarn", "CustomFunction")?;

    runner.add_function("pow", pow);
    runner.run()?;

    Ok(())
}

fn pow(base: f32, exponent: f32) -> f32 {
    base.powf(exponent)
}
