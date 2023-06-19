# Variables

Values can be stored in variables. A new variable is declared with the `<<declare>>` command:

```text
title: Start
---
<<declare $name = "Ferris">>
===
```

They can be used within text by surrounding them with curly braces (`{}`):

```text
title: Start
---
<<declare $name = "Player">>
<<declare $age = 26>>
Ferris: Hello, {$name}! I heard you are {$age} years old!
===
```
This will be displayed as:

TODO

You can change the value of a variable with the `<<set>>` command:

```text
title: Start
---
<<declare $name = "Player">>
<<declare $age = 26>>
Ferris: Hello, {name}! I heard you are {$age} years old!
One year later...
<<set $age = $age + 1>>
Ferris: Hello, {name}! Wow, you're {$age} years old now! Time sure flies, eh?
===
```

Variables can have the following types:
- `string`: A string of characters, like `"Hello World!"`.
- `number`: A number, like `42`, `0`, `-99999`, `3.1415`, or `6.0`.
- `boolean`: Either `true` or `false`.

All variable names must start with a `$` and can only contain letters, numbers, and underscores (`_`).
