#!/usr/bin/env bash
set -e

# Verify that butler is installed. See https://itch.io/docs/butler/installing.html.
butler --version

# Go to crate root.
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$SCRIPT_DIR"/../..

# Compile.
rustup target add wasm32-unknown-unknown
cargo build --profile wasm-release --target wasm32-unknown-unknown --bin bevy_yarn_slinger_demo

# Create directory we will publish.
rm -r demo/wasm/build || true
mkdir -p demo/wasm/build

# Run `wasm-bindgen`.
cargo install wasm-bindgen-cli || true
wasm-bindgen --no-typescript --out-name bevy_yarn_slinger_demo --out-dir demo/wasm/build/ --target web target/wasm32-unknown-unknown/wasm-release/bevy_yarn_slinger_demo.wasm

# Run wasm-opt. This must be run after wasm-bindgen because wasm-bindgen thinks the optimized .wasm is broken.
cargo install wasm-opt || true
wasm-opt -Oz --output demo/wasm/build/bevy_yarn_slinger_demo_bg.wasm.optimized demo/wasm/build/bevy_yarn_slinger_demo_bg.wasm
rm demo/wasm/build/bevy_yarn_slinger_demo_bg.wasm
mv demo/wasm/build/bevy_yarn_slinger_demo_bg.wasm.optimized demo/wasm/build/bevy_yarn_slinger_demo_bg.wasm

# Copy assets. Uses `git archive` to avoid including `.gitignore`d files.
cp demo/wasm/index.html demo/wasm/build/
git archive -o demo/wasm/build/assets.zip HEAD:demo/assets
unzip -o demo/wasm/build/assets.zip -d demo/wasm/build/assets
rm demo/wasm/build/assets.zip

# Publish to itch.io.
zip --recurse-paths demo/wasm/build.zip demo/wasm/build
butler push --fix-permissions --userversion="0.1.0" demo/wasm/build.zip janhohenheim/yarn-slinger-demo:wasm
