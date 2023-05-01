# Yarn Slinger

**WIP** This project is still early work in progress. There may be large chunks of missing or untested functionality.
Please ask before contributing, to make sure your work is not wasted or duplicated!

## Yarn Spinner

[Yarn Spinner](https://github.com/YarnSpinnerTool/) is a set of "narrative tools for game development". The original,
licensed under MIT, allows writers to do their part effectively and efficiently in developing games without doing any
actual coding. Yarn Spinner® and Secret Lab® are trade marks of Secret Lab Pty. Ltd. and and are not affiliated with 
this project.

Goal of the Yarn Slinger project is to bring this capability set to the rust world (
specifically [bevy](https://github.com/bevyengine/bevy)) while maintaining compatibility with the `yarn` syntax/language designed
by the great team at [Secret Lab](https://secretlab.games/).

## Build

Building requires pulling in the submodules:

```bash
git submodule update --init --recursive
```

As well as installing `protoc`
