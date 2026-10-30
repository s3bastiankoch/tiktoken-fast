#!/usr/bin/env bash
# Installs the toolchains the published runs used, on Debian 12 (x86-64 or
# arm64): Rust 1.99.1 with the wasm32 target, wasm-bindgen-cli 0.2.100,
# Node 22.23.4 and Go 1.26.0. Python 3.11 comes with Debian.
set -euo pipefail
sudo apt-get update -qq
sudo apt-get install -y -qq build-essential pkg-config git curl python3-venv python3-dev util-linux
case "$(uname -m)" in x86_64) na=x64; ga=amd64 ;; aarch64) na=arm64; ga=arm64 ;; esac
command -v rustup >/dev/null || curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain 1.99.1
. "$HOME/.cargo/env"
rustup default 1.99.1
rustup target add wasm32-unknown-unknown
cargo install -q wasm-bindgen-cli --version 0.2.100 --locked
curl -sSfL "https://nodejs.org/dist/v22.23.4/node-v22.23.4-linux-$na.tar.xz" | sudo tar -xJ -C /usr/local --strip-components=1
curl -sSfL "https://go.dev/dl/go1.26.0.linux-$ga.tar.gz" | sudo tar -xz -C /usr/local
echo 'export PATH=/usr/local/go/bin:$HOME/go/bin:$PATH' >> "$HOME/.profile"
echo "done: $(rustc --version), node $(node --version), $(/usr/local/go/bin/go version)"
