# Native Node addon against the WebAssembly build, 2026-10-27

Same machine and method as `../2026-10-27-final`, with `node/index.js` (the
napi-rs addon, built with `-C target-cpu=native`) in place of the
WebAssembly package. No token differed (`tests.txt`, `js-main.txt`).

gpt-tokenizer's seven-input benchmark, mean ms per call, native (WebAssembly):

| | encode | count | decode | load |
|---|---:|---:|---:|---:|
| o200k_base | 0.56 (1.00) | 0.45 (0.87) | 0.65 (0.67) | 50 (143) |
| cl100k_base | 0.57 (1.01) | 0.44 (0.85) | 0.65 (0.69) | 28 |

gpt-tokenizer: o200k_base 7.28, 5.34, 0.95, 152 ms. On the main workloads
(`js-main.txt`) the addon is 1.18x faster than the WebAssembly build (1.20x
new text, 1.13x fresh caches); on chat-sized documents most of a call is
crossing between JavaScript and the addon, so the gain is smallest there.
