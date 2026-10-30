// The native addon for this platform when one was built, else the
// WebAssembly build: the same API either way.
const path = require("node:path");
const fs = require("node:fs");

const native = path.join(__dirname, `tiktoken-fast.${process.platform}-${process.arch}.node`);
module.exports = fs.existsSync(native) && !process.env.TIKTOKEN_FAST_WASM
  ? require(native)
  : require(path.join(__dirname, "../js/pkg-node/tiktoken_fast_js.js"));
