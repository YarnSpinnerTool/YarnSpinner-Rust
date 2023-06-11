#!/usr/bin/env bash
set -e

# Note that the provided `assets` symlink only works on macOS and Linux.
# On Windows, you'll have to remove it and then copy the directory `crates/bevy_plugin/assets` to `crates/bevy_plugin/wasm/assets` manually.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"/../..

rustup target add wasm32-unknown-unknown
cargo build --bin bevy_yarn_slinger_demo --features editor --target wasm32-unknown-unknown

cargo install wasm-bindgen-cli || true
wasm-bindgen --no-typescript --out-name bevy_yarn_slinger_demo --out-dir demo/wasm --target web target/wasm32-unknown-unknown/debug/bevy_yarn_slinger_demo.wasm

cargo install basic-http-server || true
basic-http-server demo/wasm
