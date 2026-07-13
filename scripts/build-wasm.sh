#!/usr/bin/env bash
set -euo pipefail

# Builds the politebot-core wasm module and generates JS bindings into
# site/pkg/, which site/index.html loads directly.
#
# Requires:
#   rustup target add wasm32-unknown-unknown
#   cargo install wasm-bindgen-cli --version <matches the wasm-bindgen dep in Cargo.toml>

cd "$(dirname "$0")/.."

cargo build --release --target wasm32-unknown-unknown

wasm-bindgen \
  target/wasm32-unknown-unknown/release/politebot_core.wasm \
  --out-dir site/pkg \
  --target web \
  --no-typescript
