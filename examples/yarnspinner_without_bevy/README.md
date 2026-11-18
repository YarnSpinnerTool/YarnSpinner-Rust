# Yarn Spinner without Bevy

Terminal examples that run Yarn dialogue with the `yarnspinner` crate directly.

The dialogue has to be compiled with [`ysc`](https://github.com/YarnSpinnerTool/YarnSpinner-Console)
first (`scripts/install-ysc.sh` installs it into `tools/`). From this directory:

```sh
for f in assets/dialogue/*.yarn; do ysc compile "$f" -o assets/dialogue/compiled; done
cargo run --bin hello_world
```

Each `.yarn` file is compiled on its own, so ysc names the outputs after it
(`hello_world.yarnc`, `hello_world-Lines.csv`, `hello_world-Metadata.csv`).
