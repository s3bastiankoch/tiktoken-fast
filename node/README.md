# tiktoken-fast for Node, native

The Rust core as a native Node addon (napi-rs), with the same API as the
WebAssembly build in [`../js`](../js) and the `tiktoken` npm package.
`index.js` loads the addon for the current platform when it was built and
falls back to the WebAssembly build otherwise, so code does not change.

```sh
./build.sh      # writes tiktoken-fast.<platform>-<arch>.node
```

```js
const { get_encoding } = require("./index.js");
const enc = get_encoding("o200k_base");
enc.encode("hello world");   // Uint32Array [24912, 2375]
```

On x86 it is 1.8x faster than the WebAssembly build on gpt-tokenizer's
benchmark (encode 0.56 ms against gpt-tokenizer's 7.28 ms) and loads in 50 ms
instead of 143 ms; on new text it is 1.2x faster than the WebAssembly build (9.19x against
7.66x over the tiktoken npm package, one Xeon 8481C core). The x86 numbers
above were built with `target-cpu=native`.
See [`bench/results/2026-10-27-native`](../bench/results/2026-10-27-native).
