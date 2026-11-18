use yarnspinner_without_bevy_examples::TuiDialogueRunner;

fn main() -> anyhow::Result<()> {
    // See lib.rs for more details on how this works!
    TuiDialogueRunner::new("./assets/dialogue/compiled", "hello_world", "HelloWorld")?.run()
}
