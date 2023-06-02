#!/usr/bin/env bash
set -e

# Run this from the repo root.
# Note that the provided `assets` symlink only works on macOS and Linux.
# On Windows, you'll have to remove it and then copy the directory `crates/bevy_plugin/assets` to `crates/bevy_plugin/wasm/assets` manually.

rustup target add wasm32-unknown-unknown
cargo build --example story --features example_ui --target wasm32-unknown-unknown

cargo install wasm-bindgen-cli || true
wasm-bindgen --out-name yarn_slinger_story_demo --out-dir crates/bevy_plugin/wasm/target --target web target/wasm32-unknown-unknown/debug/examples/story.wasm

cargo install basic-http-server || true
basic-http-server crates/bevy_plugin/wasm
