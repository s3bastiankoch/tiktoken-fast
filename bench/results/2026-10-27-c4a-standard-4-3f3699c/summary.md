# Benchmark run 2026-10-27-c4a-standard-4-3f3699c

- Machine: Neoverse-V2, c4a-standard-4, kernel 6.1.0-54-cloud-arm64
- Commit 3f3699c01d1ce240a98fa4c5025e533bd22268f2, started 2026-10-27T09:14:28+00:00
- Pinned to core 2; CPU steal during the run: 0.0%
- Toolchains: rustc 1.99.1, python 3.11.2, tiktoken (PyPI) 0.14.0, node 22.23.4, go 1.26.0, tiktoken (npm) 1.0.22, gpt-tokenizer (npm) 4.1.0, js-tiktoken (npm) 1.0.21

Token check: **66860 documents** compared with the reference before timing, **0 with different tokens**.

## Headline

Geometric mean over 16 workloads × 2 encodings, 95% interval in brackets.

| | over | new text | fresh caches | same text | slowest row (new text) | rows slower |
|---|---|---|---|---|---|---:|
| Rust | tiktoken's core (tiktoken-rs 0.12.1) | **3.69x** [3.67, 3.71] | **2.98x** [2.95, 3.00] | **6.18x** [6.14, 6.23] | 1.63x o200k wiki-ru | 0 of 32 |
| Rust | bpe-openai 0.3.2 | **2.30x** [2.29, 2.31] | **1.84x** [1.82, 1.85] | **3.85x** [3.82, 3.88] | 1.10x o200k wiki-ru | 0 of 32 |
| Python | tiktoken 0.14.0 | **3.50x** [3.48, 3.52] | **2.95x** [2.92, 2.97] |  | 1.62x o200k wiki-de | 0 of 32 |
| Python, encode() | tiktoken 0.14.0 | **3.39x** [3.36, 3.42] | **2.80x** [2.78, 2.82] |  | 1.70x o200k chat-512B | 0 of 31 |
| JavaScript | tiktoken 1.0.22 (npm) | **8.76x** [8.66, 8.86] | **7.47x** [7.41, 7.53] | **13.66x** [13.51, 13.83] | 4.40x o200k chat-512B | 0 of 32 |
| JavaScript | gpt-tokenizer 4.1.0 | **11.09x** [10.96, 11.22] | **12.78x** [12.63, 12.93] | **3.32x** [3.28, 3.36] | 3.29x cl100k td-code-files | 0 of 32 |
| JavaScript | js-tiktoken 1.0.21 | **54.44x** [53.75, 55.15] | **52.28x** [51.79, 52.72] | **84.13x** [83.06, 85.16] | 18.01x cl100k book-mobydick | 0 of 32 |
| Node, native | tiktoken 1.0.22 (npm) | **10.62x** [10.51, 10.72] | **8.71x** [8.64, 8.78] | **16.15x** [15.96, 16.32] | 5.07x o200k chat-512B | 0 of 32 |
| Node, native | gpt-tokenizer 4.1.0 | **13.44x** [13.29, 13.59] | **14.90x** [14.72, 15.09] | **3.92x** [3.88, 3.97] | 5.16x cl100k td-code-files | 0 of 32 |
| Go | pkoukk/tiktoken-go (200cf60, vendored) | **6.72x** [6.60, 6.84] |  |  | 2.99x cl100k book-hongloumeng | 0 of 32 |
| Go | tiktoken-go/tokenizer 0.8.1 | **4.17x** [4.09, 4.26] |  |  | 2.12x o200k book-hongloumeng | 0 of 32 |

Documents where another library's tokens differ from its reference (timed anyway): go/tokenizer 599, js/gpt-tokenizer 0, js/js-tiktoken 0, rust/bpe-openai 0.

## Every row

Rows marked * are outside the headline.

| | over | encoding | workload | scenario | rounds | speed-up | 95% interval |
|---|---|---|---|---|---:|---:|---|
| Rust | tiktoken-rs | cl100k_base | book-gosrc | repeat | 11 | 15.30x | [14.62, 15.98] |
| Rust | tiktoken-rs | cl100k_base | book-gosrc | stream | 11 | 4.93x | [4.81, 5.04] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | repeat | 11 | 11.14x | [10.80, 11.49] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | stream | 11 | 11.03x | [10.65, 11.40] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | repeat | 11 | 13.66x | [12.95, 14.37] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | stream | 11 | 3.23x | [3.16, 3.28] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | cold | 11 | 2.37x | [2.26, 2.48] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | repeat | 11 | 3.03x | [3.00, 3.07] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | stream | 11 | 2.75x | [2.62, 2.84] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | cold | 11 | 8.77x | [8.61, 8.94] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | repeat | 11 | 12.57x | [12.39, 12.71] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | stream | 11 | 8.59x | [8.53, 8.65] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | repeat | 11 | 17.88x | [17.76, 17.99] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | stream | 11 | 15.95x | [15.88, 16.02] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | cold | 11 | 2.92x | [2.85, 3.00] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | repeat | 11 | 6.27x | [5.96, 6.52] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | stream | 11 | 4.64x | [4.52, 4.74] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | cold | 11 | 1.57x | [1.41, 1.76] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | repeat | 11 | 3.02x | [2.98, 3.05] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | stream | 11 | 2.50x | [2.47, 2.53] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | cold | 11 | 3.64x | [3.46, 3.78] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | repeat | 11 | 6.85x | [6.67, 6.96] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | stream | 11 | 4.29x | [4.17, 4.39] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | cold | 11 | 2.19x | [2.12, 2.25] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | repeat | 11 | 4.27x | [4.03, 4.53] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | stream | 11 | 3.21x | [3.16, 3.25] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | cold | 11 | 2.26x | [2.25, 2.28] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | repeat | 11 | 3.82x | [3.59, 4.08] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | stream | 11 | 2.77x | [2.58, 2.99] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | cold | 11 | 5.19x | [5.18, 5.20] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | repeat | 11 | 12.35x | [12.25, 12.41] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | stream | 11 | 8.34x | [8.31, 8.37] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | cold | 11 | 6.66x | [6.63, 6.69] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | repeat | 11 | 13.30x | [12.67, 13.84] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | stream | 11 | 6.73x | [6.65, 6.79] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | cold | 11 | 2.93x | [2.90, 2.97] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | repeat | 11 | 9.18x | [8.65, 9.51] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | stream | 11 | 3.50x | [3.47, 3.53] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | cold | 11 | 2.82x | [2.73, 2.91] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | repeat | 11 | 5.08x | [5.02, 5.14] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | stream | 11 | 3.59x | [3.53, 3.63] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | cold | 11 | 8.37x | [8.34, 8.40] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | repeat | 11 | 15.05x | [14.88, 15.18] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | stream | 11 | 9.22x | [9.18, 9.27] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | cold | 11 | 7.28x | [6.92, 7.56] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | repeat | 11 | 17.08x | [16.36, 17.81] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | stream | 11 | 7.43x | [6.90, 7.87] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | repeat | 11 | 8.75x | [8.39, 9.11] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | stream | 11 | 4.08x | [3.94, 4.17] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | repeat | 11 | 5.44x | [5.33, 5.57] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | stream | 11 | 5.35x | [5.20, 5.50] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | repeat | 11 | 10.15x | [9.48, 10.90] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | stream | 11 | 2.20x | [2.10, 2.31] |
| Rust | tiktoken-rs | o200k_base | chat-512B | cold | 11 | 1.68x | [1.58, 1.80] |
| Rust | tiktoken-rs | o200k_base | chat-512B | repeat | 11 | 1.97x | [1.81, 2.14] |
| Rust | tiktoken-rs | o200k_base | chat-512B | stream | 11 | 1.77x | [1.66, 1.87] |
| Rust | tiktoken-rs | o200k_base | td-code-files | cold | 11 | 5.83x | [5.79, 5.86] |
| Rust | tiktoken-rs | o200k_base | td-code-files | repeat | 11 | 8.09x | [8.04, 8.15] |
| Rust | tiktoken-rs | o200k_base | td-code-files | stream | 11 | 6.06x | [5.92, 6.16] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | repeat | 11 | 10.26x | [10.16, 10.34] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | stream | 11 | 9.20x | [9.12, 9.27] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | cold | 11 | 1.66x | [1.64, 1.68] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | repeat | 11 | 2.68x | [2.61, 2.74] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | stream | 11 | 1.83x | [1.81, 1.85] |
| Rust | tiktoken-rs | o200k_base | wiki-de | cold | 11 | 1.49x | [1.46, 1.51] |
| Rust | tiktoken-rs | o200k_base | wiki-de | repeat | 11 | 1.81x | [1.75, 1.89] |
| Rust | tiktoken-rs | o200k_base | wiki-de | stream | 11 | 1.74x | [1.69, 1.79] |
| Rust | tiktoken-rs | o200k_base | wiki-en | cold | 11 | 2.35x | [2.28, 2.40] |
| Rust | tiktoken-rs | o200k_base | wiki-en | repeat | 11 | 4.64x | [4.60, 4.67] |
| Rust | tiktoken-rs | o200k_base | wiki-en | stream | 11 | 2.73x | [2.70, 2.77] |
| Rust | tiktoken-rs | o200k_base | wiki-es | cold | 11 | 1.58x | [1.46, 1.70] |
| Rust | tiktoken-rs | o200k_base | wiki-es | repeat | 11 | 3.51x | [3.48, 3.54] |
| Rust | tiktoken-rs | o200k_base | wiki-es | stream | 11 | 2.10x | [2.07, 2.13] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | cold | 11 | 1.64x | [1.62, 1.65] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | repeat | 11 | 2.76x | [2.69, 2.82] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | stream | 11 | 2.00x | [1.93, 2.06] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | cold | 11 | 2.01x | [1.99, 2.02] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | repeat | 11 | 3.44x | [3.41, 3.46] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | stream | 11 | 2.35x | [2.32, 2.38] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | cold | 11 | 4.41x | [4.39, 4.43] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | repeat | 11 | 9.07x | [8.67, 9.49] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | stream | 11 | 5.24x | [5.19, 5.29] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | cold | 11 | 1.97x | [1.94, 2.00] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | repeat | 11 | 5.84x | [5.48, 6.07] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | stream | 11 | 2.14x | [2.12, 2.16] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | cold | 11 | 1.87x | [1.78, 1.94] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | repeat | 11 | 2.88x | [2.80, 2.96] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | stream | 11 | 1.63x | [1.60, 1.66] |
| Rust | tiktoken-rs | o200k_base | wiki-th | cold | 11 | 3.28x | [3.23, 3.33] |
| Rust | tiktoken-rs | o200k_base | wiki-th | repeat | 11 | 6.11x | [5.95, 6.21] |
| Rust | tiktoken-rs | o200k_base | wiki-th | stream | 11 | 4.26x | [4.23, 4.29] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | cold | 11 | 4.36x | [4.23, 4.49] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | repeat | 11 | 10.61x | [10.36, 10.79] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | stream | 11 | 4.68x | [4.48, 4.87] |
| Rust | bpe-openai | cl100k_base | book-gosrc | repeat | 11 | 6.76x | [6.50, 6.99] |
| Rust | bpe-openai | cl100k_base | book-gosrc | stream | 11 | 2.24x | [2.22, 2.27] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | repeat | 11 | 6.08x | [5.92, 6.23] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | stream | 11 | 6.03x | [5.85, 6.22] |
| Rust | bpe-openai | cl100k_base | book-mobydick | repeat | 11 | 9.08x | [8.57, 9.52] |
| Rust | bpe-openai | cl100k_base | book-mobydick | stream | 11 | 2.23x | [2.18, 2.27] |
| Rust | bpe-openai | cl100k_base | chat-512B | cold | 11 | 1.59x | [1.51, 1.67] |
| Rust | bpe-openai | cl100k_base | chat-512B | repeat | 11 | 1.91x | [1.88, 1.95] |
| Rust | bpe-openai | cl100k_base | chat-512B | stream | 11 | 1.71x | [1.64, 1.77] |
| Rust | bpe-openai | cl100k_base | td-code-files | cold | 11 | 3.57x | [3.53, 3.62] |
| Rust | bpe-openai | cl100k_base | td-code-files | repeat | 11 | 5.08x | [5.04, 5.12] |
| Rust | bpe-openai | cl100k_base | td-code-files | stream | 11 | 3.58x | [3.55, 3.60] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | repeat | 11 | 9.93x | [9.85, 10.00] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | stream | 11 | 8.89x | [8.85, 8.93] |
| Rust | bpe-openai | cl100k_base | wiki-ar | cold | 11 | 1.45x | [1.42, 1.48] |
| Rust | bpe-openai | cl100k_base | wiki-ar | repeat | 11 | 3.12x | [2.97, 3.24] |
| Rust | bpe-openai | cl100k_base | wiki-ar | stream | 11 | 2.32x | [2.25, 2.36] |
| Rust | bpe-openai | cl100k_base | wiki-de | cold | 11 | 1.25x | [1.14, 1.39] |
| Rust | bpe-openai | cl100k_base | wiki-de | repeat | 11 | 2.13x | [2.11, 2.15] |
| Rust | bpe-openai | cl100k_base | wiki-de | stream | 11 | 1.76x | [1.74, 1.78] |
| Rust | bpe-openai | cl100k_base | wiki-en | cold | 11 | 2.32x | [2.21, 2.41] |
| Rust | bpe-openai | cl100k_base | wiki-en | repeat | 11 | 4.30x | [4.20, 4.36] |
| Rust | bpe-openai | cl100k_base | wiki-en | stream | 11 | 2.72x | [2.65, 2.81] |
| Rust | bpe-openai | cl100k_base | wiki-es | cold | 11 | 1.50x | [1.48, 1.51] |
| Rust | bpe-openai | cl100k_base | wiki-es | repeat | 11 | 3.17x | [2.96, 3.41] |
| Rust | bpe-openai | cl100k_base | wiki-es | stream | 11 | 2.12x | [2.09, 2.16] |
| Rust | bpe-openai | cl100k_base | wiki-fr | cold | 11 | 1.52x | [1.50, 1.53] |
| Rust | bpe-openai | cl100k_base | wiki-fr | repeat | 11 | 2.71x | [2.58, 2.83] |
| Rust | bpe-openai | cl100k_base | wiki-fr | stream | 11 | 1.96x | [1.84, 2.07] |
| Rust | bpe-openai | cl100k_base | wiki-hi | cold | 11 | 2.27x | [2.26, 2.27] |
| Rust | bpe-openai | cl100k_base | wiki-hi | repeat | 11 | 5.39x | [5.35, 5.42] |
| Rust | bpe-openai | cl100k_base | wiki-hi | stream | 11 | 3.65x | [3.64, 3.66] |
| Rust | bpe-openai | cl100k_base | wiki-ja | cold | 11 | 3.23x | [3.21, 3.24] |
| Rust | bpe-openai | cl100k_base | wiki-ja | repeat | 11 | 6.48x | [6.17, 6.71] |
| Rust | bpe-openai | cl100k_base | wiki-ja | stream | 11 | 3.29x | [3.25, 3.32] |
| Rust | bpe-openai | cl100k_base | wiki-ko | cold | 11 | 1.48x | [1.46, 1.49] |
| Rust | bpe-openai | cl100k_base | wiki-ko | repeat | 11 | 4.61x | [4.34, 4.77] |
| Rust | bpe-openai | cl100k_base | wiki-ko | stream | 11 | 1.77x | [1.76, 1.78] |
| Rust | bpe-openai | cl100k_base | wiki-ru | cold | 11 | 1.34x | [1.30, 1.38] |
| Rust | bpe-openai | cl100k_base | wiki-ru | repeat | 11 | 2.42x | [2.39, 2.44] |
| Rust | bpe-openai | cl100k_base | wiki-ru | stream | 11 | 1.70x | [1.68, 1.73] |
| Rust | bpe-openai | cl100k_base | wiki-th | cold | 11 | 3.68x | [3.67, 3.69] |
| Rust | bpe-openai | cl100k_base | wiki-th | repeat | 11 | 6.63x | [6.57, 6.69] |
| Rust | bpe-openai | cl100k_base | wiki-th | stream | 11 | 4.10x | [4.08, 4.11] |
| Rust | bpe-openai | cl100k_base | wiki-zh | cold | 11 | 3.92x | [3.75, 4.08] |
| Rust | bpe-openai | cl100k_base | wiki-zh | repeat | 11 | 9.16x | [8.74, 9.54] |
| Rust | bpe-openai | cl100k_base | wiki-zh | stream | 11 | 4.01x | [3.75, 4.24] |
| Rust | bpe-openai | o200k_base | book-gosrc | repeat | 11 | 6.36x | [6.03, 6.70] |
| Rust | bpe-openai | o200k_base | book-gosrc | stream | 11 | 3.05x | [2.98, 3.12] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | repeat | 11 | 3.26x | [3.21, 3.30] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | stream | 11 | 3.21x | [3.16, 3.26] |
| Rust | bpe-openai | o200k_base | book-mobydick | repeat | 11 | 10.17x | [9.38, 11.09] |
| Rust | bpe-openai | o200k_base | book-mobydick | stream | 11 | 2.37x | [2.29, 2.46] |
| Rust | bpe-openai | o200k_base | chat-512B | cold | 11 | 1.48x | [1.41, 1.54] |
| Rust | bpe-openai | o200k_base | chat-512B | repeat | 11 | 1.68x | [1.50, 1.87] |
| Rust | bpe-openai | o200k_base | chat-512B | stream | 11 | 1.54x | [1.45, 1.64] |
| Rust | bpe-openai | o200k_base | td-code-files | cold | 11 | 3.75x | [3.72, 3.77] |
| Rust | bpe-openai | o200k_base | td-code-files | repeat | 11 | 5.22x | [5.18, 5.26] |
| Rust | bpe-openai | o200k_base | td-code-files | stream | 11 | 3.93x | [3.86, 3.98] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | repeat | 11 | 10.09x | [10.00, 10.18] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | stream | 11 | 9.06x | [8.98, 9.12] |
| Rust | bpe-openai | o200k_base | wiki-ar | cold | 11 | 1.19x | [1.18, 1.20] |
| Rust | bpe-openai | o200k_base | wiki-ar | repeat | 11 | 1.92x | [1.88, 1.96] |
| Rust | bpe-openai | o200k_base | wiki-ar | stream | 11 | 1.32x | [1.30, 1.33] |
| Rust | bpe-openai | o200k_base | wiki-de | cold | 11 | 1.43x | [1.39, 1.46] |
| Rust | bpe-openai | o200k_base | wiki-de | repeat | 11 | 1.91x | [1.87, 1.94] |
| Rust | bpe-openai | o200k_base | wiki-de | stream | 11 | 1.67x | [1.61, 1.71] |
| Rust | bpe-openai | o200k_base | wiki-en | cold | 11 | 2.35x | [2.25, 2.46] |
| Rust | bpe-openai | o200k_base | wiki-en | repeat | 11 | 4.64x | [4.60, 4.69] |
| Rust | bpe-openai | o200k_base | wiki-en | stream | 11 | 2.73x | [2.71, 2.76] |
| Rust | bpe-openai | o200k_base | wiki-es | cold | 11 | 1.54x | [1.42, 1.65] |
| Rust | bpe-openai | o200k_base | wiki-es | repeat | 11 | 3.49x | [3.46, 3.52] |
| Rust | bpe-openai | o200k_base | wiki-es | stream | 11 | 2.06x | [2.04, 2.08] |
| Rust | bpe-openai | o200k_base | wiki-fr | cold | 11 | 1.61x | [1.59, 1.63] |
| Rust | bpe-openai | o200k_base | wiki-fr | repeat | 11 | 2.71x | [2.66, 2.75] |
| Rust | bpe-openai | o200k_base | wiki-fr | stream | 11 | 2.00x | [1.94, 2.05] |
| Rust | bpe-openai | o200k_base | wiki-hi | cold | 11 | 1.29x | [1.29, 1.30] |
| Rust | bpe-openai | o200k_base | wiki-hi | repeat | 11 | 2.21x | [2.20, 2.23] |
| Rust | bpe-openai | o200k_base | wiki-hi | stream | 11 | 1.51x | [1.49, 1.53] |
| Rust | bpe-openai | o200k_base | wiki-ja | cold | 11 | 2.26x | [2.25, 2.26] |
| Rust | bpe-openai | o200k_base | wiki-ja | repeat | 11 | 4.72x | [4.54, 4.90] |
| Rust | bpe-openai | o200k_base | wiki-ja | stream | 11 | 2.69x | [2.66, 2.71] |
| Rust | bpe-openai | o200k_base | wiki-ko | cold | 11 | 1.19x | [1.17, 1.20] |
| Rust | bpe-openai | o200k_base | wiki-ko | repeat | 11 | 3.49x | [3.28, 3.63] |
| Rust | bpe-openai | o200k_base | wiki-ko | stream | 11 | 1.29x | [1.28, 1.30] |
| Rust | bpe-openai | o200k_base | wiki-ru | cold | 11 | 1.19x | [1.15, 1.23] |
| Rust | bpe-openai | o200k_base | wiki-ru | repeat | 11 | 1.94x | [1.89, 2.00] |
| Rust | bpe-openai | o200k_base | wiki-ru | stream | 11 | 1.10x | [1.08, 1.13] |
| Rust | bpe-openai | o200k_base | wiki-th | cold | 11 | 1.13x | [1.11, 1.14] |
| Rust | bpe-openai | o200k_base | wiki-th | repeat | 11 | 2.06x | [2.00, 2.10] |
| Rust | bpe-openai | o200k_base | wiki-th | stream | 11 | 1.43x | [1.43, 1.44] |
| Rust | bpe-openai | o200k_base | wiki-zh | cold | 11 | 2.48x | [2.43, 2.52] |
| Rust | bpe-openai | o200k_base | wiki-zh | repeat | 11 | 5.99x | [5.84, 6.10] |
| Rust | bpe-openai | o200k_base | wiki-zh | stream | 11 | 2.68x | [2.59, 2.78] |
| Python | tiktoken | cl100k_base | book-gosrc | stream | 11 | 5.30x | [5.12, 5.49] |
| Python | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 6.39x | [6.24, 6.54] |
| Python | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.67x | [3.44, 3.90] |
| Python | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.36x | [2.11, 2.63] |
| Python | tiktoken | cl100k_base | chat-512B | stream | 11 | 2.84x | [2.74, 2.94] |
| Python | tiktoken | cl100k_base | td-code-files | cold | 11 | 8.68x | [8.52, 8.85] |
| Python | tiktoken | cl100k_base | td-code-files | stream | 11 | 8.91x | [8.76, 9.09] |
| Python | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 11.47x | [11.40, 11.53] |
| Python | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.97x | [2.91, 3.05] |
| Python | tiktoken | cl100k_base | wiki-ar | stream | 11 | 3.87x | [3.84, 3.90] |
| Python | tiktoken | cl100k_base | wiki-de | cold | 11 | 2.18x | [2.15, 2.21] |
| Python | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.67x | [2.63, 2.71] |
| Python | tiktoken | cl100k_base | wiki-en | cold | 11 | 3.52x | [3.28, 3.76] |
| Python | tiktoken | cl100k_base | wiki-en | stream | 11 | 4.43x | [4.33, 4.54] |
| Python | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.31x | [2.28, 2.34] |
| Python | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.33x | [3.25, 3.40] |
| Python | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.28x | [2.25, 2.31] |
| Python | tiktoken | cl100k_base | wiki-fr | stream | 11 | 3.04x | [2.98, 3.09] |
| Python | tiktoken | cl100k_base | wiki-hi | cold | 11 | 4.33x | [4.30, 4.36] |
| Python | tiktoken | cl100k_base | wiki-hi | stream | 11 | 5.98x | [5.93, 6.02] |
| Python | tiktoken | cl100k_base | wiki-ja | cold | 11 | 4.80x | [4.75, 4.85] |
| Python | tiktoken | cl100k_base | wiki-ja | stream | 11 | 5.33x | [5.12, 5.57] |
| Python | tiktoken | cl100k_base | wiki-ko | cold | 11 | 2.71x | [2.57, 2.84] |
| Python | tiktoken | cl100k_base | wiki-ko | stream | 11 | 2.82x | [2.78, 2.86] |
| Python | tiktoken | cl100k_base | wiki-ru | cold | 11 | 2.80x | [2.74, 2.85] |
| Python | tiktoken | cl100k_base | wiki-ru | stream | 11 | 2.62x | [2.60, 2.64] |
| Python | tiktoken | cl100k_base | wiki-th | cold | 11 | 6.09x | [6.03, 6.16] |
| Python | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.71x | [6.63, 6.77] |
| Python | tiktoken | cl100k_base | wiki-zh | cold | 11 | 5.33x | [5.26, 5.40] |
| Python | tiktoken | cl100k_base | wiki-zh | stream | 11 | 5.37x | [5.30, 5.44] |
| Python | tiktoken | o200k_base | book-gosrc | stream | 11 | 4.21x | [4.13, 4.29] |
| Python | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 5.08x | [4.95, 5.21] |
| Python | tiktoken | o200k_base | book-mobydick | stream | 11 | 2.23x | [2.14, 2.33] |
| Python | tiktoken | o200k_base | chat-512B | cold | 11 | 1.81x | [1.72, 1.90] |
| Python | tiktoken | o200k_base | chat-512B | stream | 11 | 1.92x | [1.86, 1.98] |
| Python | tiktoken | o200k_base | td-code-files | cold | 11 | 5.74x | [5.68, 5.81] |
| Python | tiktoken | o200k_base | td-code-files | stream | 11 | 5.89x | [5.82, 5.98] |
| Python | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 6.78x | [6.75, 6.81] |
| Python | tiktoken | o200k_base | wiki-ar | cold | 11 | 1.87x | [1.83, 1.92] |
| Python | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.02x | [1.96, 2.07] |
| Python | tiktoken | o200k_base | wiki-de | cold | 11 | 1.65x | [1.59, 1.71] |
| Python | tiktoken | o200k_base | wiki-de | stream | 11 | 1.62x | [1.59, 1.64] |
| Python | tiktoken | o200k_base | wiki-en | cold | 11 | 2.34x | [2.22, 2.46] |
| Python | tiktoken | o200k_base | wiki-en | stream | 11 | 2.30x | [2.20, 2.40] |
| Python | tiktoken | o200k_base | wiki-es | cold | 11 | 1.88x | [1.84, 1.93] |
| Python | tiktoken | o200k_base | wiki-es | stream | 11 | 2.00x | [1.94, 2.08] |
| Python | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.63x | [1.61, 1.66] |
| Python | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.14x | [2.02, 2.27] |
| Python | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.15x | [2.09, 2.21] |
| Python | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.54x | [2.47, 2.61] |
| Python | tiktoken | o200k_base | wiki-ja | cold | 11 | 3.71x | [3.64, 3.78] |
| Python | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.56x | [4.38, 4.76] |
| Python | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.18x | [2.11, 2.27] |
| Python | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.30x | [2.21, 2.40] |
| Python | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.06x | [2.01, 2.12] |
| Python | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.35x | [2.31, 2.39] |
| Python | tiktoken | o200k_base | wiki-th | cold | 11 | 3.57x | [3.51, 3.63] |
| Python | tiktoken | o200k_base | wiki-th | stream | 11 | 4.08x | [4.01, 4.15] |
| Python | tiktoken | o200k_base | wiki-zh | cold | 11 | 4.20x | [4.09, 4.31] |
| Python | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.63x | [4.53, 4.74] |
| Python, encode() | tiktoken | cl100k_base | book-gosrc | stream | 11 | 4.94x | [4.81, 5.07] |
| Python, encode() | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 6.37x | [6.18, 6.56] |
| Python, encode() | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.65x | [3.42, 3.89] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.24x | [1.93, 2.58] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | stream | 11 | 2.68x | [2.52, 2.84] |
| Python, encode() | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 11.42x | [11.32, 11.53] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.99x | [2.92, 3.06] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | stream | 11 | 4.19x | [4.08, 4.30] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | cold | 11 | 2.15x | [2.10, 2.18] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.52x | [2.41, 2.63] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | cold | 11 | 4.05x | [3.96, 4.15] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | stream | 11 | 4.54x | [4.41, 4.68] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.49x | [2.46, 2.53] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.38x | [3.30, 3.46] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.12x | [2.04, 2.20] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | stream | 11 | 2.67x | [2.52, 2.83] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | cold | 11 | 4.28x | [4.26, 4.30] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | stream | 11 | 5.96x | [5.90, 6.01] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | cold | 11 | 5.13x | [5.07, 5.21] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | stream | 11 | 5.11x | [5.00, 5.22] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | cold | 11 | 2.55x | [2.51, 2.60] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | stream | 11 | 2.78x | [2.73, 2.83] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | cold | 11 | 1.88x | [1.86, 1.90] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | stream | 11 | 3.22x | [2.89, 3.46] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | cold | 11 | 6.38x | [6.30, 6.47] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.83x | [6.68, 6.96] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | cold | 11 | 5.05x | [5.00, 5.10] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | stream | 11 | 4.87x | [4.79, 4.95] |
| Python, encode() | tiktoken | o200k_base | book-gosrc | stream | 11 | 4.45x | [4.26, 4.66] |
| Python, encode() | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 5.22x | [5.06, 5.39] |
| Python, encode() | tiktoken | o200k_base | book-mobydick | stream | 11 | 2.18x | [2.08, 2.28] |
| Python, encode() | tiktoken | o200k_base | chat-512B | cold | 11 | 1.79x | [1.71, 1.87] |
| Python, encode() | tiktoken | o200k_base | chat-512B | stream | 11 | 1.70x | [1.52, 1.90] |
| Python, encode() | tiktoken | o200k_base | td-code-files | cold | 11 | 5.77x | [5.70, 5.85] |
| Python, encode() | tiktoken | o200k_base | td-code-files | stream | 11 | 5.92x | [5.83, 6.02] |
| Python, encode() | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 6.79x | [6.73, 6.84] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | cold | 11 | 1.88x | [1.84, 1.92] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.01x | [1.96, 2.06] |
| Python, encode() | tiktoken | o200k_base | wiki-de | cold | 11 | 1.74x | [1.71, 1.77] |
| Python, encode() | tiktoken | o200k_base | wiki-de | stream | 11 | 1.76x | [1.67, 1.87] |
| Python, encode() | tiktoken | o200k_base | wiki-en | cold | 11 | 2.51x | [2.37, 2.66] |
| Python, encode() | tiktoken | o200k_base | wiki-en | stream | 11 | 2.62x | [2.41, 2.86] |
| Python, encode() | tiktoken | o200k_base | wiki-es | cold | 11 | 1.70x | [1.64, 1.75] |
| Python, encode() | tiktoken | o200k_base | wiki-es | stream | 11 | 2.22x | [2.03, 2.41] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.71x | [1.64, 1.80] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.03x | [1.90, 2.18] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.18x | [2.12, 2.24] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.56x | [2.49, 2.63] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | cold | 11 | 4.13x | [4.05, 4.21] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.88x | [4.77, 5.00] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.24x | [2.16, 2.33] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.32x | [2.23, 2.42] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.10x | [2.05, 2.15] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.25x | [2.21, 2.29] |
| Python, encode() | tiktoken | o200k_base | wiki-th | cold | 11 | 3.57x | [3.51, 3.63] |
| Python, encode() | tiktoken | o200k_base | wiki-th | stream | 11 | 4.09x | [4.03, 4.16] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | cold | 11 | 3.86x | [3.80, 3.94] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.07x | [3.97, 4.17] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 24.92x | [23.30, 26.19] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | stream | 11 | 11.92x | [10.48, 13.16] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 15.38x | [15.16, 15.62] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 12.78x | [10.91, 14.38] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 23.00x | [21.45, 25.04] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | stream | 11 | 8.45x | [8.21, 8.72] |
| JavaScript | tiktoken | cl100k_base | chat-512B | cold | 11 | 5.33x | [5.04, 5.62] |
| JavaScript | tiktoken | cl100k_base | chat-512B | repeat | 11 | 5.52x | [4.73, 6.17] |
| JavaScript | tiktoken | cl100k_base | chat-512B | stream | 11 | 5.02x | [4.42, 5.60] |
| JavaScript | tiktoken | cl100k_base | td-code-files | cold | 11 | 22.91x | [21.81, 23.77] |
| JavaScript | tiktoken | cl100k_base | td-code-files | repeat | 11 | 26.70x | [25.15, 28.54] |
| JavaScript | tiktoken | cl100k_base | td-code-files | stream | 11 | 21.19x | [20.19, 22.39] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 23.30x | [20.26, 27.61] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 31.28x | [27.34, 34.80] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | cold | 11 | 5.93x | [5.83, 6.03] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 13.05x | [12.58, 13.40] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | stream | 11 | 8.60x | [8.27, 8.89] |
| JavaScript | tiktoken | cl100k_base | wiki-de | cold | 11 | 5.30x | [5.13, 5.42] |
| JavaScript | tiktoken | cl100k_base | wiki-de | repeat | 11 | 7.60x | [7.18, 7.97] |
| JavaScript | tiktoken | cl100k_base | wiki-de | stream | 11 | 6.18x | [5.63, 6.60] |
| JavaScript | tiktoken | cl100k_base | wiki-en | cold | 11 | 10.73x | [10.32, 11.12] |
| JavaScript | tiktoken | cl100k_base | wiki-en | repeat | 11 | 18.33x | [17.64, 19.01] |
| JavaScript | tiktoken | cl100k_base | wiki-en | stream | 11 | 11.84x | [11.24, 12.38] |
| JavaScript | tiktoken | cl100k_base | wiki-es | cold | 11 | 6.16x | [6.00, 6.28] |
| JavaScript | tiktoken | cl100k_base | wiki-es | repeat | 11 | 13.18x | [12.48, 13.80] |
| JavaScript | tiktoken | cl100k_base | wiki-es | stream | 11 | 8.02x | [7.32, 8.63] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | cold | 11 | 6.13x | [5.95, 6.25] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 11.04x | [10.63, 11.37] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | stream | 11 | 8.36x | [8.05, 8.56] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | cold | 11 | 10.12x | [10.04, 10.19] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 21.91x | [21.54, 22.22] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | stream | 11 | 15.20x | [14.86, 15.46] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | cold | 11 | 11.02x | [10.83, 11.21] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 20.21x | [19.53, 20.81] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | stream | 11 | 12.03x | [11.54, 12.39] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | cold | 11 | 5.82x | [5.63, 5.97] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 15.83x | [14.96, 16.50] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | stream | 11 | 5.82x | [5.50, 6.16] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | cold | 11 | 5.26x | [5.08, 5.44] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 9.74x | [9.28, 10.14] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | stream | 11 | 6.49x | [6.32, 6.65] |
| JavaScript | tiktoken | cl100k_base | wiki-th | cold | 11 | 13.52x | [13.19, 13.81] |
| JavaScript | tiktoken | cl100k_base | wiki-th | repeat | 11 | 22.69x | [22.52, 22.85] |
| JavaScript | tiktoken | cl100k_base | wiki-th | stream | 11 | 15.24x | [14.91, 15.55] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | cold | 11 | 11.56x | [10.75, 12.15] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 24.79x | [24.15, 25.23] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | stream | 11 | 12.01x | [10.91, 12.81] |
| JavaScript | tiktoken | o200k_base | book-gosrc | repeat | 11 | 19.02x | [16.22, 22.65] |
| JavaScript | tiktoken | o200k_base | book-gosrc | stream | 11 | 15.45x | [13.98, 16.84] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 9.46x | [9.31, 9.62] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 8.14x | [7.66, 8.71] |
| JavaScript | tiktoken | o200k_base | book-mobydick | repeat | 11 | 17.46x | [14.52, 21.25] |
| JavaScript | tiktoken | o200k_base | book-mobydick | stream | 11 | 8.14x | [7.69, 8.61] |
| JavaScript | tiktoken | o200k_base | chat-512B | cold | 11 | 4.61x | [4.44, 4.78] |
| JavaScript | tiktoken | o200k_base | chat-512B | repeat | 11 | 4.65x | [4.07, 5.05] |
| JavaScript | tiktoken | o200k_base | chat-512B | stream | 11 | 4.40x | [4.09, 4.66] |
| JavaScript | tiktoken | o200k_base | td-code-files | cold | 11 | 23.22x | [21.65, 24.20] |
| JavaScript | tiktoken | o200k_base | td-code-files | repeat | 11 | 28.64x | [26.76, 30.48] |
| JavaScript | tiktoken | o200k_base | td-code-files | stream | 11 | 23.13x | [21.80, 24.27] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 25.92x | [23.34, 29.40] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 33.22x | [28.64, 37.08] |
| JavaScript | tiktoken | o200k_base | wiki-ar | cold | 11 | 4.55x | [4.21, 4.80] |
| JavaScript | tiktoken | o200k_base | wiki-ar | repeat | 11 | 7.10x | [6.61, 7.54] |
| JavaScript | tiktoken | o200k_base | wiki-ar | stream | 11 | 5.23x | [5.11, 5.34] |
| JavaScript | tiktoken | o200k_base | wiki-de | cold | 11 | 4.67x | [4.36, 4.97] |
| JavaScript | tiktoken | o200k_base | wiki-de | repeat | 11 | 6.56x | [6.10, 6.95] |
| JavaScript | tiktoken | o200k_base | wiki-de | stream | 11 | 5.63x | [5.23, 6.00] |
| JavaScript | tiktoken | o200k_base | wiki-en | cold | 11 | 9.68x | [9.32, 10.04] |
| JavaScript | tiktoken | o200k_base | wiki-en | repeat | 11 | 17.51x | [16.84, 18.23] |
| JavaScript | tiktoken | o200k_base | wiki-en | stream | 11 | 9.60x | [9.21, 9.98] |
| JavaScript | tiktoken | o200k_base | wiki-es | cold | 11 | 6.33x | [5.93, 6.67] |
| JavaScript | tiktoken | o200k_base | wiki-es | repeat | 11 | 11.49x | [10.53, 12.43] |
| JavaScript | tiktoken | o200k_base | wiki-es | stream | 11 | 7.26x | [6.77, 7.74] |
| JavaScript | tiktoken | o200k_base | wiki-fr | cold | 11 | 6.27x | [5.94, 6.56] |
| JavaScript | tiktoken | o200k_base | wiki-fr | repeat | 11 | 10.06x | [9.58, 10.55] |
| JavaScript | tiktoken | o200k_base | wiki-fr | stream | 11 | 7.40x | [6.87, 7.95] |
| JavaScript | tiktoken | o200k_base | wiki-hi | cold | 11 | 5.00x | [4.82, 5.17] |
| JavaScript | tiktoken | o200k_base | wiki-hi | repeat | 11 | 8.22x | [7.71, 8.70] |
| JavaScript | tiktoken | o200k_base | wiki-hi | stream | 11 | 5.50x | [5.13, 5.83] |
| JavaScript | tiktoken | o200k_base | wiki-ja | cold | 11 | 8.60x | [8.45, 8.73] |
| JavaScript | tiktoken | o200k_base | wiki-ja | repeat | 11 | 17.20x | [16.63, 17.66] |
| JavaScript | tiktoken | o200k_base | wiki-ja | stream | 11 | 9.43x | [9.00, 9.81] |
| JavaScript | tiktoken | o200k_base | wiki-ko | cold | 11 | 5.39x | [5.31, 5.47] |
| JavaScript | tiktoken | o200k_base | wiki-ko | repeat | 11 | 14.13x | [13.34, 14.75] |
| JavaScript | tiktoken | o200k_base | wiki-ko | stream | 11 | 5.58x | [5.47, 5.69] |
| JavaScript | tiktoken | o200k_base | wiki-ru | cold | 11 | 4.72x | [4.49, 4.85] |
| JavaScript | tiktoken | o200k_base | wiki-ru | repeat | 11 | 8.66x | [8.50, 8.79] |
| JavaScript | tiktoken | o200k_base | wiki-ru | stream | 11 | 5.09x | [4.64, 5.37] |
| JavaScript | tiktoken | o200k_base | wiki-th | cold | 11 | 7.80x | [7.35, 8.08] |
| JavaScript | tiktoken | o200k_base | wiki-th | repeat | 11 | 12.80x | [12.35, 13.10] |
| JavaScript | tiktoken | o200k_base | wiki-th | stream | 11 | 9.27x | [9.22, 9.33] |
| JavaScript | tiktoken | o200k_base | wiki-zh | cold | 11 | 8.27x | [7.89, 8.55] |
| JavaScript | tiktoken | o200k_base | wiki-zh | repeat | 11 | 18.78x | [18.42, 19.05] |
| JavaScript | tiktoken | o200k_base | wiki-zh | stream | 11 | 9.20x | [8.64, 9.63] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 5.88x | [5.48, 6.22] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 4.91x | [4.50, 5.22] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.14x | [3.08, 3.21] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 44.15x | [37.02, 50.29] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 6.19x | [5.86, 6.60] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.81x | [5.33, 6.40] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 14.13x | [13.26, 15.12] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 11.94x | [10.23, 13.49] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 10.04x | [8.88, 11.39] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 4.30x | [3.87, 4.94] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 3.27x | [3.10, 3.42] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 3.29x | [3.14, 3.46] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 3.24x | [2.59, 4.17] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 4.85x | [4.08, 5.66] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 16.82x | [16.43, 17.27] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 3.19x | [3.07, 3.36] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 13.07x | [12.60, 13.66] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 7.54x | [7.29, 7.78] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 2.03x | [1.92, 2.19] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 7.08x | [6.70, 7.45] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 5.67x | [5.25, 6.07] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 5.32x | [5.16, 5.49] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 6.13x | [5.88, 6.40] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 7.36x | [6.85, 7.87] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 2.97x | [2.85, 3.07] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 6.67x | [6.40, 6.91] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 8.12x | [7.72, 8.52] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 2.81x | [2.68, 2.91] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 7.04x | [6.82, 7.28] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 10.50x | [9.56, 11.56] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 4.56x | [4.51, 4.62] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 5.44x | [5.28, 5.66] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 39.51x | [38.63, 40.41] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.48x | [3.37, 3.56] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 42.41x | [41.25, 43.39] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 14.83x | [14.45, 15.20] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.27x | [3.10, 3.40] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 12.22x | [11.69, 12.80] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 16.69x | [16.23, 17.16] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 2.15x | [2.05, 2.22] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 13.42x | [13.17, 13.67] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 42.87x | [41.38, 44.47] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 5.44x | [5.30, 5.59] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 34.10x | [33.38, 35.01] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 47.85x | [45.37, 51.67] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 4.74x | [4.58, 4.88] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 44.95x | [40.95, 47.80] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 5.76x | [4.86, 6.85] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 6.17x | [5.49, 6.81] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.18x | [2.14, 2.21] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 34.93x | [33.14, 36.98] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 5.34x | [4.48, 6.51] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 5.51x | [5.17, 5.83] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 11.69x | [11.36, 12.00] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.02x | [0.90, 1.10] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 9.45x | [8.78, 10.08] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 4.35x | [3.88, 5.08] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 3.44x | [3.23, 3.61] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 3.89x | [3.52, 4.26] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 3.13x | [2.65, 3.77] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 4.53x | [3.71, 5.31] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 12.04x | [11.29, 12.66] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.01x | [1.92, 2.10] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 9.38x | [9.14, 9.58] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 7.34x | [6.95, 7.74] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 1.97x | [1.87, 2.06] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 6.79x | [6.41, 7.19] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 5.77x | [5.54, 5.99] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 5.70x | [5.49, 5.91] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 6.52x | [6.12, 7.04] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 6.85x | [6.38, 7.31] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 3.42x | [3.24, 3.60] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 6.40x | [5.94, 6.79] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 7.25x | [6.84, 7.63] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 3.09x | [2.97, 3.20] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 6.48x | [6.02, 6.98] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 11.61x | [11.15, 12.13] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 1.95x | [1.87, 2.02] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 8.74x | [8.30, 9.13] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 34.39x | [33.60, 35.21] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.27x | [3.18, 3.34] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 37.56x | [35.88, 38.91] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 14.72x | [14.31, 15.13] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.55x | [3.36, 3.69] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 12.63x | [12.29, 13.01] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 14.79x | [14.30, 15.13] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.17x | [2.14, 2.20] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 11.92x | [11.10, 12.58] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 25.19x | [23.83, 26.23] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.20x | [1.17, 1.22] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 29.32x | [29.17, 29.46] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 37.57x | [35.85, 39.79] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 3.84x | [3.75, 3.92] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 39.49x | [37.25, 40.95] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | repeat | 11 | 50.87x | [47.74, 53.61] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | stream | 11 | 23.31x | [20.56, 25.53] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 190.80x | [186.74, 195.27] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 158.82x | [136.19, 179.01] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | repeat | 11 | 46.43x | [43.53, 49.82] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | stream | 11 | 18.01x | [17.59, 18.37] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | cold | 11 | 41.03x | [38.79, 43.33] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | repeat | 11 | 43.40x | [37.75, 47.71] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | stream | 11 | 39.08x | [33.66, 44.58] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | cold | 11 | 32.16x | [30.51, 33.84] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | repeat | 11 | 36.89x | [34.46, 39.57] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | stream | 11 | 28.88x | [26.94, 31.17] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 22.16x | [19.06, 26.34] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 28.75x | [25.06, 32.19] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | cold | 11 | 46.39x | [45.39, 47.30] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | repeat | 11 | 101.32x | [97.01, 105.46] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | stream | 11 | 66.70x | [64.47, 68.85] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | cold | 11 | 28.72x | [27.65, 29.65] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | repeat | 11 | 41.37x | [39.78, 42.90] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | stream | 11 | 34.38x | [31.91, 36.55] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | cold | 11 | 22.56x | [21.76, 23.40] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | repeat | 11 | 37.95x | [36.76, 39.16] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | stream | 11 | 25.25x | [23.91, 26.52] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | cold | 11 | 26.74x | [25.84, 27.66] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | repeat | 11 | 55.92x | [53.12, 58.69] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | stream | 11 | 34.02x | [30.93, 36.70] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | cold | 11 | 24.89x | [23.79, 25.82] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | repeat | 11 | 44.14x | [42.22, 45.50] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | stream | 11 | 34.72x | [33.00, 36.09] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | cold | 11 | 43.28x | [42.46, 44.11] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | repeat | 11 | 97.52x | [95.00, 100.07] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | stream | 11 | 68.17x | [67.01, 69.80] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | cold | 11 | 286.47x | [281.50, 291.05] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | repeat | 11 | 525.40x | [506.03, 542.63] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | stream | 11 | 311.32x | [298.39, 320.44] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | cold | 11 | 38.06x | [36.61, 39.18] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | repeat | 11 | 105.26x | [100.02, 109.95] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | stream | 11 | 40.17x | [37.88, 42.70] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | cold | 11 | 56.69x | [54.39, 58.78] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | repeat | 11 | 107.94x | [103.09, 112.52] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | stream | 11 | 69.91x | [68.10, 71.48] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | cold | 11 | 165.97x | [161.55, 169.99] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | repeat | 11 | 281.62x | [278.83, 284.58] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | stream | 11 | 186.66x | [182.98, 190.04] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | cold | 11 | 220.88x | [206.43, 232.07] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | repeat | 11 | 467.48x | [454.23, 477.08] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | stream | 11 | 226.73x | [206.91, 240.95] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | repeat | 11 | 24.69x | [21.19, 29.46] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | stream | 11 | 21.60x | [18.81, 25.09] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 136.69x | [133.41, 141.00] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | stream | 11 | 116.12x | [109.23, 123.94] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | repeat | 11 | 39.16x | [33.35, 48.35] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | stream | 11 | 18.52x | [17.44, 19.71] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | cold | 11 | 55.20x | [52.80, 57.51] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | repeat | 11 | 54.36x | [47.54, 59.08] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | stream | 11 | 50.58x | [46.92, 53.78] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | cold | 11 | 32.24x | [31.42, 32.93] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | repeat | 11 | 37.28x | [34.54, 39.87] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | stream | 11 | 31.31x | [29.12, 33.22] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 23.30x | [20.86, 26.74] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 29.64x | [26.03, 33.29] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | cold | 11 | 29.32x | [27.06, 30.93] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | repeat | 11 | 45.81x | [42.45, 49.16] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | stream | 11 | 33.94x | [33.01, 35.01] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | cold | 11 | 24.97x | [23.17, 26.68] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | repeat | 11 | 33.15x | [31.16, 34.94] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | stream | 11 | 28.64x | [26.54, 30.74] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | cold | 11 | 22.31x | [21.08, 23.58] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | repeat | 11 | 40.35x | [38.73, 42.02] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | stream | 11 | 22.59x | [21.67, 23.82] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | cold | 11 | 21.70x | [20.17, 23.02] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | repeat | 11 | 40.76x | [37.50, 44.36] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | stream | 11 | 25.67x | [23.56, 27.79] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | cold | 11 | 21.30x | [20.38, 22.11] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | repeat | 11 | 33.03x | [31.31, 34.68] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | stream | 11 | 25.70x | [23.59, 27.87] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | cold | 11 | 45.98x | [44.07, 47.78] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | repeat | 11 | 75.66x | [70.82, 79.93] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | stream | 11 | 51.04x | [47.43, 54.18] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | cold | 11 | 232.60x | [229.19, 235.69] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | repeat | 11 | 462.82x | [448.41, 475.62] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | stream | 11 | 255.06x | [243.27, 266.30] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | cold | 11 | 36.17x | [35.74, 36.66] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | repeat | 11 | 94.98x | [90.03, 99.43] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | stream | 11 | 38.10x | [37.22, 39.00] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | cold | 11 | 46.28x | [44.05, 47.86] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | repeat | 11 | 84.42x | [82.89, 85.75] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | stream | 11 | 50.00x | [45.92, 52.89] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | cold | 11 | 455.00x | [430.72, 470.00] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | repeat | 11 | 762.07x | [735.45, 779.65] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | stream | 11 | 545.95x | [541.72, 550.32] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | cold | 11 | 167.54x | [158.05, 174.41] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | repeat | 11 | 386.36x | [377.68, 394.92] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | stream | 11 | 189.10x | [177.70, 197.60] |
| Node, native | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 29.66x | [28.77, 30.48] |
| Node, native | tiktoken | cl100k_base | book-gosrc | stream | 11 | 14.99x | [14.71, 15.26] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 18.77x | [18.36, 19.12] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 19.03x | [18.06, 19.63] |
| Node, native | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 28.30x | [26.17, 30.38] |
| Node, native | tiktoken | cl100k_base | book-mobydick | stream | 11 | 9.70x | [9.46, 9.95] |
| Node, native | tiktoken | cl100k_base | chat-512B | cold | 11 | 5.67x | [5.33, 6.00] |
| Node, native | tiktoken | cl100k_base | chat-512B | repeat | 11 | 6.90x | [6.32, 7.45] |
| Node, native | tiktoken | cl100k_base | chat-512B | stream | 11 | 5.35x | [4.73, 6.01] |
| Node, native | tiktoken | cl100k_base | td-code-files | cold | 11 | 32.37x | [31.73, 32.88] |
| Node, native | tiktoken | cl100k_base | td-code-files | repeat | 11 | 39.32x | [36.86, 40.90] |
| Node, native | tiktoken | cl100k_base | td-code-files | stream | 11 | 33.25x | [32.28, 34.11] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 54.87x | [43.40, 66.00] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 60.64x | [56.63, 64.34] |
| Node, native | tiktoken | cl100k_base | wiki-ar | cold | 11 | 7.32x | [7.18, 7.45] |
| Node, native | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 14.27x | [13.66, 14.86] |
| Node, native | tiktoken | cl100k_base | wiki-ar | stream | 11 | 10.22x | [9.83, 10.55] |
| Node, native | tiktoken | cl100k_base | wiki-de | cold | 11 | 5.91x | [5.67, 6.10] |
| Node, native | tiktoken | cl100k_base | wiki-de | repeat | 11 | 8.36x | [7.91, 8.78] |
| Node, native | tiktoken | cl100k_base | wiki-de | stream | 11 | 7.19x | [6.78, 7.53] |
| Node, native | tiktoken | cl100k_base | wiki-en | cold | 11 | 12.36x | [12.03, 12.63] |
| Node, native | tiktoken | cl100k_base | wiki-en | repeat | 11 | 20.53x | [19.82, 21.01] |
| Node, native | tiktoken | cl100k_base | wiki-en | stream | 11 | 13.36x | [12.69, 13.97] |
| Node, native | tiktoken | cl100k_base | wiki-es | cold | 11 | 6.68x | [6.41, 6.92] |
| Node, native | tiktoken | cl100k_base | wiki-es | repeat | 11 | 14.36x | [13.50, 15.03] |
| Node, native | tiktoken | cl100k_base | wiki-es | stream | 11 | 9.30x | [8.57, 9.87] |
| Node, native | tiktoken | cl100k_base | wiki-fr | cold | 11 | 6.72x | [6.55, 6.86] |
| Node, native | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 11.99x | [11.37, 12.52] |
| Node, native | tiktoken | cl100k_base | wiki-fr | stream | 11 | 9.27x | [8.73, 9.61] |
| Node, native | tiktoken | cl100k_base | wiki-hi | cold | 11 | 12.63x | [12.38, 12.79] |
| Node, native | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 25.51x | [25.27, 25.73] |
| Node, native | tiktoken | cl100k_base | wiki-hi | stream | 11 | 18.90x | [18.66, 19.11] |
| Node, native | tiktoken | cl100k_base | wiki-ja | cold | 11 | 12.69x | [12.46, 12.89] |
| Node, native | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 22.50x | [21.80, 23.11] |
| Node, native | tiktoken | cl100k_base | wiki-ja | stream | 11 | 13.90x | [13.42, 14.34] |
| Node, native | tiktoken | cl100k_base | wiki-ko | cold | 11 | 6.91x | [6.85, 6.97] |
| Node, native | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 16.69x | [15.82, 17.31] |
| Node, native | tiktoken | cl100k_base | wiki-ko | stream | 11 | 6.84x | [6.11, 7.53] |
| Node, native | tiktoken | cl100k_base | wiki-ru | cold | 11 | 6.03x | [5.86, 6.18] |
| Node, native | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 10.98x | [10.47, 11.40] |
| Node, native | tiktoken | cl100k_base | wiki-ru | stream | 11 | 7.49x | [7.11, 7.74] |
| Node, native | tiktoken | cl100k_base | wiki-th | cold | 11 | 15.63x | [15.38, 15.85] |
| Node, native | tiktoken | cl100k_base | wiki-th | repeat | 11 | 25.80x | [25.24, 26.26] |
| Node, native | tiktoken | cl100k_base | wiki-th | stream | 11 | 18.44x | [18.05, 18.77] |
| Node, native | tiktoken | cl100k_base | wiki-zh | cold | 11 | 12.69x | [11.61, 13.62] |
| Node, native | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 27.46x | [26.86, 27.89] |
| Node, native | tiktoken | cl100k_base | wiki-zh | stream | 11 | 14.17x | [12.85, 15.06] |
| Node, native | tiktoken | o200k_base | book-gosrc | repeat | 11 | 29.92x | [25.46, 33.81] |
| Node, native | tiktoken | o200k_base | book-gosrc | stream | 11 | 19.12x | [18.10, 19.94] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 11.77x | [11.66, 11.88] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 11.05x | [10.22, 11.72] |
| Node, native | tiktoken | o200k_base | book-mobydick | repeat | 11 | 27.89x | [22.71, 32.24] |
| Node, native | tiktoken | o200k_base | book-mobydick | stream | 11 | 10.00x | [9.76, 10.14] |
| Node, native | tiktoken | o200k_base | chat-512B | cold | 11 | 4.92x | [4.64, 5.21] |
| Node, native | tiktoken | o200k_base | chat-512B | repeat | 11 | 4.89x | [4.29, 5.39] |
| Node, native | tiktoken | o200k_base | chat-512B | stream | 11 | 5.07x | [4.70, 5.39] |
| Node, native | tiktoken | o200k_base | td-code-files | cold | 11 | 30.66x | [28.92, 32.04] |
| Node, native | tiktoken | o200k_base | td-code-files | repeat | 11 | 39.25x | [37.52, 40.89] |
| Node, native | tiktoken | o200k_base | td-code-files | stream | 11 | 32.54x | [30.61, 33.75] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 62.54x | [51.79, 72.35] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 64.07x | [60.93, 66.95] |
| Node, native | tiktoken | o200k_base | wiki-ar | cold | 11 | 5.25x | [4.95, 5.53] |
| Node, native | tiktoken | o200k_base | wiki-ar | repeat | 11 | 8.37x | [7.78, 8.82] |
| Node, native | tiktoken | o200k_base | wiki-ar | stream | 11 | 6.15x | [6.05, 6.24] |
| Node, native | tiktoken | o200k_base | wiki-de | cold | 11 | 5.16x | [4.89, 5.44] |
| Node, native | tiktoken | o200k_base | wiki-de | repeat | 11 | 7.23x | [6.86, 7.65] |
| Node, native | tiktoken | o200k_base | wiki-de | stream | 11 | 6.46x | [6.11, 6.80] |
| Node, native | tiktoken | o200k_base | wiki-en | cold | 11 | 10.80x | [10.36, 11.23] |
| Node, native | tiktoken | o200k_base | wiki-en | repeat | 11 | 19.92x | [19.25, 20.59] |
| Node, native | tiktoken | o200k_base | wiki-en | stream | 11 | 11.30x | [10.10, 12.33] |
| Node, native | tiktoken | o200k_base | wiki-es | cold | 11 | 7.13x | [6.68, 7.54] |
| Node, native | tiktoken | o200k_base | wiki-es | repeat | 11 | 13.81x | [12.94, 14.63] |
| Node, native | tiktoken | o200k_base | wiki-es | stream | 11 | 8.60x | [7.96, 9.19] |
| Node, native | tiktoken | o200k_base | wiki-fr | cold | 11 | 7.53x | [7.24, 7.80] |
| Node, native | tiktoken | o200k_base | wiki-fr | repeat | 11 | 11.40x | [10.83, 11.98] |
| Node, native | tiktoken | o200k_base | wiki-fr | stream | 11 | 8.20x | [7.77, 8.71] |
| Node, native | tiktoken | o200k_base | wiki-hi | cold | 11 | 6.36x | [6.14, 6.56] |
| Node, native | tiktoken | o200k_base | wiki-hi | repeat | 11 | 9.99x | [9.44, 10.44] |
| Node, native | tiktoken | o200k_base | wiki-hi | stream | 11 | 7.29x | [6.98, 7.59] |
| Node, native | tiktoken | o200k_base | wiki-ja | cold | 11 | 9.84x | [9.70, 9.98] |
| Node, native | tiktoken | o200k_base | wiki-ja | repeat | 11 | 19.85x | [19.28, 20.35] |
| Node, native | tiktoken | o200k_base | wiki-ja | stream | 11 | 11.05x | [10.50, 11.61] |
| Node, native | tiktoken | o200k_base | wiki-ko | cold | 11 | 6.29x | [6.08, 6.45] |
| Node, native | tiktoken | o200k_base | wiki-ko | repeat | 11 | 15.17x | [14.45, 15.63] |
| Node, native | tiktoken | o200k_base | wiki-ko | stream | 11 | 6.70x | [6.61, 6.79] |
| Node, native | tiktoken | o200k_base | wiki-ru | cold | 11 | 5.48x | [5.41, 5.55] |
| Node, native | tiktoken | o200k_base | wiki-ru | repeat | 11 | 9.85x | [9.70, 9.99] |
| Node, native | tiktoken | o200k_base | wiki-ru | stream | 11 | 6.25x | [6.11, 6.36] |
| Node, native | tiktoken | o200k_base | wiki-th | cold | 11 | 9.77x | [9.66, 9.89] |
| Node, native | tiktoken | o200k_base | wiki-th | repeat | 11 | 15.32x | [14.85, 15.72] |
| Node, native | tiktoken | o200k_base | wiki-th | stream | 11 | 11.33x | [11.22, 11.45] |
| Node, native | tiktoken | o200k_base | wiki-zh | cold | 11 | 9.51x | [8.66, 10.02] |
| Node, native | tiktoken | o200k_base | wiki-zh | repeat | 11 | 21.13x | [20.78, 21.38] |
| Node, native | tiktoken | o200k_base | wiki-zh | stream | 11 | 11.02x | [10.54, 11.41] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 7.00x | [6.75, 7.27] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 6.17x | [5.79, 6.60] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.83x | [3.73, 3.93] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 65.77x | [62.52, 69.29] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 7.62x | [7.12, 8.06] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 6.67x | [6.24, 7.20] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 15.02x | [13.82, 16.22] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 14.92x | [13.31, 16.82] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 10.71x | [9.33, 12.32] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 6.08x | [5.62, 6.82] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 4.82x | [4.57, 5.04] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 5.16x | [5.03, 5.28] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 7.62x | [5.65, 9.81] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 9.41x | [8.44, 10.64] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 20.76x | [20.25, 21.28] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 3.49x | [3.29, 3.72] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 15.53x | [15.12, 16.09] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 8.40x | [8.05, 8.76] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 2.23x | [2.06, 2.48] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 8.24x | [7.85, 8.64] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 6.53x | [6.01, 7.07] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 5.96x | [5.75, 6.19] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 6.91x | [6.68, 7.18] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 7.98x | [7.32, 8.66] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 3.24x | [3.09, 3.35] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 7.73x | [7.40, 8.07] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 8.89x | [8.52, 9.30] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 3.05x | [2.84, 3.22] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 7.81x | [7.44, 8.14] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 13.10x | [11.84, 14.55] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 5.31x | [5.22, 5.40] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 6.77x | [6.59, 6.99] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 45.51x | [44.33, 46.62] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.88x | [3.73, 4.00] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 48.98x | [46.96, 50.74] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 17.62x | [17.06, 18.41] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.45x | [3.28, 3.56] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 14.36x | [13.06, 15.61] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 19.10x | [18.62, 19.58] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 2.42x | [2.35, 2.49] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 15.49x | [14.51, 16.40] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 49.56x | [47.77, 51.54] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 6.19x | [5.95, 6.41] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 41.27x | [39.97, 42.61] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 52.56x | [48.04, 57.05] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 5.25x | [5.11, 5.38] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 53.02x | [48.07, 56.04] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 9.07x | [7.60, 10.48] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 7.63x | [7.35, 7.95] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.71x | [2.67, 2.74] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 47.43x | [43.27, 50.96] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 8.53x | [6.96, 9.85] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 6.77x | [6.59, 6.97] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 12.50x | [11.80, 13.14] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.07x | [0.94, 1.19] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 10.88x | [10.03, 11.67] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 5.74x | [5.17, 6.70] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 4.71x | [4.43, 4.97] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 5.48x | [5.17, 5.85] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 7.56x | [6.45, 8.96] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 8.74x | [8.09, 9.48] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 13.89x | [13.08, 14.60] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.37x | [2.24, 2.49] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 11.02x | [10.79, 11.25] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 8.12x | [7.70, 8.51] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 2.17x | [2.04, 2.32] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 7.79x | [7.26, 8.26] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 6.44x | [6.10, 6.74] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 6.48x | [6.22, 6.74] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 7.68x | [7.25, 8.09] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 7.72x | [7.17, 8.31] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 4.11x | [3.87, 4.33] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 7.58x | [6.93, 8.23] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 8.71x | [8.36, 9.04] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 3.50x | [3.33, 3.66] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 7.19x | [6.67, 7.83] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 14.77x | [14.17, 15.37] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 2.37x | [2.30, 2.45] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 11.57x | [10.98, 12.14] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 39.34x | [37.97, 40.89] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.77x | [3.66, 3.85] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 44.01x | [41.40, 46.38] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 17.18x | [16.68, 17.70] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.82x | [3.59, 3.96] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 15.15x | [14.85, 15.45] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 17.18x | [16.71, 17.61] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.47x | [2.43, 2.50] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 14.63x | [14.16, 15.10] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 31.55x | [31.01, 32.24] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.43x | [1.40, 1.46] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 35.83x | [35.49, 36.13] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 43.24x | [38.48, 46.98] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 4.32x | [4.22, 4.40] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 47.30x | [45.36, 48.65] |
| Go | tiktoken-go | cl100k_base | book-gosrc | stream | 11 | 11.50x | [9.76, 13.07] |
| Go | tiktoken-go | cl100k_base | book-hongloumeng | stream | 11 | 2.99x | [2.63, 3.42] |
| Go | tiktoken-go | cl100k_base | book-mobydick | stream | 11 | 10.09x | [8.77, 11.63] |
| Go | tiktoken-go | cl100k_base | chat-512B | stream | 11 | 4.57x | [4.02, 4.95] |
| Go | tiktoken-go | cl100k_base | td-code-files | stream | 11 | 25.08x | [22.46, 27.73] |
| Go | tiktoken-go | cl100k_base | td-synthetic-1MB * | stream | 11 | 25.78x | [21.65, 29.78] |
| Go | tiktoken-go | cl100k_base | wiki-ar | stream | 11 | 6.25x | [5.87, 6.59] |
| Go | tiktoken-go | cl100k_base | wiki-de | stream | 11 | 6.97x | [6.43, 7.50] |
| Go | tiktoken-go | cl100k_base | wiki-en | stream | 11 | 11.40x | [9.95, 12.83] |
| Go | tiktoken-go | cl100k_base | wiki-es | stream | 11 | 8.52x | [7.61, 9.39] |
| Go | tiktoken-go | cl100k_base | wiki-fr | stream | 11 | 8.62x | [7.95, 9.26] |
| Go | tiktoken-go | cl100k_base | wiki-hi | stream | 11 | 9.48x | [8.83, 10.09] |
| Go | tiktoken-go | cl100k_base | wiki-ja | stream | 11 | 3.27x | [3.12, 3.42] |
| Go | tiktoken-go | cl100k_base | wiki-ko | stream | 11 | 5.21x | [4.44, 5.94] |
| Go | tiktoken-go | cl100k_base | wiki-ru | stream | 11 | 4.34x | [4.07, 4.65] |
| Go | tiktoken-go | cl100k_base | wiki-th | stream | 11 | 4.69x | [4.44, 4.94] |
| Go | tiktoken-go | cl100k_base | wiki-zh | stream | 11 | 3.04x | [2.85, 3.22] |
| Go | tiktoken-go | o200k_base | book-gosrc | stream | 11 | 14.51x | [10.72, 18.81] |
| Go | tiktoken-go | o200k_base | book-hongloumeng | stream | 11 | 3.13x | [2.95, 3.39] |
| Go | tiktoken-go | o200k_base | book-mobydick | stream | 11 | 9.81x | [8.44, 11.22] |
| Go | tiktoken-go | o200k_base | chat-512B | stream | 11 | 4.68x | [4.53, 4.81] |
| Go | tiktoken-go | o200k_base | td-code-files | stream | 11 | 25.69x | [23.94, 27.30] |
| Go | tiktoken-go | o200k_base | td-synthetic-1MB * | stream | 11 | 29.09x | [25.47, 31.42] |
| Go | tiktoken-go | o200k_base | wiki-ar | stream | 11 | 7.79x | [7.63, 7.94] |
| Go | tiktoken-go | o200k_base | wiki-de | stream | 11 | 7.52x | [7.10, 7.85] |
| Go | tiktoken-go | o200k_base | wiki-en | stream | 11 | 12.72x | [11.81, 13.36] |
| Go | tiktoken-go | o200k_base | wiki-es | stream | 11 | 9.08x | [8.05, 9.97] |
| Go | tiktoken-go | o200k_base | wiki-fr | stream | 11 | 10.39x | [9.72, 10.84] |
| Go | tiktoken-go | o200k_base | wiki-hi | stream | 11 | 5.83x | [5.31, 6.54] |
| Go | tiktoken-go | o200k_base | wiki-ja | stream | 11 | 3.61x | [3.45, 3.75] |
| Go | tiktoken-go | o200k_base | wiki-ko | stream | 11 | 5.00x | [4.31, 5.77] |
| Go | tiktoken-go | o200k_base | wiki-ru | stream | 11 | 5.15x | [4.86, 5.37] |
| Go | tiktoken-go | o200k_base | wiki-th | stream | 11 | 3.13x | [3.07, 3.18] |
| Go | tiktoken-go | o200k_base | wiki-zh | stream | 11 | 3.24x | [3.20, 3.28] |
| Go | tokenizer | cl100k_base | book-gosrc | stream | 11 | 8.32x | [6.56, 10.56] |
| Go | tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 2.27x | [1.92, 2.70] |
| Go | tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.01x | [4.28, 5.95] |
| Go | tokenizer | cl100k_base | chat-512B | stream | 11 | 2.90x | [2.61, 3.14] |
| Go | tokenizer | cl100k_base | td-code-files | stream | 11 | 12.43x | [11.54, 13.36] |
| Go | tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 12.36x | [9.70, 15.09] |
| Go | tokenizer | cl100k_base | wiki-ar | stream | 11 | 4.40x | [4.17, 4.63] |
| Go | tokenizer | cl100k_base | wiki-de | stream | 11 | 4.30x | [3.91, 4.66] |
| Go | tokenizer | cl100k_base | wiki-en | stream | 11 | 6.21x | [5.56, 6.84] |
| Go | tokenizer | cl100k_base | wiki-es | stream | 11 | 4.94x | [4.46, 5.42] |
| Go | tokenizer | cl100k_base | wiki-fr | stream | 11 | 5.20x | [4.67, 5.74] |
| Go | tokenizer | cl100k_base | wiki-hi | stream | 11 | 6.63x | [6.11, 7.10] |
| Go | tokenizer | cl100k_base | wiki-ja | stream | 11 | 2.84x | [2.66, 3.02] |
| Go | tokenizer | cl100k_base | wiki-ko | stream | 11 | 3.42x | [3.04, 3.81] |
| Go | tokenizer | cl100k_base | wiki-ru | stream | 11 | 3.10x | [2.90, 3.31] |
| Go | tokenizer | cl100k_base | wiki-th | stream | 11 | 3.70x | [3.50, 3.93] |
| Go | tokenizer | cl100k_base | wiki-zh | stream | 11 | 2.64x | [2.50, 2.74] |
| Go | tokenizer | o200k_base | book-gosrc | stream | 11 | 6.99x | [4.93, 9.94] |
| Go | tokenizer | o200k_base | book-hongloumeng | stream | 11 | 2.12x | [1.96, 2.29] |
| Go | tokenizer | o200k_base | book-mobydick | stream | 11 | 3.88x | [3.42, 4.39] |
| Go | tokenizer | o200k_base | chat-512B | stream | 11 | 2.86x | [2.74, 2.97] |
| Go | tokenizer | o200k_base | td-code-files | stream | 11 | 11.39x | [10.66, 12.19] |
| Go | tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 12.67x | [10.36, 14.72] |
| Go | tokenizer | o200k_base | wiki-ar | stream | 11 | 4.28x | [4.20, 4.36] |
| Go | tokenizer | o200k_base | wiki-de | stream | 11 | 4.40x | [4.03, 4.69] |
| Go | tokenizer | o200k_base | wiki-en | stream | 11 | 6.69x | [6.30, 7.02] |
| Go | tokenizer | o200k_base | wiki-es | stream | 11 | 5.08x | [4.52, 5.64] |
| Go | tokenizer | o200k_base | wiki-fr | stream | 11 | 5.39x | [4.99, 5.74] |
| Go | tokenizer | o200k_base | wiki-hi | stream | 11 | 3.34x | [2.95, 3.84] |
| Go | tokenizer | o200k_base | wiki-ja | stream | 11 | 2.75x | [2.61, 2.88] |
| Go | tokenizer | o200k_base | wiki-ko | stream | 11 | 2.78x | [2.36, 3.28] |
| Go | tokenizer | o200k_base | wiki-ru | stream | 11 | 3.32x | [3.14, 3.51] |
| Go | tokenizer | o200k_base | wiki-th | stream | 11 | 2.65x | [2.58, 2.72] |
| Go | tokenizer | o200k_base | wiki-zh | stream | 11 | 2.41x | [2.38, 2.45] |
