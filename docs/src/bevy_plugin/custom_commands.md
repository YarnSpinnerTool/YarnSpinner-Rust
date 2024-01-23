# Custom Commands

[Commands](../yarn_files/commands.md) work very similar to Yarn [functions](../yarn_files/functions.md), but 
use a different syntax and are able to modify the game world. As a consequence of their similarity,
registering custom commands is very similar to registering [custom functions](./custom_functions.md).

## Command Registration

Just as with Yarn functions, registration happens when creating a `DialogRunner`.
Let's again modify the example from the [setup](./setup.md):

```rust
fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    // Add our custom command to the dialogue runner
    dialogue_runner
        .commands_mut()
        .add_command("print_addition", print_addition);
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}

fn print_addition(In((a, b)): In<(f32, f32)>) {
    print!("{a} + {b} = {c}", c = a + b)
}
```

We call the command like this:

```text
title: Start
---
Let's print the addition of 1 and 3 in the console:
<<print_addition(1, 3)>>
===
```

You will have seen one crucial difference to Yarn functions immediately.
The parameters are not passed in directly to the Rust function, but are wrapped in an `In` struct.
This is because Rust functions that are registered as commands are always valid Bevy systems. 
The `In` parameter just tells the function which values come from the Yarn file, but we can additionally query the Bevy world as we want:

```rust
fn spawn_dialogue_runner(mut commands: Commands, project: Res<YarnProject>) {
    let mut dialogue_runner = project.create_dialogue_runner();
    // Add our custom command to the dialogue runner
    dialogue_runner
        .commands_mut()
        .add_command("insert_resource", insert_resource);
    dialogue_runner.start_node("Start");
    commands.spawn(dialogue_runner);
}

TODO 
fn insert_resource(In(name): In<&str>) {
    print!("{a} + {b} = {c}", c = a + b)
}
```

TODO

TODO mention return types

TODO mention that no params still needs In param
