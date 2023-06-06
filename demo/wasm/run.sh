#!/usr/bin/env bash
set -e

# Run this from the repo root.
# Note that the provided `assets` symlink only works on macOS and Linux.
# On Windows, you'll have to remove it and then copy the directory `crates/bevy_plugin/assets` to `crates/bevy_plugin/wasm/assets` manually.

rustup target add wasm32-unknown-unknown
cargo build --bin bevy_yarn_slinger_demo --target wasm32-unknown-unknown

cargo install wasm-bindgen-cli || true
wasm-bindgen --out-name bevy_yarn_slinger_demo --out-dir demo/wasm/target --target web target/wasm32-unknown-unknown/debug/bevy_yarn_slinger_demo.wasm

cargo install basic-http-server || true
basic-http-server demo/wasm
