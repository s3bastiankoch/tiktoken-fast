# tiktoken-fast for JavaScript

The Rust core compiled to WebAssembly, with the API of the `tiktoken` npm
package, so it is a drop-in for code that uses its common calls (not yet
`encode_with_unstable`, `encode_single_token`, `decode_single_token_bytes`,
`token_byte_values` or `tiktoken/lite`):

```js
const { get_encoding } = require("tiktoken-fast");

const enc = get_encoding("o200k_base");
const ids = enc.encode("hello world");        // Uint32Array, same ids as tiktoken
new TextDecoder().decode(enc.decode(ids));    // "hello world"
enc.count("hello world");                      // 2, without returning the ids
enc.free();
```

Node and bundlers (webpack, Vite) load it as above. In a browser without a
bundler, initialise the module first:

```js
import init, { get_encoding } from "tiktoken-fast/web";
await init();
```

`get_encoding` knows `r50k_base` (`gpt2`), `p50k_base`, `p50k_edit`,
`cl100k_base` and `o200k_base`; `encoding_for_model` maps OpenAI model names;
`new Tiktoken(fileText, specialTokens, pattern)` takes any .tiktoken file with
one of their split patterns. `encode(text, allowed_special, disallowed_special)`
follows tiktoken: special tokens in the text throw unless allowed.

## Speed

Node 22, one core of a Xeon Platinum 8481C, 2026-10-27, no token differing in
any run ([raw results](../bench/results/2026-10-27-final)).

On gpt-tokenizer's own headline benchmark (seven inputs, encoded repeatedly
with default caches; rebuilt in `bench/seven.mjs`), o200k_base, mean ms per
call:

| | encode | count | decode | load |
|---|---:|---:|---:|---:|
| tiktoken-fast | **1.00** | **0.87** | **0.67** | **143** |
| gpt-tokenizer 4.1.0 | 7.28 | 5.34 | 0.95 | 153 |
| bpe-lite 0.5.2 | 6.24 | 4.72 | 2.85 | 642 |
| tiktoken 1.0.22 (npm) | 27.80 | 27.77 | 1.34 | 322 |

That benchmark repeats its inputs, which favours caches. On Wikipedia in
eleven languages, code, books and chat-sized pieces, with each library's
caches kept across documents as in a server, tiktoken-fast is 7.7x faster
than the tiktoken npm package and 10.2x faster than gpt-tokenizer on text it
has not seen (one Xeon 8481C core; 8.8 to 9.7x and 10.6 to 11.1x on AMD and
ARM, see the [main README](../README.md#numbers)).

The tables for non-Latin text by backtracking are built once a process has
merged 1 MiB of such text; before that the plain merge runs, so loading stays
short and no call waits for them.

## Build

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100
./build.sh          # pkg-node, pkg-bundler, pkg-web
./pack.sh           # the npm package, in dist/
cd bench && npm install && node ../test/parity.mjs
```
