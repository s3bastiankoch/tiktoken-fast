# Benchmark run 2026-10-27-c3d-standard-4-3f3699c

- Machine: AMD EPYC 9B14, c3d-standard-4, kernel 6.1.0-54-cloud-amd64
- Commit 3f3699c01d1ce240a98fa4c5025e533bd22268f2, started 2026-10-27T10:46:27+00:00
- Pinned to core 2; CPU steal during the run: 0.0%
- Toolchains: rustc 1.99.1, python 3.11.2, tiktoken (PyPI) 0.14.0, node 22.23.4, go 1.26.0, tiktoken (npm) 1.0.22, gpt-tokenizer (npm) 4.1.0, js-tiktoken (npm) 1.0.21

Token check: **66860 documents** compared with the reference before timing, **0 with different tokens**.

## Headline

Geometric mean over 16 workloads × 2 encodings, 95% interval in brackets.

| | over | new text | fresh caches | same text | slowest row (new text) | rows slower |
|---|---|---|---|---|---|---:|
| Rust | tiktoken's core (tiktoken-rs 0.12.1) | **4.13x** [4.11, 4.15] | **3.25x** [3.24, 3.26] | **7.00x** [6.95, 7.05] | 2.05x o200k wiki-de | 0 of 32 |
| Rust | bpe-openai 0.3.2 | **2.30x** [2.29, 2.31] | **1.79x** [1.79, 1.80] | **3.88x** [3.86, 3.91] | 1.37x o200k wiki-ko | 0 of 32 |
| Python | tiktoken 0.14.0 | **3.78x** [3.76, 3.80] | **3.12x** [3.10, 3.13] |  | 2.10x o200k wiki-de | 0 of 32 |
| Python, encode() | tiktoken 0.14.0 | **3.68x** [3.66, 3.71] | **3.01x** [3.00, 3.03] |  | 2.12x o200k wiki-de | 0 of 31 |
| JavaScript | tiktoken 1.0.22 (npm) | **9.71x** [9.63, 9.77] | **8.08x** [8.05, 8.11] | **14.68x** [14.57, 14.79] | 5.95x o200k wiki-ru | 0 of 32 |
| JavaScript | gpt-tokenizer 4.1.0 | **10.56x** [10.46, 10.64] | **11.77x** [11.68, 11.88] | **3.37x** [3.34, 3.40] | 3.28x cl100k td-code-files | 0 of 32 |
| JavaScript | js-tiktoken 1.0.21 | **56.62x** [56.15, 57.06] | **53.33x** [53.12, 53.54] | **85.49x** [84.74, 86.30] | 17.72x cl100k book-mobydick | 0 of 32 |
| Node, native | tiktoken 1.0.22 (npm) | **11.46x** [11.42, 11.50] | **8.85x** [8.82, 8.87] | **17.67x** [17.54, 17.79] | 6.86x o200k wiki-ko | 0 of 32 |
| Node, native | gpt-tokenizer 4.1.0 | **12.46x** [12.39, 12.53] | **12.89x** [12.80, 12.99] | **4.05x** [4.02, 4.09] | 5.22x cl100k td-code-files | 0 of 32 |
| Go | pkoukk/tiktoken-go (200cf60, vendored) | **6.80x** [6.67, 6.93] |  |  | 2.94x cl100k book-hongloumeng | 0 of 32 |
| Go | tiktoken-go/tokenizer 0.8.1 | **4.29x** [4.20, 4.37] |  |  | 2.27x o200k book-hongloumeng | 0 of 32 |

Documents where another library's tokens differ from its reference (timed anyway): go/tokenizer 599, js/gpt-tokenizer 0, js/js-tiktoken 0, rust/bpe-openai 0.

## Every row

Rows marked * are outside the headline.

| | over | encoding | workload | scenario | rounds | speed-up | 95% interval |
|---|---|---|---|---|---:|---:|---|
| Rust | tiktoken-rs | cl100k_base | book-gosrc | repeat | 11 | 17.05x | [16.02, 18.22] |
| Rust | tiktoken-rs | cl100k_base | book-gosrc | stream | 11 | 5.67x | [5.38, 5.91] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | repeat | 11 | 9.99x | [9.71, 10.28] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | stream | 11 | 9.94x | [9.62, 10.23] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | repeat | 11 | 14.90x | [13.09, 16.77] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | stream | 11 | 3.73x | [3.66, 3.80] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | cold | 11 | 2.87x | [2.86, 2.87] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | repeat | 11 | 3.62x | [3.61, 3.63] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | stream | 11 | 3.20x | [3.07, 3.28] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | cold | 11 | 10.69x | [10.53, 10.90] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | repeat | 11 | 15.19x | [14.63, 15.84] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | stream | 11 | 11.77x | [11.47, 12.11] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | repeat | 11 | 21.63x | [20.91, 22.29] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | stream | 11 | 20.04x | [19.80, 20.30] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | cold | 11 | 3.10x | [3.08, 3.12] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | repeat | 11 | 7.16x | [7.05, 7.29] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | stream | 11 | 4.64x | [4.58, 4.70] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | cold | 11 | 2.11x | [2.10, 2.12] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | repeat | 11 | 3.34x | [3.33, 3.36] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | stream | 11 | 2.76x | [2.75, 2.78] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | cold | 11 | 3.90x | [3.84, 3.97] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | repeat | 11 | 8.02x | [7.79, 8.32] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | stream | 11 | 4.78x | [4.69, 4.88] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | cold | 11 | 2.34x | [2.33, 2.36] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | repeat | 11 | 5.85x | [5.71, 6.00] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | stream | 11 | 3.54x | [3.50, 3.58] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | cold | 11 | 2.34x | [2.33, 2.36] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | repeat | 11 | 4.77x | [4.71, 4.82] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | stream | 11 | 3.52x | [3.48, 3.56] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | cold | 11 | 5.19x | [5.17, 5.23] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | repeat | 11 | 12.20x | [12.09, 12.34] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | stream | 11 | 8.05x | [7.97, 8.14] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | cold | 11 | 6.15x | [6.08, 6.22] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | repeat | 11 | 13.01x | [12.60, 13.40] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | stream | 11 | 7.02x | [6.94, 7.10] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | cold | 11 | 3.18x | [3.09, 3.27] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | repeat | 11 | 10.55x | [9.80, 11.37] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | stream | 11 | 3.82x | [3.70, 3.95] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | cold | 11 | 2.85x | [2.83, 2.86] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | repeat | 11 | 5.75x | [5.68, 5.82] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | stream | 11 | 3.53x | [3.50, 3.56] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | cold | 11 | 7.87x | [7.79, 7.96] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | repeat | 11 | 14.64x | [14.44, 14.87] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | stream | 11 | 9.19x | [9.07, 9.33] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | cold | 11 | 7.16x | [7.10, 7.24] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | repeat | 11 | 16.60x | [16.24, 16.98] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | stream | 11 | 7.63x | [7.38, 7.85] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | repeat | 11 | 11.04x | [10.33, 11.75] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | stream | 11 | 4.92x | [4.63, 5.15] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | repeat | 11 | 5.10x | [4.98, 5.22] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | stream | 11 | 5.15x | [4.94, 5.34] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | repeat | 11 | 10.46x | [9.71, 11.23] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | stream | 11 | 2.52x | [2.38, 2.65] |
| Rust | tiktoken-rs | o200k_base | chat-512B | cold | 11 | 2.19x | [2.18, 2.20] |
| Rust | tiktoken-rs | o200k_base | chat-512B | repeat | 11 | 2.53x | [2.51, 2.54] |
| Rust | tiktoken-rs | o200k_base | chat-512B | stream | 11 | 2.32x | [2.31, 2.32] |
| Rust | tiktoken-rs | o200k_base | td-code-files | cold | 11 | 7.16x | [7.07, 7.26] |
| Rust | tiktoken-rs | o200k_base | td-code-files | repeat | 11 | 10.35x | [10.09, 10.63] |
| Rust | tiktoken-rs | o200k_base | td-code-files | stream | 11 | 7.71x | [7.54, 7.90] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | repeat | 11 | 12.80x | [12.15, 13.45] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | stream | 11 | 11.61x | [11.50, 11.72] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | cold | 11 | 1.98x | [1.96, 2.00] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | repeat | 11 | 3.43x | [3.38, 3.48] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | stream | 11 | 2.28x | [2.25, 2.31] |
| Rust | tiktoken-rs | o200k_base | wiki-de | cold | 11 | 1.69x | [1.68, 1.71] |
| Rust | tiktoken-rs | o200k_base | wiki-de | repeat | 11 | 2.48x | [2.47, 2.50] |
| Rust | tiktoken-rs | o200k_base | wiki-de | stream | 11 | 2.05x | [2.04, 2.07] |
| Rust | tiktoken-rs | o200k_base | wiki-en | cold | 11 | 2.59x | [2.53, 2.64] |
| Rust | tiktoken-rs | o200k_base | wiki-en | repeat | 11 | 5.50x | [5.35, 5.66] |
| Rust | tiktoken-rs | o200k_base | wiki-en | stream | 11 | 3.14x | [3.07, 3.20] |
| Rust | tiktoken-rs | o200k_base | wiki-es | cold | 11 | 1.90x | [1.88, 1.92] |
| Rust | tiktoken-rs | o200k_base | wiki-es | repeat | 11 | 4.07x | [4.00, 4.16] |
| Rust | tiktoken-rs | o200k_base | wiki-es | stream | 11 | 2.44x | [2.42, 2.47] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | cold | 11 | 1.83x | [1.80, 1.85] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | repeat | 11 | 3.21x | [3.18, 3.24] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | stream | 11 | 2.37x | [2.34, 2.40] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | cold | 11 | 2.06x | [2.02, 2.10] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | repeat | 11 | 3.94x | [3.84, 4.05] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | stream | 11 | 2.60x | [2.55, 2.65] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | cold | 11 | 4.32x | [4.28, 4.37] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | repeat | 11 | 9.52x | [9.35, 9.68] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | stream | 11 | 5.10x | [5.05, 5.16] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | cold | 11 | 2.32x | [2.22, 2.41] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | repeat | 11 | 7.11x | [6.49, 7.82] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | stream | 11 | 2.50x | [2.36, 2.63] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | cold | 11 | 2.14x | [2.12, 2.16] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | repeat | 11 | 4.11x | [4.08, 4.15] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | stream | 11 | 2.45x | [2.44, 2.47] |
| Rust | tiktoken-rs | o200k_base | wiki-th | cold | 11 | 3.21x | [3.18, 3.26] |
| Rust | tiktoken-rs | o200k_base | wiki-th | repeat | 11 | 5.79x | [5.71, 5.89] |
| Rust | tiktoken-rs | o200k_base | wiki-th | stream | 11 | 3.96x | [3.91, 4.00] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | cold | 11 | 4.33x | [4.26, 4.42] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | repeat | 11 | 10.34x | [10.08, 10.64] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | stream | 11 | 4.95x | [4.84, 5.07] |
| Rust | bpe-openai | cl100k_base | book-gosrc | repeat | 11 | 6.88x | [6.56, 7.23] |
| Rust | bpe-openai | cl100k_base | book-gosrc | stream | 11 | 2.29x | [2.19, 2.38] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | repeat | 11 | 5.18x | [5.03, 5.34] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | stream | 11 | 5.16x | [5.01, 5.31] |
| Rust | bpe-openai | cl100k_base | book-mobydick | repeat | 11 | 8.67x | [7.60, 9.85] |
| Rust | bpe-openai | cl100k_base | book-mobydick | stream | 11 | 2.19x | [2.16, 2.24] |
| Rust | bpe-openai | cl100k_base | chat-512B | cold | 11 | 1.56x | [1.56, 1.57] |
| Rust | bpe-openai | cl100k_base | chat-512B | repeat | 11 | 1.98x | [1.97, 1.98] |
| Rust | bpe-openai | cl100k_base | chat-512B | stream | 11 | 1.75x | [1.68, 1.79] |
| Rust | bpe-openai | cl100k_base | td-code-files | cold | 11 | 3.83x | [3.80, 3.86] |
| Rust | bpe-openai | cl100k_base | td-code-files | repeat | 11 | 5.42x | [5.29, 5.57] |
| Rust | bpe-openai | cl100k_base | td-code-files | stream | 11 | 4.15x | [4.11, 4.21] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | repeat | 11 | 10.59x | [10.22, 10.94] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | stream | 11 | 9.80x | [9.70, 9.91] |
| Rust | bpe-openai | cl100k_base | wiki-ar | cold | 11 | 1.37x | [1.37, 1.38] |
| Rust | bpe-openai | cl100k_base | wiki-ar | repeat | 11 | 3.17x | [3.13, 3.21] |
| Rust | bpe-openai | cl100k_base | wiki-ar | stream | 11 | 2.07x | [2.04, 2.10] |
| Rust | bpe-openai | cl100k_base | wiki-de | cold | 11 | 1.38x | [1.37, 1.38] |
| Rust | bpe-openai | cl100k_base | wiki-de | repeat | 11 | 2.18x | [2.16, 2.20] |
| Rust | bpe-openai | cl100k_base | wiki-de | stream | 11 | 1.81x | [1.80, 1.82] |
| Rust | bpe-openai | cl100k_base | wiki-en | cold | 11 | 2.14x | [2.13, 2.16] |
| Rust | bpe-openai | cl100k_base | wiki-en | repeat | 11 | 4.38x | [4.31, 4.46] |
| Rust | bpe-openai | cl100k_base | wiki-en | stream | 11 | 2.62x | [2.59, 2.64] |
| Rust | bpe-openai | cl100k_base | wiki-es | cold | 11 | 1.39x | [1.38, 1.39] |
| Rust | bpe-openai | cl100k_base | wiki-es | repeat | 11 | 3.44x | [3.40, 3.48] |
| Rust | bpe-openai | cl100k_base | wiki-es | stream | 11 | 2.09x | [2.07, 2.11] |
| Rust | bpe-openai | cl100k_base | wiki-fr | cold | 11 | 1.42x | [1.41, 1.43] |
| Rust | bpe-openai | cl100k_base | wiki-fr | repeat | 11 | 2.88x | [2.85, 2.91] |
| Rust | bpe-openai | cl100k_base | wiki-fr | stream | 11 | 2.14x | [2.12, 2.16] |
| Rust | bpe-openai | cl100k_base | wiki-hi | cold | 11 | 1.99x | [1.99, 2.00] |
| Rust | bpe-openai | cl100k_base | wiki-hi | repeat | 11 | 4.67x | [4.64, 4.71] |
| Rust | bpe-openai | cl100k_base | wiki-hi | stream | 11 | 3.09x | [3.07, 3.10] |
| Rust | bpe-openai | cl100k_base | wiki-ja | cold | 11 | 2.79x | [2.77, 2.80] |
| Rust | bpe-openai | cl100k_base | wiki-ja | repeat | 11 | 5.90x | [5.73, 6.04] |
| Rust | bpe-openai | cl100k_base | wiki-ja | stream | 11 | 3.19x | [3.17, 3.21] |
| Rust | bpe-openai | cl100k_base | wiki-ko | cold | 11 | 1.46x | [1.43, 1.50] |
| Rust | bpe-openai | cl100k_base | wiki-ko | repeat | 11 | 4.81x | [4.48, 5.18] |
| Rust | bpe-openai | cl100k_base | wiki-ko | stream | 11 | 1.75x | [1.70, 1.80] |
| Rust | bpe-openai | cl100k_base | wiki-ru | cold | 11 | 1.26x | [1.25, 1.26] |
| Rust | bpe-openai | cl100k_base | wiki-ru | repeat | 11 | 2.54x | [2.51, 2.56] |
| Rust | bpe-openai | cl100k_base | wiki-ru | stream | 11 | 1.56x | [1.55, 1.57] |
| Rust | bpe-openai | cl100k_base | wiki-th | cold | 11 | 3.14x | [3.12, 3.16] |
| Rust | bpe-openai | cl100k_base | wiki-th | repeat | 11 | 5.86x | [5.82, 5.90] |
| Rust | bpe-openai | cl100k_base | wiki-th | stream | 11 | 3.71x | [3.68, 3.74] |
| Rust | bpe-openai | cl100k_base | wiki-zh | cold | 11 | 3.61x | [3.59, 3.63] |
| Rust | bpe-openai | cl100k_base | wiki-zh | repeat | 11 | 8.39x | [8.17, 8.58] |
| Rust | bpe-openai | cl100k_base | wiki-zh | stream | 11 | 3.84x | [3.72, 3.92] |
| Rust | bpe-openai | o200k_base | book-gosrc | repeat | 11 | 6.91x | [6.47, 7.33] |
| Rust | bpe-openai | o200k_base | book-gosrc | stream | 11 | 3.11x | [2.95, 3.25] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | repeat | 11 | 2.92x | [2.82, 3.01] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | stream | 11 | 2.94x | [2.85, 3.03] |
| Rust | bpe-openai | o200k_base | book-mobydick | repeat | 11 | 9.05x | [8.35, 9.78] |
| Rust | bpe-openai | o200k_base | book-mobydick | stream | 11 | 2.21x | [2.10, 2.33] |
| Rust | bpe-openai | o200k_base | chat-512B | cold | 11 | 1.50x | [1.49, 1.51] |
| Rust | bpe-openai | o200k_base | chat-512B | repeat | 11 | 1.74x | [1.73, 1.75] |
| Rust | bpe-openai | o200k_base | chat-512B | stream | 11 | 1.60x | [1.59, 1.60] |
| Rust | bpe-openai | o200k_base | td-code-files | cold | 11 | 3.99x | [3.96, 4.01] |
| Rust | bpe-openai | o200k_base | td-code-files | repeat | 11 | 5.77x | [5.67, 5.87] |
| Rust | bpe-openai | o200k_base | td-code-files | stream | 11 | 4.36x | [4.26, 4.46] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | repeat | 11 | 10.63x | [10.14, 11.11] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | stream | 11 | 9.72x | [9.63, 9.81] |
| Rust | bpe-openai | o200k_base | wiki-ar | cold | 11 | 1.29x | [1.28, 1.30] |
| Rust | bpe-openai | o200k_base | wiki-ar | repeat | 11 | 2.22x | [2.20, 2.25] |
| Rust | bpe-openai | o200k_base | wiki-ar | stream | 11 | 1.48x | [1.46, 1.49] |
| Rust | bpe-openai | o200k_base | wiki-de | cold | 11 | 1.39x | [1.37, 1.40] |
| Rust | bpe-openai | o200k_base | wiki-de | repeat | 11 | 2.02x | [2.01, 2.04] |
| Rust | bpe-openai | o200k_base | wiki-de | stream | 11 | 1.68x | [1.67, 1.69] |
| Rust | bpe-openai | o200k_base | wiki-en | cold | 11 | 2.16x | [2.12, 2.20] |
| Rust | bpe-openai | o200k_base | wiki-en | repeat | 11 | 4.56x | [4.48, 4.66] |
| Rust | bpe-openai | o200k_base | wiki-en | stream | 11 | 2.64x | [2.58, 2.71] |
| Rust | bpe-openai | o200k_base | wiki-es | cold | 11 | 1.60x | [1.58, 1.62] |
| Rust | bpe-openai | o200k_base | wiki-es | repeat | 11 | 3.38x | [3.34, 3.42] |
| Rust | bpe-openai | o200k_base | wiki-es | stream | 11 | 2.03x | [2.00, 2.07] |
| Rust | bpe-openai | o200k_base | wiki-fr | cold | 11 | 1.56x | [1.54, 1.58] |
| Rust | bpe-openai | o200k_base | wiki-fr | repeat | 11 | 2.74x | [2.71, 2.77] |
| Rust | bpe-openai | o200k_base | wiki-fr | stream | 11 | 2.03x | [2.00, 2.05] |
| Rust | bpe-openai | o200k_base | wiki-hi | cold | 11 | 1.23x | [1.21, 1.25] |
| Rust | bpe-openai | o200k_base | wiki-hi | repeat | 11 | 2.33x | [2.27, 2.39] |
| Rust | bpe-openai | o200k_base | wiki-hi | stream | 11 | 1.52x | [1.50, 1.54] |
| Rust | bpe-openai | o200k_base | wiki-ja | cold | 11 | 2.13x | [2.12, 2.14] |
| Rust | bpe-openai | o200k_base | wiki-ja | repeat | 11 | 4.73x | [4.63, 4.80] |
| Rust | bpe-openai | o200k_base | wiki-ja | stream | 11 | 2.54x | [2.52, 2.57] |
| Rust | bpe-openai | o200k_base | wiki-ko | cold | 11 | 1.28x | [1.23, 1.32] |
| Rust | bpe-openai | o200k_base | wiki-ko | repeat | 11 | 3.90x | [3.55, 4.28] |
| Rust | bpe-openai | o200k_base | wiki-ko | stream | 11 | 1.37x | [1.30, 1.44] |
| Rust | bpe-openai | o200k_base | wiki-ru | cold | 11 | 1.32x | [1.31, 1.33] |
| Rust | bpe-openai | o200k_base | wiki-ru | repeat | 11 | 2.52x | [2.51, 2.53] |
| Rust | bpe-openai | o200k_base | wiki-ru | stream | 11 | 1.51x | [1.50, 1.53] |
| Rust | bpe-openai | o200k_base | wiki-th | cold | 11 | 1.13x | [1.13, 1.14] |
| Rust | bpe-openai | o200k_base | wiki-th | repeat | 11 | 2.05x | [2.03, 2.07] |
| Rust | bpe-openai | o200k_base | wiki-th | stream | 11 | 1.38x | [1.37, 1.39] |
| Rust | bpe-openai | o200k_base | wiki-zh | cold | 11 | 2.42x | [2.38, 2.45] |
| Rust | bpe-openai | o200k_base | wiki-zh | repeat | 11 | 5.73x | [5.59, 5.86] |
| Rust | bpe-openai | o200k_base | wiki-zh | stream | 11 | 2.74x | [2.70, 2.78] |
| Python | tiktoken | cl100k_base | book-gosrc | stream | 11 | 5.39x | [5.18, 5.59] |
| Python | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 5.50x | [5.39, 5.61] |
| Python | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.79x | [3.62, 3.94] |
| Python | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.77x | [2.75, 2.80] |
| Python | tiktoken | cl100k_base | chat-512B | stream | 11 | 3.11x | [3.08, 3.14] |
| Python | tiktoken | cl100k_base | td-code-files | cold | 11 | 8.84x | [8.65, 9.03] |
| Python | tiktoken | cl100k_base | td-code-files | stream | 11 | 9.21x | [8.94, 9.47] |
| Python | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 12.04x | [11.93, 12.15] |
| Python | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.99x | [2.95, 3.02] |
| Python | tiktoken | cl100k_base | wiki-ar | stream | 11 | 4.07x | [4.01, 4.12] |
| Python | tiktoken | cl100k_base | wiki-de | cold | 11 | 2.25x | [2.23, 2.27] |
| Python | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.86x | [2.82, 2.91] |
| Python | tiktoken | cl100k_base | wiki-en | cold | 11 | 4.06x | [3.96, 4.17] |
| Python | tiktoken | cl100k_base | wiki-en | stream | 11 | 4.87x | [4.73, 5.03] |
| Python | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.55x | [2.51, 2.60] |
| Python | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.73x | [3.64, 3.82] |
| Python | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.48x | [2.45, 2.52] |
| Python | tiktoken | cl100k_base | wiki-fr | stream | 11 | 3.69x | [3.61, 3.77] |
| Python | tiktoken | cl100k_base | wiki-hi | cold | 11 | 4.54x | [4.50, 4.57] |
| Python | tiktoken | cl100k_base | wiki-hi | stream | 11 | 6.10x | [6.05, 6.14] |
| Python | tiktoken | cl100k_base | wiki-ja | cold | 11 | 4.90x | [4.80, 4.99] |
| Python | tiktoken | cl100k_base | wiki-ja | stream | 11 | 5.32x | [5.19, 5.47] |
| Python | tiktoken | cl100k_base | wiki-ko | cold | 11 | 3.00x | [2.90, 3.10] |
| Python | tiktoken | cl100k_base | wiki-ko | stream | 11 | 3.43x | [3.29, 3.56] |
| Python | tiktoken | cl100k_base | wiki-ru | cold | 11 | 2.94x | [2.90, 2.97] |
| Python | tiktoken | cl100k_base | wiki-ru | stream | 11 | 3.47x | [3.42, 3.52] |
| Python | tiktoken | cl100k_base | wiki-th | cold | 11 | 5.99x | [5.91, 6.07] |
| Python | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.66x | [6.57, 6.77] |
| Python | tiktoken | cl100k_base | wiki-zh | cold | 11 | 4.97x | [4.84, 5.10] |
| Python | tiktoken | cl100k_base | wiki-zh | stream | 11 | 5.12x | [4.99, 5.25] |
| Python | tiktoken | o200k_base | book-gosrc | stream | 11 | 4.31x | [4.21, 4.41] |
| Python | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 4.87x | [4.72, 5.02] |
| Python | tiktoken | o200k_base | book-mobydick | stream | 11 | 2.53x | [2.44, 2.62] |
| Python | tiktoken | o200k_base | chat-512B | cold | 11 | 2.19x | [2.16, 2.23] |
| Python | tiktoken | o200k_base | chat-512B | stream | 11 | 2.29x | [2.25, 2.34] |
| Python | tiktoken | o200k_base | td-code-files | cold | 11 | 5.71x | [5.57, 5.86] |
| Python | tiktoken | o200k_base | td-code-files | stream | 11 | 5.99x | [5.84, 6.15] |
| Python | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 6.98x | [6.93, 7.03] |
| Python | tiktoken | o200k_base | wiki-ar | cold | 11 | 2.00x | [1.94, 2.06] |
| Python | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.26x | [2.18, 2.33] |
| Python | tiktoken | o200k_base | wiki-de | cold | 11 | 1.76x | [1.72, 1.79] |
| Python | tiktoken | o200k_base | wiki-de | stream | 11 | 2.10x | [2.05, 2.15] |
| Python | tiktoken | o200k_base | wiki-en | cold | 11 | 2.62x | [2.53, 2.71] |
| Python | tiktoken | o200k_base | wiki-en | stream | 11 | 3.09x | [2.96, 3.22] |
| Python | tiktoken | o200k_base | wiki-es | cold | 11 | 1.99x | [1.92, 2.05] |
| Python | tiktoken | o200k_base | wiki-es | stream | 11 | 2.49x | [2.39, 2.59] |
| Python | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.95x | [1.90, 2.00] |
| Python | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.50x | [2.42, 2.59] |
| Python | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.12x | [2.06, 2.18] |
| Python | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.59x | [2.51, 2.68] |
| Python | tiktoken | o200k_base | wiki-ja | cold | 11 | 4.01x | [3.90, 4.12] |
| Python | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.65x | [4.50, 4.80] |
| Python | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.44x | [2.31, 2.58] |
| Python | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.61x | [2.45, 2.77] |
| Python | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.29x | [2.23, 2.35] |
| Python | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.59x | [2.52, 2.66] |
| Python | tiktoken | o200k_base | wiki-th | cold | 11 | 3.19x | [3.14, 3.23] |
| Python | tiktoken | o200k_base | wiki-th | stream | 11 | 3.85x | [3.78, 3.92] |
| Python | tiktoken | o200k_base | wiki-zh | cold | 11 | 4.11x | [3.96, 4.26] |
| Python | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.49x | [4.31, 4.67] |
| Python, encode() | tiktoken | cl100k_base | book-gosrc | stream | 11 | 5.45x | [5.24, 5.68] |
| Python, encode() | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 5.52x | [5.39, 5.66] |
| Python, encode() | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.83x | [3.68, 3.98] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.87x | [2.85, 2.89] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | stream | 11 | 3.19x | [3.16, 3.22] |
| Python, encode() | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 12.44x | [12.33, 12.56] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.94x | [2.91, 2.98] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | stream | 11 | 4.11x | [4.05, 4.17] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | cold | 11 | 2.23x | [2.21, 2.26] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.85x | [2.81, 2.89] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | cold | 11 | 4.14x | [4.04, 4.24] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | stream | 11 | 4.95x | [4.81, 5.09] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.60x | [2.55, 2.64] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.77x | [3.67, 3.87] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.59x | [2.55, 2.63] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | stream | 11 | 3.74x | [3.66, 3.83] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | cold | 11 | 4.57x | [4.53, 4.60] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | stream | 11 | 6.10x | [6.02, 6.17] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | cold | 11 | 4.90x | [4.81, 5.00] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | stream | 11 | 5.34x | [5.21, 5.48] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | cold | 11 | 3.03x | [2.94, 3.13] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | stream | 11 | 3.43x | [3.30, 3.56] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | cold | 11 | 2.96x | [2.93, 2.99] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | stream | 11 | 3.27x | [3.23, 3.32] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | cold | 11 | 6.00x | [5.91, 6.08] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.63x | [6.47, 6.78] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | cold | 11 | 4.95x | [4.83, 5.07] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | stream | 11 | 5.13x | [4.99, 5.27] |
| Python, encode() | tiktoken | o200k_base | book-gosrc | stream | 11 | 4.29x | [4.18, 4.40] |
| Python, encode() | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 4.84x | [4.68, 5.01] |
| Python, encode() | tiktoken | o200k_base | book-mobydick | stream | 11 | 2.54x | [2.44, 2.64] |
| Python, encode() | tiktoken | o200k_base | chat-512B | cold | 11 | 2.27x | [2.23, 2.30] |
| Python, encode() | tiktoken | o200k_base | chat-512B | stream | 11 | 2.34x | [2.30, 2.39] |
| Python, encode() | tiktoken | o200k_base | td-code-files | cold | 11 | 5.55x | [5.42, 5.68] |
| Python, encode() | tiktoken | o200k_base | td-code-files | stream | 11 | 5.88x | [5.74, 6.03] |
| Python, encode() | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 6.98x | [6.93, 7.04] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | cold | 11 | 2.02x | [1.96, 2.08] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.27x | [2.20, 2.35] |
| Python, encode() | tiktoken | o200k_base | wiki-de | cold | 11 | 1.78x | [1.74, 1.81] |
| Python, encode() | tiktoken | o200k_base | wiki-de | stream | 11 | 2.12x | [2.07, 2.16] |
| Python, encode() | tiktoken | o200k_base | wiki-en | cold | 11 | 2.64x | [2.53, 2.75] |
| Python, encode() | tiktoken | o200k_base | wiki-en | stream | 11 | 3.11x | [2.97, 3.25] |
| Python, encode() | tiktoken | o200k_base | wiki-es | cold | 11 | 2.00x | [1.94, 2.07] |
| Python, encode() | tiktoken | o200k_base | wiki-es | stream | 11 | 2.51x | [2.41, 2.62] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.97x | [1.91, 2.03] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.50x | [2.42, 2.59] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.15x | [2.08, 2.21] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.63x | [2.54, 2.72] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | cold | 11 | 4.01x | [3.90, 4.12] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.66x | [4.51, 4.81] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.46x | [2.31, 2.60] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.60x | [2.43, 2.77] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.30x | [2.25, 2.36] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.59x | [2.52, 2.67] |
| Python, encode() | tiktoken | o200k_base | wiki-th | cold | 11 | 3.20x | [3.15, 3.25] |
| Python, encode() | tiktoken | o200k_base | wiki-th | stream | 11 | 3.85x | [3.78, 3.93] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | cold | 11 | 4.11x | [3.96, 4.27] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.48x | [4.30, 4.68] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 24.99x | [24.36, 25.82] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | stream | 11 | 11.50x | [10.10, 12.52] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 14.94x | [14.69, 15.17] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 12.79x | [11.16, 14.20] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 21.36x | [20.41, 22.58] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | stream | 11 | 9.36x | [9.14, 9.59] |
| JavaScript | tiktoken | cl100k_base | chat-512B | cold | 11 | 6.18x | [5.91, 6.36] |
| JavaScript | tiktoken | cl100k_base | chat-512B | repeat | 11 | 7.39x | [7.33, 7.45] |
| JavaScript | tiktoken | cl100k_base | chat-512B | stream | 11 | 6.72x | [6.36, 6.99] |
| JavaScript | tiktoken | cl100k_base | td-code-files | cold | 11 | 21.86x | [20.38, 23.14] |
| JavaScript | tiktoken | cl100k_base | td-code-files | repeat | 11 | 25.86x | [24.21, 27.65] |
| JavaScript | tiktoken | cl100k_base | td-code-files | stream | 11 | 20.86x | [19.37, 22.53] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 21.19x | [18.65, 24.53] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 27.81x | [25.45, 30.17] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | cold | 11 | 6.11x | [6.08, 6.13] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 12.75x | [12.62, 12.87] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | stream | 11 | 8.68x | [8.59, 8.77] |
| JavaScript | tiktoken | cl100k_base | wiki-de | cold | 11 | 5.69x | [5.65, 5.72] |
| JavaScript | tiktoken | cl100k_base | wiki-de | repeat | 11 | 8.77x | [8.70, 8.84] |
| JavaScript | tiktoken | cl100k_base | wiki-de | stream | 11 | 7.26x | [7.18, 7.34] |
| JavaScript | tiktoken | cl100k_base | wiki-en | cold | 11 | 11.04x | [10.89, 11.17] |
| JavaScript | tiktoken | cl100k_base | wiki-en | repeat | 11 | 19.51x | [19.13, 19.79] |
| JavaScript | tiktoken | cl100k_base | wiki-en | stream | 11 | 12.72x | [12.50, 12.93] |
| JavaScript | tiktoken | cl100k_base | wiki-es | cold | 11 | 6.47x | [6.42, 6.51] |
| JavaScript | tiktoken | cl100k_base | wiki-es | repeat | 11 | 13.83x | [13.62, 14.06] |
| JavaScript | tiktoken | cl100k_base | wiki-es | stream | 11 | 9.21x | [9.07, 9.36] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | cold | 11 | 6.41x | [6.38, 6.45] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 12.05x | [11.98, 12.12] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | stream | 11 | 8.98x | [8.87, 9.09] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | cold | 11 | 9.65x | [9.60, 9.71] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 20.36x | [20.23, 20.48] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | stream | 11 | 14.35x | [14.27, 14.44] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | cold | 11 | 11.47x | [11.39, 11.55] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 21.03x | [20.44, 21.42] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | stream | 11 | 12.74x | [12.60, 12.87] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | cold | 11 | 5.96x | [5.89, 6.02] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 15.69x | [14.98, 16.14] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | stream | 11 | 6.70x | [6.58, 6.82] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | cold | 11 | 5.83x | [5.78, 5.88] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 10.56x | [10.48, 10.65] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | stream | 11 | 6.98x | [6.93, 7.03] |
| JavaScript | tiktoken | cl100k_base | wiki-th | cold | 11 | 12.98x | [12.84, 13.12] |
| JavaScript | tiktoken | cl100k_base | wiki-th | repeat | 11 | 21.70x | [21.62, 21.79] |
| JavaScript | tiktoken | cl100k_base | wiki-th | stream | 11 | 14.87x | [14.70, 15.03] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | cold | 11 | 12.16x | [12.05, 12.29] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 24.06x | [23.61, 24.37] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | stream | 11 | 12.45x | [12.31, 12.61] |
| JavaScript | tiktoken | o200k_base | book-gosrc | repeat | 11 | 20.86x | [18.00, 24.27] |
| JavaScript | tiktoken | o200k_base | book-gosrc | stream | 11 | 16.36x | [15.25, 17.31] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 9.68x | [9.53, 9.85] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 8.72x | [8.35, 9.12] |
| JavaScript | tiktoken | o200k_base | book-mobydick | repeat | 11 | 18.78x | [16.87, 21.29] |
| JavaScript | tiktoken | o200k_base | book-mobydick | stream | 11 | 8.80x | [8.36, 9.19] |
| JavaScript | tiktoken | o200k_base | chat-512B | cold | 11 | 5.93x | [5.83, 6.01] |
| JavaScript | tiktoken | o200k_base | chat-512B | repeat | 11 | 6.62x | [6.57, 6.67] |
| JavaScript | tiktoken | o200k_base | chat-512B | stream | 11 | 6.23x | [6.19, 6.26] |
| JavaScript | tiktoken | o200k_base | td-code-files | cold | 11 | 23.88x | [23.09, 24.48] |
| JavaScript | tiktoken | o200k_base | td-code-files | repeat | 11 | 27.23x | [25.39, 29.07] |
| JavaScript | tiktoken | o200k_base | td-code-files | stream | 11 | 21.99x | [20.46, 23.64] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 23.80x | [21.42, 26.80] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 31.01x | [28.23, 33.36] |
| JavaScript | tiktoken | o200k_base | wiki-ar | cold | 11 | 5.57x | [5.53, 5.62] |
| JavaScript | tiktoken | o200k_base | wiki-ar | repeat | 11 | 8.73x | [8.65, 8.82] |
| JavaScript | tiktoken | o200k_base | wiki-ar | stream | 11 | 6.18x | [6.08, 6.27] |
| JavaScript | tiktoken | o200k_base | wiki-de | cold | 11 | 5.81x | [5.80, 5.83] |
| JavaScript | tiktoken | o200k_base | wiki-de | repeat | 11 | 8.37x | [8.33, 8.41] |
| JavaScript | tiktoken | o200k_base | wiki-de | stream | 11 | 6.92x | [6.86, 6.97] |
| JavaScript | tiktoken | o200k_base | wiki-en | cold | 11 | 10.69x | [10.58, 10.79] |
| JavaScript | tiktoken | o200k_base | wiki-en | repeat | 11 | 19.47x | [19.04, 19.89] |
| JavaScript | tiktoken | o200k_base | wiki-en | stream | 11 | 12.26x | [12.13, 12.40] |
| JavaScript | tiktoken | o200k_base | wiki-es | cold | 11 | 7.25x | [7.20, 7.31] |
| JavaScript | tiktoken | o200k_base | wiki-es | repeat | 11 | 13.99x | [13.74, 14.22] |
| JavaScript | tiktoken | o200k_base | wiki-es | stream | 11 | 9.06x | [8.99, 9.13] |
| JavaScript | tiktoken | o200k_base | wiki-fr | cold | 11 | 7.07x | [7.04, 7.11] |
| JavaScript | tiktoken | o200k_base | wiki-fr | repeat | 11 | 11.92x | [11.73, 12.08] |
| JavaScript | tiktoken | o200k_base | wiki-fr | stream | 11 | 9.02x | [8.84, 9.21] |
| JavaScript | tiktoken | o200k_base | wiki-hi | cold | 11 | 5.32x | [5.29, 5.35] |
| JavaScript | tiktoken | o200k_base | wiki-hi | repeat | 11 | 9.15x | [9.04, 9.26] |
| JavaScript | tiktoken | o200k_base | wiki-hi | stream | 11 | 6.37x | [6.32, 6.42] |
| JavaScript | tiktoken | o200k_base | wiki-ja | cold | 11 | 9.12x | [9.05, 9.19] |
| JavaScript | tiktoken | o200k_base | wiki-ja | repeat | 11 | 18.16x | [17.72, 18.49] |
| JavaScript | tiktoken | o200k_base | wiki-ja | stream | 11 | 10.87x | [10.77, 10.98] |
| JavaScript | tiktoken | o200k_base | wiki-ko | cold | 11 | 5.80x | [5.73, 5.87] |
| JavaScript | tiktoken | o200k_base | wiki-ko | repeat | 11 | 14.64x | [13.98, 15.05] |
| JavaScript | tiktoken | o200k_base | wiki-ko | stream | 11 | 6.06x | [5.96, 6.17] |
| JavaScript | tiktoken | o200k_base | wiki-ru | cold | 11 | 5.29x | [5.26, 5.31] |
| JavaScript | tiktoken | o200k_base | wiki-ru | repeat | 11 | 9.31x | [9.06, 9.47] |
| JavaScript | tiktoken | o200k_base | wiki-ru | stream | 11 | 5.95x | [5.90, 6.01] |
| JavaScript | tiktoken | o200k_base | wiki-th | cold | 11 | 8.94x | [8.89, 8.99] |
| JavaScript | tiktoken | o200k_base | wiki-th | repeat | 11 | 15.02x | [14.93, 15.10] |
| JavaScript | tiktoken | o200k_base | wiki-th | stream | 11 | 10.67x | [10.61, 10.72] |
| JavaScript | tiktoken | o200k_base | wiki-zh | cold | 11 | 9.03x | [8.90, 9.16] |
| JavaScript | tiktoken | o200k_base | wiki-zh | repeat | 11 | 19.02x | [18.67, 19.24] |
| JavaScript | tiktoken | o200k_base | wiki-zh | stream | 11 | 9.72x | [9.60, 9.86] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 6.26x | [6.09, 6.44] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 4.70x | [4.23, 4.99] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.30x | [3.19, 3.40] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 40.42x | [35.13, 44.88] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 5.99x | [5.74, 6.30] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.32x | [4.95, 5.83] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 13.56x | [12.97, 14.03] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 11.56x | [11.46, 11.65] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 9.96x | [9.40, 10.38] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 4.02x | [3.61, 4.57] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 3.34x | [3.09, 3.54] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 3.28x | [3.03, 3.58] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 3.63x | [3.05, 4.44] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 5.14x | [4.54, 5.76] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 14.71x | [14.54, 14.91] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 2.86x | [2.84, 2.88] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 11.29x | [11.20, 11.40] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 6.55x | [6.49, 6.64] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 2.12x | [2.09, 2.15] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 6.64x | [6.52, 6.76] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 4.97x | [4.83, 5.10] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 5.27x | [5.17, 5.37] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 5.48x | [5.32, 5.65] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 6.43x | [6.22, 6.64] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 3.16x | [3.13, 3.19] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 6.34x | [6.09, 6.64] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 6.95x | [6.76, 7.19] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 2.94x | [2.92, 2.97] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 6.04x | [5.89, 6.19] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 9.48x | [8.89, 10.19] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 4.38x | [4.33, 4.43] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 4.90x | [4.85, 4.95] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 34.99x | [34.61, 35.35] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.32x | [3.22, 3.40] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 38.54x | [37.70, 39.52] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 14.16x | [13.87, 14.45] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.03x | [2.90, 3.12] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 12.38x | [11.99, 12.92] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 15.89x | [15.71, 16.08] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 2.11x | [2.09, 2.13] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 12.25x | [12.10, 12.40] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 34.50x | [33.74, 35.38] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 4.54x | [4.52, 4.55] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 29.14x | [28.49, 29.92] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 39.80x | [39.20, 40.37] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 4.28x | [4.21, 4.34] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 40.50x | [39.76, 41.40] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 6.49x | [5.55, 7.60] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 6.16x | [5.64, 6.64] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.26x | [2.24, 2.27] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 32.15x | [30.77, 33.62] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 6.01x | [5.42, 6.82] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 5.28x | [5.00, 5.55] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 12.58x | [12.30, 12.82] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.26x | [1.25, 1.28] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 11.34x | [11.12, 11.57] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 4.23x | [3.90, 4.81] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 3.78x | [3.26, 4.72] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 3.54x | [3.24, 3.85] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 3.50x | [3.06, 4.07] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 4.91x | [4.29, 5.48] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 12.82x | [12.45, 13.26] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.17x | [2.14, 2.20] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 9.58x | [9.39, 9.75] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 6.62x | [6.46, 6.78] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 2.34x | [2.32, 2.36] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 6.83x | [6.61, 7.08] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 5.27x | [5.19, 5.35] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 5.77x | [5.64, 5.90] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 6.12x | [6.00, 6.23] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 6.30x | [6.12, 6.48] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 3.76x | [3.70, 3.83] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 6.51x | [6.22, 7.04] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 6.97x | [6.77, 7.14] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 3.40x | [3.35, 3.45] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 6.43x | [6.33, 6.53] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 10.94x | [10.76, 11.13] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 2.01x | [1.98, 2.03] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 8.91x | [8.84, 8.99] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 30.76x | [30.21, 31.56] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.07x | [3.00, 3.14] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 36.31x | [35.45, 37.43] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 14.41x | [14.02, 14.77] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.34x | [3.17, 3.46] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 12.21x | [11.96, 12.49] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 14.81x | [14.48, 15.22] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.18x | [2.12, 2.22] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 11.91x | [11.77, 12.07] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 23.13x | [22.84, 23.53] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.16x | [1.14, 1.17] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 27.18x | [27.03, 27.32] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 33.54x | [32.67, 34.50] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 3.56x | [3.50, 3.60] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 35.68x | [35.20, 36.19] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | repeat | 11 | 50.71x | [49.30, 52.39] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | stream | 11 | 23.41x | [20.14, 25.61] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 178.51x | [175.75, 181.11] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 153.95x | [133.89, 170.55] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | repeat | 11 | 40.19x | [38.08, 42.74] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | stream | 11 | 17.72x | [17.28, 18.18] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | cold | 11 | 46.72x | [44.88, 48.01] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | repeat | 11 | 54.34x | [54.00, 54.73] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | stream | 11 | 48.62x | [45.66, 50.62] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | cold | 11 | 27.49x | [25.59, 29.28] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | repeat | 11 | 31.81x | [29.50, 34.00] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | stream | 11 | 25.88x | [23.95, 28.01] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 18.39x | [16.29, 21.16] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 24.24x | [22.52, 25.75] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | cold | 11 | 48.55x | [48.20, 48.91] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | repeat | 11 | 101.05x | [99.91, 102.05] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | stream | 11 | 69.07x | [68.28, 69.93] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | cold | 11 | 29.37x | [29.23, 29.53] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | repeat | 11 | 45.23x | [44.77, 45.65] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | stream | 11 | 37.94x | [37.47, 38.49] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | cold | 11 | 20.89x | [20.63, 21.11] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | repeat | 11 | 36.96x | [36.20, 37.53] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | stream | 11 | 24.07x | [23.64, 24.50] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | cold | 11 | 25.14x | [24.89, 25.39] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | repeat | 11 | 53.49x | [52.77, 54.32] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | stream | 11 | 36.05x | [35.55, 36.60] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | cold | 11 | 23.78x | [23.64, 23.91] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | repeat | 11 | 44.54x | [44.16, 44.86] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | stream | 11 | 33.99x | [33.22, 34.91] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | cold | 11 | 42.49x | [42.17, 42.78] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | repeat | 11 | 89.55x | [88.78, 90.31] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | stream | 11 | 63.56x | [63.27, 63.90] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | cold | 11 | 283.16x | [279.42, 286.72] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | repeat | 11 | 526.48x | [510.71, 536.84] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | stream | 11 | 315.51x | [312.07, 319.07] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | cold | 11 | 39.48x | [38.98, 39.96] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | repeat | 11 | 103.07x | [98.46, 106.05] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | stream | 11 | 44.00x | [43.24, 44.79] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | cold | 11 | 62.87x | [62.35, 63.39] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | repeat | 11 | 114.52x | [113.47, 115.54] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | stream | 11 | 74.97x | [73.87, 76.05] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | cold | 11 | 163.05x | [160.79, 165.18] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | repeat | 11 | 271.36x | [269.48, 273.35] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | stream | 11 | 186.49x | [183.18, 189.06] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | cold | 11 | 220.69x | [218.01, 223.64] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | repeat | 11 | 440.28x | [430.96, 447.21] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | stream | 11 | 226.71x | [223.81, 229.61] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | repeat | 11 | 26.76x | [22.17, 33.64] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | stream | 11 | 19.95x | [18.50, 21.19] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 131.83x | [129.89, 134.01] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | stream | 11 | 119.75x | [114.71, 124.91] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | repeat | 11 | 37.82x | [33.76, 43.60] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | stream | 11 | 18.45x | [16.97, 20.85] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | cold | 11 | 69.99x | [68.78, 70.95] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | repeat | 11 | 78.53x | [77.73, 79.35] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | stream | 11 | 72.82x | [72.10, 73.41] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | cold | 11 | 27.60x | [26.48, 28.78] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | repeat | 11 | 30.86x | [28.70, 33.19] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | stream | 11 | 24.67x | [22.88, 26.49] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 18.64x | [16.75, 21.05] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 24.22x | [22.20, 26.22] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | cold | 11 | 34.54x | [34.33, 34.76] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | repeat | 11 | 54.53x | [53.82, 55.30] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | stream | 11 | 38.60x | [37.95, 39.26] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | cold | 11 | 27.18x | [27.02, 27.34] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | repeat | 11 | 38.85x | [38.62, 39.09] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | stream | 11 | 32.72x | [32.29, 33.22] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | cold | 11 | 20.12x | [19.91, 20.31] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | repeat | 11 | 36.65x | [35.78, 37.48] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | stream | 11 | 23.22x | [22.99, 23.49] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | cold | 11 | 21.56x | [21.41, 21.71] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | repeat | 11 | 41.37x | [40.62, 42.09] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | stream | 11 | 26.99x | [26.81, 27.20] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | cold | 11 | 20.91x | [20.75, 21.09] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | repeat | 11 | 35.40x | [34.72, 36.09] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | stream | 11 | 27.00x | [26.46, 27.59] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | cold | 11 | 51.48x | [51.16, 51.75] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | repeat | 11 | 88.12x | [86.90, 89.11] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | stream | 11 | 61.71x | [61.20, 62.17] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | cold | 11 | 235.99x | [232.90, 238.81] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | repeat | 11 | 473.97x | [460.75, 484.16] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | stream | 11 | 281.38x | [278.52, 284.41] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | cold | 11 | 38.59x | [38.13, 39.07] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | repeat | 11 | 96.29x | [91.66, 99.07] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | stream | 11 | 39.77x | [39.24, 40.31] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | cold | 11 | 52.73x | [52.18, 53.24] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | repeat | 11 | 93.15x | [90.66, 95.06] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | stream | 11 | 58.75x | [58.13, 59.39] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | cold | 11 | 449.04x | [446.25, 451.72] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | repeat | 11 | 763.94x | [755.31, 771.62] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | stream | 11 | 544.75x | [539.95, 548.91] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | cold | 11 | 177.50x | [175.45, 179.43] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | repeat | 11 | 373.61x | [366.49, 378.05] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | stream | 11 | 191.39x | [188.73, 194.04] |
| Node, native | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 31.33x | [30.08, 32.44] |
| Node, native | tiktoken | cl100k_base | book-gosrc | stream | 11 | 13.49x | [13.13, 13.84] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 16.75x | [16.15, 17.37] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 17.37x | [16.49, 18.05] |
| Node, native | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 27.98x | [26.40, 29.24] |
| Node, native | tiktoken | cl100k_base | book-mobydick | stream | 11 | 10.13x | [9.75, 10.49] |
| Node, native | tiktoken | cl100k_base | chat-512B | cold | 11 | 6.71x | [6.59, 6.84] |
| Node, native | tiktoken | cl100k_base | chat-512B | repeat | 11 | 8.71x | [8.56, 8.85] |
| Node, native | tiktoken | cl100k_base | chat-512B | stream | 11 | 7.87x | [7.52, 8.14] |
| Node, native | tiktoken | cl100k_base | td-code-files | cold | 11 | 30.95x | [30.26, 31.68] |
| Node, native | tiktoken | cl100k_base | td-code-files | repeat | 11 | 40.17x | [37.08, 42.17] |
| Node, native | tiktoken | cl100k_base | td-code-files | stream | 11 | 33.23x | [31.95, 34.51] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 51.91x | [41.01, 62.45] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 54.13x | [49.88, 58.37] |
| Node, native | tiktoken | cl100k_base | wiki-ar | cold | 11 | 7.29x | [7.18, 7.41] |
| Node, native | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 14.40x | [14.21, 14.59] |
| Node, native | tiktoken | cl100k_base | wiki-ar | stream | 11 | 10.24x | [10.15, 10.33] |
| Node, native | tiktoken | cl100k_base | wiki-de | cold | 11 | 6.11x | [6.04, 6.21] |
| Node, native | tiktoken | cl100k_base | wiki-de | repeat | 11 | 9.90x | [9.83, 9.96] |
| Node, native | tiktoken | cl100k_base | wiki-de | stream | 11 | 8.13x | [8.05, 8.21] |
| Node, native | tiktoken | cl100k_base | wiki-en | cold | 11 | 10.79x | [10.65, 10.92] |
| Node, native | tiktoken | cl100k_base | wiki-en | repeat | 11 | 23.05x | [22.88, 23.23] |
| Node, native | tiktoken | cl100k_base | wiki-en | stream | 11 | 14.54x | [14.42, 14.65] |
| Node, native | tiktoken | cl100k_base | wiki-es | cold | 11 | 6.61x | [6.57, 6.65] |
| Node, native | tiktoken | cl100k_base | wiki-es | repeat | 11 | 15.68x | [15.39, 15.94] |
| Node, native | tiktoken | cl100k_base | wiki-es | stream | 11 | 10.25x | [10.13, 10.37] |
| Node, native | tiktoken | cl100k_base | wiki-fr | cold | 11 | 6.40x | [6.35, 6.46] |
| Node, native | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 13.66x | [13.59, 13.74] |
| Node, native | tiktoken | cl100k_base | wiki-fr | stream | 11 | 10.46x | [10.37, 10.55] |
| Node, native | tiktoken | cl100k_base | wiki-hi | cold | 11 | 11.51x | [11.38, 11.65] |
| Node, native | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 22.91x | [22.78, 23.02] |
| Node, native | tiktoken | cl100k_base | wiki-hi | stream | 11 | 15.71x | [15.64, 15.77] |
| Node, native | tiktoken | cl100k_base | wiki-ja | cold | 11 | 11.67x | [11.51, 11.81] |
| Node, native | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 24.27x | [23.60, 24.78] |
| Node, native | tiktoken | cl100k_base | wiki-ja | stream | 11 | 14.48x | [14.38, 14.60] |
| Node, native | tiktoken | cl100k_base | wiki-ko | cold | 11 | 7.06x | [7.01, 7.10] |
| Node, native | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 16.86x | [16.21, 17.24] |
| Node, native | tiktoken | cl100k_base | wiki-ko | stream | 11 | 7.64x | [7.53, 7.76] |
| Node, native | tiktoken | cl100k_base | wiki-ru | cold | 11 | 5.92x | [5.86, 5.97] |
| Node, native | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 12.17x | [12.04, 12.30] |
| Node, native | tiktoken | cl100k_base | wiki-ru | stream | 11 | 8.02x | [7.98, 8.06] |
| Node, native | tiktoken | cl100k_base | wiki-th | cold | 11 | 13.86x | [13.73, 13.98] |
| Node, native | tiktoken | cl100k_base | wiki-th | repeat | 11 | 25.68x | [25.54, 25.81] |
| Node, native | tiktoken | cl100k_base | wiki-th | stream | 11 | 16.55x | [16.41, 16.68] |
| Node, native | tiktoken | cl100k_base | wiki-zh | cold | 11 | 12.67x | [12.47, 12.89] |
| Node, native | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 27.39x | [26.85, 27.71] |
| Node, native | tiktoken | cl100k_base | wiki-zh | stream | 11 | 14.20x | [13.97, 14.39] |
| Node, native | tiktoken | o200k_base | book-gosrc | repeat | 11 | 31.77x | [27.66, 35.30] |
| Node, native | tiktoken | o200k_base | book-gosrc | stream | 11 | 20.52x | [20.13, 20.79] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 11.44x | [11.31, 11.61] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 10.94x | [10.42, 11.34] |
| Node, native | tiktoken | o200k_base | book-mobydick | repeat | 11 | 28.05x | [24.31, 31.04] |
| Node, native | tiktoken | o200k_base | book-mobydick | stream | 11 | 10.36x | [10.17, 10.48] |
| Node, native | tiktoken | o200k_base | chat-512B | cold | 11 | 6.51x | [6.44, 6.58] |
| Node, native | tiktoken | o200k_base | chat-512B | repeat | 11 | 7.54x | [7.47, 7.62] |
| Node, native | tiktoken | o200k_base | chat-512B | stream | 11 | 7.01x | [6.94, 7.08] |
| Node, native | tiktoken | o200k_base | td-code-files | cold | 11 | 30.74x | [30.01, 31.44] |
| Node, native | tiktoken | o200k_base | td-code-files | repeat | 11 | 46.13x | [44.19, 47.50] |
| Node, native | tiktoken | o200k_base | td-code-files | stream | 11 | 34.12x | [33.15, 34.88] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 67.70x | [55.46, 79.40] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 73.95x | [68.47, 79.63] |
| Node, native | tiktoken | o200k_base | wiki-ar | cold | 11 | 6.32x | [6.25, 6.40] |
| Node, native | tiktoken | o200k_base | wiki-ar | repeat | 11 | 10.29x | [10.20, 10.38] |
| Node, native | tiktoken | o200k_base | wiki-ar | stream | 11 | 7.28x | [7.20, 7.35] |
| Node, native | tiktoken | o200k_base | wiki-de | cold | 11 | 6.20x | [6.17, 6.24] |
| Node, native | tiktoken | o200k_base | wiki-de | repeat | 11 | 9.33x | [9.27, 9.39] |
| Node, native | tiktoken | o200k_base | wiki-de | stream | 11 | 7.79x | [7.71, 7.86] |
| Node, native | tiktoken | o200k_base | wiki-en | cold | 11 | 10.52x | [10.39, 10.64] |
| Node, native | tiktoken | o200k_base | wiki-en | repeat | 11 | 23.74x | [23.57, 23.90] |
| Node, native | tiktoken | o200k_base | wiki-en | stream | 11 | 13.87x | [13.75, 14.01] |
| Node, native | tiktoken | o200k_base | wiki-es | cold | 11 | 7.33x | [7.26, 7.39] |
| Node, native | tiktoken | o200k_base | wiki-es | repeat | 11 | 16.30x | [16.02, 16.55] |
| Node, native | tiktoken | o200k_base | wiki-es | stream | 11 | 10.27x | [10.14, 10.38] |
| Node, native | tiktoken | o200k_base | wiki-fr | cold | 11 | 7.07x | [6.96, 7.16] |
| Node, native | tiktoken | o200k_base | wiki-fr | repeat | 11 | 13.65x | [13.54, 13.75] |
| Node, native | tiktoken | o200k_base | wiki-fr | stream | 11 | 10.52x | [10.48, 10.56] |
| Node, native | tiktoken | o200k_base | wiki-hi | cold | 11 | 6.28x | [6.22, 6.33] |
| Node, native | tiktoken | o200k_base | wiki-hi | repeat | 11 | 10.69x | [10.55, 10.81] |
| Node, native | tiktoken | o200k_base | wiki-hi | stream | 11 | 7.39x | [7.35, 7.43] |
| Node, native | tiktoken | o200k_base | wiki-ja | cold | 11 | 9.73x | [9.61, 9.84] |
| Node, native | tiktoken | o200k_base | wiki-ja | repeat | 11 | 20.89x | [20.50, 21.20] |
| Node, native | tiktoken | o200k_base | wiki-ja | stream | 11 | 12.29x | [12.23, 12.34] |
| Node, native | tiktoken | o200k_base | wiki-ko | cold | 11 | 6.67x | [6.60, 6.73] |
| Node, native | tiktoken | o200k_base | wiki-ko | repeat | 11 | 15.60x | [14.89, 16.03] |
| Node, native | tiktoken | o200k_base | wiki-ko | stream | 11 | 6.86x | [6.72, 7.00] |
| Node, native | tiktoken | o200k_base | wiki-ru | cold | 11 | 5.40x | [5.34, 5.47] |
| Node, native | tiktoken | o200k_base | wiki-ru | repeat | 11 | 10.79x | [10.74, 10.84] |
| Node, native | tiktoken | o200k_base | wiki-ru | stream | 11 | 6.91x | [6.87, 6.96] |
| Node, native | tiktoken | o200k_base | wiki-th | cold | 11 | 10.72x | [10.52, 10.87] |
| Node, native | tiktoken | o200k_base | wiki-th | repeat | 11 | 18.66x | [18.46, 18.84] |
| Node, native | tiktoken | o200k_base | wiki-th | stream | 11 | 12.50x | [12.41, 12.62] |
| Node, native | tiktoken | o200k_base | wiki-zh | cold | 11 | 9.88x | [9.71, 10.01] |
| Node, native | tiktoken | o200k_base | wiki-zh | repeat | 11 | 22.29x | [21.89, 22.60] |
| Node, native | tiktoken | o200k_base | wiki-zh | stream | 11 | 11.54x | [11.35, 11.69] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 7.85x | [7.51, 8.17] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 5.52x | [5.25, 5.83] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.70x | [3.53, 3.89] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 54.89x | [51.83, 57.27] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 7.85x | [7.43, 8.19] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.76x | [5.42, 6.17] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 14.72x | [14.37, 15.09] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 13.62x | [13.38, 13.89] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 11.66x | [11.14, 12.08] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 5.70x | [5.37, 6.18] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 5.19x | [4.78, 5.49] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 5.22x | [4.99, 5.47] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 8.89x | [6.67, 11.23] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 10.00x | [8.88, 11.14] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 17.56x | [17.26, 17.87] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 3.23x | [3.18, 3.27] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 13.32x | [13.19, 13.47] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 7.04x | [6.95, 7.16] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 2.39x | [2.37, 2.41] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 7.44x | [7.32, 7.55] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 4.85x | [4.72, 5.00] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 6.23x | [6.14, 6.31] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 6.26x | [6.12, 6.44] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 6.57x | [6.38, 6.77] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 3.59x | [3.53, 3.64] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 7.05x | [6.73, 7.50] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 6.94x | [6.78, 7.14] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 3.34x | [3.31, 3.37] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 7.04x | [6.88, 7.22] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 11.32x | [10.50, 12.27] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 4.93x | [4.88, 4.97] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 5.36x | [5.32, 5.42] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 35.60x | [35.05, 36.26] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.83x | [3.73, 3.92] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 43.80x | [43.00, 44.81] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 16.77x | [16.52, 17.05] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.26x | [3.12, 3.35] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 14.12x | [13.54, 14.92] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 16.13x | [15.96, 16.32] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 2.43x | [2.40, 2.46] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 14.08x | [13.91, 14.26] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 36.83x | [36.01, 37.74] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 5.37x | [5.34, 5.40] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 32.43x | [31.81, 33.19] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 41.47x | [40.65, 42.52] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 4.87x | [4.79, 4.93] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 46.19x | [45.31, 47.14] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 9.88x | [8.45, 11.19] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 7.73x | [7.46, 8.02] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.67x | [2.66, 2.67] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 40.33x | [38.20, 41.99] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 8.98x | [7.76, 9.97] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 6.21x | [6.07, 6.38] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 13.83x | [13.58, 14.10] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.44x | [1.42, 1.46] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 12.76x | [12.54, 13.02] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 5.45x | [5.00, 6.28] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 6.41x | [5.71, 7.75] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 5.50x | [5.20, 5.75] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 9.94x | [8.30, 11.78] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 11.71x | [10.54, 12.98] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 14.55x | [14.03, 15.07] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.56x | [2.53, 2.59] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 11.28x | [11.10, 11.51] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 7.06x | [6.89, 7.23] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 2.61x | [2.58, 2.64] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 7.69x | [7.42, 8.01] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 5.18x | [5.08, 5.31] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 7.04x | [6.97, 7.10] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 6.92x | [6.76, 7.06] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 6.37x | [6.18, 6.58] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 4.38x | [4.30, 4.46] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 7.38x | [7.04, 7.92] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 6.97x | [6.75, 7.16] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 3.89x | [3.87, 3.92] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 7.50x | [7.36, 7.64] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 12.91x | [12.68, 13.13] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 2.35x | [2.32, 2.37] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 10.34x | [10.27, 10.41] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 32.83x | [32.27, 33.51] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.54x | [3.46, 3.60] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 41.03x | [40.20, 41.98] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 16.56x | [16.10, 16.96] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.56x | [3.36, 3.68] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 13.83x | [13.45, 14.21] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 15.13x | [14.74, 15.54] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.53x | [2.51, 2.54] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 13.84x | [13.66, 13.99] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 27.72x | [26.98, 28.52] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.44x | [1.42, 1.46] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 31.86x | [31.59, 32.14] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 36.69x | [35.89, 37.68] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 4.18x | [4.10, 4.23] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 42.36x | [41.51, 43.08] |
| Go | tiktoken-go | cl100k_base | book-gosrc | stream | 11 | 11.47x | [9.08, 13.84] |
| Go | tiktoken-go | cl100k_base | book-hongloumeng | stream | 11 | 2.94x | [2.59, 3.35] |
| Go | tiktoken-go | cl100k_base | book-mobydick | stream | 11 | 8.72x | [6.93, 10.97] |
| Go | tiktoken-go | cl100k_base | chat-512B | stream | 11 | 4.70x | [4.54, 4.84] |
| Go | tiktoken-go | cl100k_base | td-code-files | stream | 11 | 20.80x | [18.43, 23.43] |
| Go | tiktoken-go | cl100k_base | td-synthetic-1MB * | stream | 11 | 19.65x | [15.72, 23.79] |
| Go | tiktoken-go | cl100k_base | wiki-ar | stream | 11 | 5.98x | [5.43, 6.48] |
| Go | tiktoken-go | cl100k_base | wiki-de | stream | 11 | 6.98x | [6.60, 7.28] |
| Go | tiktoken-go | cl100k_base | wiki-en | stream | 11 | 11.79x | [11.24, 12.36] |
| Go | tiktoken-go | cl100k_base | wiki-es | stream | 11 | 8.22x | [7.39, 8.96] |
| Go | tiktoken-go | cl100k_base | wiki-fr | stream | 11 | 9.21x | [8.79, 9.54] |
| Go | tiktoken-go | cl100k_base | wiki-hi | stream | 11 | 8.94x | [8.32, 9.58] |
| Go | tiktoken-go | cl100k_base | wiki-ja | stream | 11 | 3.52x | [3.35, 3.66] |
| Go | tiktoken-go | cl100k_base | wiki-ko | stream | 11 | 5.12x | [4.47, 5.88] |
| Go | tiktoken-go | cl100k_base | wiki-ru | stream | 11 | 4.75x | [4.40, 5.06] |
| Go | tiktoken-go | cl100k_base | wiki-th | stream | 11 | 4.50x | [4.18, 4.78] |
| Go | tiktoken-go | cl100k_base | wiki-zh | stream | 11 | 3.27x | [3.08, 3.39] |
| Go | tiktoken-go | o200k_base | book-gosrc | stream | 11 | 15.18x | [12.32, 17.54] |
| Go | tiktoken-go | o200k_base | book-hongloumeng | stream | 11 | 3.01x | [2.69, 3.30] |
| Go | tiktoken-go | o200k_base | book-mobydick | stream | 11 | 10.54x | [8.59, 12.90] |
| Go | tiktoken-go | o200k_base | chat-512B | stream | 11 | 4.88x | [4.72, 5.02] |
| Go | tiktoken-go | o200k_base | td-code-files | stream | 11 | 20.33x | [17.48, 23.34] |
| Go | tiktoken-go | o200k_base | td-synthetic-1MB * | stream | 11 | 24.35x | [21.99, 26.26] |
| Go | tiktoken-go | o200k_base | wiki-ar | stream | 11 | 7.82x | [7.69, 7.96] |
| Go | tiktoken-go | o200k_base | wiki-de | stream | 11 | 7.53x | [6.93, 8.11] |
| Go | tiktoken-go | o200k_base | wiki-en | stream | 11 | 13.23x | [12.15, 14.17] |
| Go | tiktoken-go | o200k_base | wiki-es | stream | 11 | 9.36x | [8.44, 10.20] |
| Go | tiktoken-go | o200k_base | wiki-fr | stream | 11 | 10.43x | [10.04, 10.78] |
| Go | tiktoken-go | o200k_base | wiki-hi | stream | 11 | 7.12x | [6.24, 7.99] |
| Go | tiktoken-go | o200k_base | wiki-ja | stream | 11 | 3.92x | [3.76, 4.03] |
| Go | tiktoken-go | o200k_base | wiki-ko | stream | 11 | 6.20x | [5.35, 6.98] |
| Go | tiktoken-go | o200k_base | wiki-ru | stream | 11 | 5.42x | [5.24, 5.60] |
| Go | tiktoken-go | o200k_base | wiki-th | stream | 11 | 3.30x | [3.10, 3.46] |
| Go | tiktoken-go | o200k_base | wiki-zh | stream | 11 | 3.19x | [3.03, 3.34] |
| Go | tokenizer | cl100k_base | book-gosrc | stream | 11 | 6.02x | [4.83, 7.63] |
| Go | tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 2.56x | [2.21, 3.00] |
| Go | tokenizer | cl100k_base | book-mobydick | stream | 11 | 4.44x | [3.65, 5.30] |
| Go | tokenizer | cl100k_base | chat-512B | stream | 11 | 3.19x | [3.08, 3.28] |
| Go | tokenizer | cl100k_base | td-code-files | stream | 11 | 10.08x | [9.05, 11.19] |
| Go | tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 10.08x | [7.98, 12.47] |
| Go | tokenizer | cl100k_base | wiki-ar | stream | 11 | 4.31x | [3.91, 4.68] |
| Go | tokenizer | cl100k_base | wiki-de | stream | 11 | 4.70x | [4.35, 5.00] |
| Go | tokenizer | cl100k_base | wiki-en | stream | 11 | 6.40x | [6.00, 6.83] |
| Go | tokenizer | cl100k_base | wiki-es | stream | 11 | 5.01x | [4.57, 5.44] |
| Go | tokenizer | cl100k_base | wiki-fr | stream | 11 | 5.54x | [5.30, 5.79] |
| Go | tokenizer | cl100k_base | wiki-hi | stream | 11 | 6.22x | [5.65, 6.76] |
| Go | tokenizer | cl100k_base | wiki-ja | stream | 11 | 2.95x | [2.80, 3.05] |
| Go | tokenizer | cl100k_base | wiki-ko | stream | 11 | 3.79x | [3.11, 4.59] |
| Go | tokenizer | cl100k_base | wiki-ru | stream | 11 | 3.50x | [3.24, 3.75] |
| Go | tokenizer | cl100k_base | wiki-th | stream | 11 | 3.61x | [3.33, 3.84] |
| Go | tokenizer | cl100k_base | wiki-zh | stream | 11 | 2.75x | [2.58, 2.87] |
| Go | tokenizer | o200k_base | book-gosrc | stream | 11 | 8.30x | [6.74, 10.58] |
| Go | tokenizer | o200k_base | book-hongloumeng | stream | 11 | 2.27x | [2.03, 2.57] |
| Go | tokenizer | o200k_base | book-mobydick | stream | 11 | 4.34x | [3.60, 5.09] |
| Go | tokenizer | o200k_base | chat-512B | stream | 11 | 2.97x | [2.85, 3.07] |
| Go | tokenizer | o200k_base | td-code-files | stream | 11 | 9.49x | [8.30, 10.66] |
| Go | tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 10.80x | [9.11, 12.69] |
| Go | tokenizer | o200k_base | wiki-ar | stream | 11 | 4.54x | [4.49, 4.60] |
| Go | tokenizer | o200k_base | wiki-de | stream | 11 | 4.30x | [3.93, 4.65] |
| Go | tokenizer | o200k_base | wiki-en | stream | 11 | 6.73x | [5.87, 7.49] |
| Go | tokenizer | o200k_base | wiki-es | stream | 11 | 5.09x | [4.44, 5.70] |
| Go | tokenizer | o200k_base | wiki-fr | stream | 11 | 5.89x | [5.59, 6.20] |
| Go | tokenizer | o200k_base | wiki-hi | stream | 11 | 4.27x | [3.84, 4.72] |
| Go | tokenizer | o200k_base | wiki-ja | stream | 11 | 2.98x | [2.87, 3.06] |
| Go | tokenizer | o200k_base | wiki-ko | stream | 11 | 3.51x | [3.12, 3.92] |
| Go | tokenizer | o200k_base | wiki-ru | stream | 11 | 3.62x | [3.50, 3.74] |
| Go | tokenizer | o200k_base | wiki-th | stream | 11 | 2.49x | [2.36, 2.60] |
| Go | tokenizer | o200k_base | wiki-zh | stream | 11 | 2.44x | [2.30, 2.56] |
