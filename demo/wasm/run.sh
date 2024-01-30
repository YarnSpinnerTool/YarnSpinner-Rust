#!/usr/bin/env bash
set -e

# Note that the provided `assets` symlink only works on macOS and Linux.
# On Windows you'll have to remove it and then copy the directory `demo/assets` to `demo/wasm/assets` manually.

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

cd "$SCRIPT_DIR"/../..

rustup target add wasm32-unknown-unknown
cargo build --bin bevy_yarnspinner_demo --target wasm32-unknown-unknown

# Keep this in sync with the version in `Cargo.lock`.
cargo install wasm-bindgen-cli --version 0.2.90 || true
wasm-bindgen --no-typescript --out-name bevy_yarnspinner_demo --out-dir demo/wasm --target web target/wasm32-unknown-unknown/debug/bevy_yarnspinner_demo.wasm

cargo install basic-http-server || true
basic-http-server demo/wasm
