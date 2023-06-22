# Functions

Function calls look like this:
```text
title: Start
---
I rolled a die and got a {dice(6)}!
===
```
Which will result in something like this:

![dice.png](dice.png)

The curly braces (`{}`) are not part of the function call, but are used to interpolate the result of the function into the text,
as seen previously in the chapter [Variables](variables.md).

There are a number of built-in functions available, such as the `dice` function used above.
Defining your own functions is specific to the game engine used. 
For Bevy, see the chapter [Custom Functions](../bevy_plugin/custom_functions.md).

The following functions are available by default in all game engines:

## Random
- `dice(sides)`: Simulates a `sides`-sided die roll, i.e. returns a random number between 1 and `sides`, inclusive.
- `random()`: Returns a random real number between 0 and 1.
- `random_range(min, max)`: Returns a random integer between `min` and `max`, inclusive.
If either `min` or `max` is not an integer, the generated number will instead be a real number between `min` and `max`.

## Visited nodes
- `visited(node)`: Returns `true` if the node named `node` exists and has been visited and exited before, `false` otherwise.
- `visited_count(node)`: Returns the number of times the node named `node` has been visited and exited.

## Type casts

- `string(value)`: Returns the string representation of `value`.
- `number(value)`: Returns the number representation of `value`.
- `bool(value)`: Returns the boolean representation of `value`.

## Number manipulation

- `round(n)`: Rounds `n` to the nearest integer.
- `round_places(n, places)`: Rounds `n` to the nearest integer with `places` decimals.
- `floor(n)`: Rounds `n` down to the nearest integer.
- `ceil(n)`: Rounds `n` up to the nearest integer.
- `inc(n)`: Returns `n + 1` if `n` is an integer, otherwise rounds `n` up to the nearest integer.
- `dec(n)`: Returns `n - 1` if `n` is an integer, otherwise rounds `n` down to the nearest integer.
- `decimal(n)`: Returns the decimal part of `n`. This is guaranteed to return a number between 0.0 and 1.0, e.g. `decimal(3.14)` returns `0.14`.
- `int(n)`: Returns the integer part of `n`, e.g. `int(3.14)` returns `3` and `int(-3.14)` returns `-3`. 
This effectively means that the number is rounded towards the nearest integer toward zero.
