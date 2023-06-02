#!/usr/bin/env bash
set -e

# Run this from the repo root

# Requires `rustup target add wasm32-unknown-unknown`
cargo build --example story --target wasm32-unknown-unknown
# Requires `cargo install wasm-bindgen-cli`
wasm-bindgen --out-name yarn_slinger_story_demo --out-dir crates/bevy_plugin/wasm/target --target web target/wasm32-unknown-unknown/release/examples/story.wasm
# Requires `cargo install basic-http-server`
basic-http-server crates/bevy_plugin/wasm
