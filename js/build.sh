#!/usr/bin/env bash
# Builds the WebAssembly module and its JavaScript bindings for Node
# (pkg-node), bundlers (pkg-bundler) and plain browsers (pkg-web).
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli 0.2.100.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release --target wasm32-unknown-unknown
wasm=target/wasm32-unknown-unknown/release/tiktoken_fast_js.wasm
for t in nodejs bundler web; do
  out=pkg-${t/nodejs/node}
  wasm-bindgen --target "$t" --out-dir "$out" "$wasm"
done
