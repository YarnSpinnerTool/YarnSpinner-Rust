# Yarn Slinger Demo

This crate is a complete example of a demo game showing off all kinds of features of Yarn Slinger.
You can run it natively with
```bash
cargo run --bin bevy_yarn_slinger_demo
```
Or you can run the `run.sh` found in the `wasm` directory to build and run it in the browser.
Note that doing so on Windows requires deleting the `assets` symlink found in the `wasm` directory and copying `crates/bevy_plugin/assets` to `crates/bevy_plugin/wasm/assets`.

All assets included were made by Jan Hohenheim and are licensed under [CC0](https://creativecommons.org/publicdomain/zero/1.0/).
