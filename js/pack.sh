#!/usr/bin/env bash
# Builds the npm package into js/dist: the three wasm-bindgen outputs plus
# package.json, README, LICENSE and NOTICE. Publish with `npm publish js/dist`.
set -euo pipefail
cd "$(dirname "$0")"
./build.sh
rm -rf dist && mkdir dist
cp -r pkg-node dist/node && cp -r pkg-bundler dist/bundler && cp -r pkg-web dist/web
rm -f dist/*/.gitignore dist/*/package.json
cp npm/package.json README.md dist/ && cp ../LICENSE ../NOTICE dist/
