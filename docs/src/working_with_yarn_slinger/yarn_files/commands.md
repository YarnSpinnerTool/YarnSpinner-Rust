# Commands

You've seen the `<<something something>>` syntax a couple of times now. 
Everything that happens between the double angle brackets (`<<` & `>>`) is called a *command*.

Commands serve either fundamental operations such as declaring new [variables](variables.md) or instructing the game engine to 
manipulate the world somehow. A command takes up an entire line.
In contrast to [functions](functions.md), commands return no value and can thus not be used within lines via interpolation.

Defining your own commands is specific to the game engine used. 
For Bevy, see the chapter [Custom Commands](../bevy_plugin/custom_commands.md).

The following commands are available by default in all game engines:

## Variables
- `<<declare $variable = initial_value>>`: Creates a new variable and initializes it with a value.
- `<<set $variable = new_value>>`: Assigns a new value to an existing variable.

## Flow control
- `<<if $condition>>` / `<<elseif $condition>>` / `<<else>>` / `<<endif>>`: Executes lines conditionally. In [options](options.md), place `<<if $foo>>` at the end of the line.
- `<<stop>>`: Immediately ends the dialog as if it ran out of lines.

## Engine communication
- `<<wait $seconds>>`: Waits for `$seconds` seconds before continuing the dialog, e.g. `wait 3.5` will wait for 3.5 seconds. 
This will not block the game engine, so the rest of the game can continue running in the meantime, presumably without the player gaining control.
