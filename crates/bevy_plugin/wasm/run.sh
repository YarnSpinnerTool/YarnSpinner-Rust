#!/usr/bin/env bash
set -e

# Run this from the repo root.
# Note that the provided `assets` symlink only works on macOS and Linux,
# so on Windows you'll have to remove it and then copy the directory `crates/bevy_plugin/assets` to `crates/bevy_plugin/wasm/assets` manually.

# Requires `rustup target add wasm32-unknown-unknown`
cargo build --example story --target wasm32-unknown-unknown
# Requires `cargo install wasm-bindgen-cli`
wasm-bindgen --out-name yarn_slinger_story_demo --out-dir crates/bevy_plugin/wasm/target --target web target/wasm32-unknown-unknown/debug/examples/story.wasm
# Requires `cargo install basic-http-server`
basic-http-server crates/bevy_plugin/wasm
