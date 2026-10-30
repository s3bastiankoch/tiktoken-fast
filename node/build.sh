#!/usr/bin/env bash
# Builds the native addon for this machine as tiktoken-fast.<platform>-<arch>.node.
set -euo pipefail
cd "$(dirname "$0")"
cargo build --release
plat=$(node -p 'process.platform + "-" + process.arch')
case "$(uname -s)" in
  Darwin) lib=target/release/libtiktoken_fast_node.dylib ;;
  Linux) lib=target/release/libtiktoken_fast_node.so ;;
  *) lib=target/release/tiktoken_fast_node.dll ;;
esac
cp "$lib" "tiktoken-fast.$plat.node"
echo "built tiktoken-fast.$plat.node"
