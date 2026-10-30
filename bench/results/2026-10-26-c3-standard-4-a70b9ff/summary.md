# Benchmark run 2026-10-26-c3-standard-4-a70b9ff

- Machine: Intel(R) Xeon(R) Platinum 8481C CPU @ 2.70GHz, c3-standard-4, kernel 6.1.0-54-cloud-amd64
- Commit a70b9ff00902bc69836fb16cba26ed5b0323b27e, started 2026-10-26T17:56:28+00:00
- Pinned to core 2; CPU steal during the run: 0.0%
- Toolchains: rustc 1.99.1, python 3.11.2, tiktoken (PyPI) 0.14.0, node 22.23.4, go 1.26.0, tiktoken (npm) 1.0.22, gpt-tokenizer (npm) 4.1.0, js-tiktoken (npm) 1.0.21

Token check: **66860 documents** compared with the reference before timing, **0 with different tokens**.

## Headline

Geometric mean over 16 workloads × 2 encodings, 95% interval in brackets.

| | over | new text | fresh caches | same text | slowest row (new text) | rows slower |
|---|---|---|---|---|---|---:|
| Rust | tiktoken's core (tiktoken-rs 0.12.1) | **3.55x** [3.54, 3.57] | **2.86x** [2.85, 2.86] | **6.32x** [6.30, 6.35] | 1.70x o200k wiki-de | 0 of 32 |
| Rust | bpe-openai 0.3.2 | **1.98x** [1.97, 1.98] | **1.57x** [1.56, 1.57] | **3.49x** [3.47, 3.50] | 1.13x o200k wiki-ko | 0 of 32 |
| Python | tiktoken 0.14.0 | **3.28x** [3.26, 3.30] | **2.72x** [2.70, 2.73] |  | 1.80x o200k wiki-de | 0 of 32 |
| Python, encode() | tiktoken 0.14.0 | **3.21x** [3.19, 3.23] | **2.64x** [2.62, 2.65] |  | 1.84x o200k wiki-de | 0 of 31 |
| JavaScript | tiktoken 1.0.22 (npm) | **7.66x** [7.60, 7.71] | **6.37x** [6.36, 6.39] | **11.87x** [11.75, 11.99] | 4.64x o200k chat-512B | 0 of 32 |
| JavaScript | gpt-tokenizer 4.1.0 | **10.24x** [10.14, 10.32] | **11.44x** [11.36, 11.52] | **3.13x** [3.10, 3.16] | 3.37x o200k td-code-files | 0 of 32 |
| JavaScript | js-tiktoken 1.0.21 | **54.39x** [53.93, 54.79] | **50.61x** [50.46, 50.76] | **83.90x** [83.08, 84.73] | 17.13x cl100k book-mobydick | 0 of 32 |
| Node, native | tiktoken 1.0.22 (npm) | **9.19x** [9.16, 9.21] | **7.23x** [7.21, 7.25] | **14.56x** [14.41, 14.69] | 5.27x o200k chat-512B | 0 of 32 |
| Node, native | gpt-tokenizer 4.1.0 | **12.28x** [12.21, 12.35] | **12.97x** [12.89, 13.06] | **3.84x** [3.79, 3.88] | 5.12x cl100k td-code-files | 0 of 32 |
| Go | pkoukk/tiktoken-go (200cf60, vendored) | **6.44x** [6.31, 6.57] |  |  | 2.69x o200k book-hongloumeng | 0 of 32 |
| Go | tiktoken-go/tokenizer 0.8.1 | **4.37x** [4.27, 4.47] |  |  | 2.32x o200k book-hongloumeng | 0 of 32 |

Documents where another library's tokens differ from its reference (timed anyway): go/tokenizer 599, js/gpt-tokenizer 0, js/js-tiktoken 0, rust/bpe-openai 0.

## Every row

Rows marked * are outside the headline.

| | over | encoding | workload | scenario | rounds | speed-up | 95% interval |
|---|---|---|---|---|---:|---:|---|
| Rust | tiktoken-rs | cl100k_base | book-gosrc | repeat | 11 | 14.80x | [14.22, 15.38] |
| Rust | tiktoken-rs | cl100k_base | book-gosrc | stream | 11 | 4.23x | [4.00, 4.44] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | repeat | 11 | 10.05x | [9.96, 10.14] |
| Rust | tiktoken-rs | cl100k_base | book-hongloumeng | stream | 11 | 9.47x | [9.18, 9.73] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | repeat | 11 | 13.87x | [13.10, 14.48] |
| Rust | tiktoken-rs | cl100k_base | book-mobydick | stream | 11 | 3.04x | [2.89, 3.18] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | cold | 11 | 2.48x | [2.45, 2.50] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | repeat | 11 | 3.12x | [3.08, 3.16] |
| Rust | tiktoken-rs | cl100k_base | chat-512B | stream | 11 | 2.73x | [2.64, 2.80] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | cold | 11 | 8.15x | [8.06, 8.26] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | repeat | 11 | 12.47x | [12.24, 12.72] |
| Rust | tiktoken-rs | cl100k_base | td-code-files | stream | 11 | 8.70x | [8.53, 8.89] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | repeat | 11 | 16.66x | [16.49, 16.85] |
| Rust | tiktoken-rs | cl100k_base | td-synthetic-1MB * | stream | 11 | 14.05x | [13.76, 14.34] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | cold | 11 | 3.10x | [3.08, 3.11] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | repeat | 11 | 7.04x | [6.98, 7.13] |
| Rust | tiktoken-rs | cl100k_base | wiki-ar | stream | 11 | 4.38x | [4.37, 4.40] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | cold | 11 | 1.91x | [1.89, 1.93] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | repeat | 11 | 2.97x | [2.94, 3.00] |
| Rust | tiktoken-rs | cl100k_base | wiki-de | stream | 11 | 2.42x | [2.38, 2.45] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | cold | 11 | 3.15x | [3.12, 3.18] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | repeat | 11 | 6.69x | [6.61, 6.78] |
| Rust | tiktoken-rs | cl100k_base | wiki-en | stream | 11 | 3.90x | [3.84, 3.97] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | cold | 11 | 2.09x | [2.06, 2.12] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | repeat | 11 | 5.36x | [5.28, 5.47] |
| Rust | tiktoken-rs | cl100k_base | wiki-es | stream | 11 | 3.14x | [3.11, 3.18] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | cold | 11 | 2.07x | [2.05, 2.08] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | repeat | 11 | 4.33x | [4.30, 4.38] |
| Rust | tiktoken-rs | cl100k_base | wiki-fr | stream | 11 | 3.13x | [3.11, 3.16] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | cold | 11 | 4.63x | [4.62, 4.65] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | repeat | 11 | 10.66x | [10.58, 10.74] |
| Rust | tiktoken-rs | cl100k_base | wiki-hi | stream | 11 | 6.86x | [6.80, 6.93] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | cold | 11 | 5.71x | [5.63, 5.81] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | repeat | 11 | 12.79x | [12.43, 13.14] |
| Rust | tiktoken-rs | cl100k_base | wiki-ja | stream | 11 | 6.61x | [6.56, 6.67] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | cold | 11 | 3.16x | [3.08, 3.24] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | repeat | 11 | 10.76x | [10.15, 11.42] |
| Rust | tiktoken-rs | cl100k_base | wiki-ko | stream | 11 | 3.68x | [3.53, 3.82] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | cold | 11 | 2.77x | [2.75, 2.80] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | repeat | 11 | 5.85x | [5.79, 5.91] |
| Rust | tiktoken-rs | cl100k_base | wiki-ru | stream | 11 | 3.57x | [3.53, 3.61] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | cold | 11 | 7.20x | [7.11, 7.30] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | repeat | 11 | 14.19x | [14.07, 14.31] |
| Rust | tiktoken-rs | cl100k_base | wiki-th | stream | 11 | 8.61x | [8.50, 8.73] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | cold | 11 | 6.56x | [6.39, 6.75] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | repeat | 11 | 16.32x | [15.94, 16.75] |
| Rust | tiktoken-rs | cl100k_base | wiki-zh | stream | 11 | 7.08x | [6.92, 7.25] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | repeat | 11 | 8.80x | [8.45, 9.09] |
| Rust | tiktoken-rs | o200k_base | book-gosrc | stream | 11 | 3.53x | [3.33, 3.66] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | repeat | 11 | 4.96x | [4.94, 4.99] |
| Rust | tiktoken-rs | o200k_base | book-hongloumeng | stream | 11 | 4.72x | [4.50, 4.85] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | repeat | 11 | 9.11x | [8.82, 9.38] |
| Rust | tiktoken-rs | o200k_base | book-mobydick | stream | 11 | 1.85x | [1.78, 1.93] |
| Rust | tiktoken-rs | o200k_base | chat-512B | cold | 11 | 1.85x | [1.85, 1.87] |
| Rust | tiktoken-rs | o200k_base | chat-512B | repeat | 11 | 2.16x | [2.15, 2.17] |
| Rust | tiktoken-rs | o200k_base | chat-512B | stream | 11 | 1.95x | [1.94, 1.96] |
| Rust | tiktoken-rs | o200k_base | td-code-files | cold | 11 | 5.12x | [5.01, 5.24] |
| Rust | tiktoken-rs | o200k_base | td-code-files | repeat | 11 | 7.89x | [7.70, 8.10] |
| Rust | tiktoken-rs | o200k_base | td-code-files | stream | 11 | 5.44x | [5.32, 5.56] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | repeat | 11 | 9.80x | [9.66, 9.92] |
| Rust | tiktoken-rs | o200k_base | td-synthetic-1MB * | stream | 11 | 7.85x | [7.70, 7.98] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | cold | 11 | 1.78x | [1.76, 1.80] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | repeat | 11 | 3.13x | [3.09, 3.18] |
| Rust | tiktoken-rs | o200k_base | wiki-ar | stream | 11 | 2.03x | [2.00, 2.07] |
| Rust | tiktoken-rs | o200k_base | wiki-de | cold | 11 | 1.39x | [1.38, 1.40] |
| Rust | tiktoken-rs | o200k_base | wiki-de | repeat | 11 | 2.10x | [2.08, 2.12] |
| Rust | tiktoken-rs | o200k_base | wiki-de | stream | 11 | 1.70x | [1.68, 1.71] |
| Rust | tiktoken-rs | o200k_base | wiki-en | cold | 11 | 1.99x | [1.95, 2.05] |
| Rust | tiktoken-rs | o200k_base | wiki-en | repeat | 11 | 4.42x | [4.32, 4.52] |
| Rust | tiktoken-rs | o200k_base | wiki-en | stream | 11 | 2.41x | [2.35, 2.47] |
| Rust | tiktoken-rs | o200k_base | wiki-es | cold | 11 | 1.49x | [1.48, 1.51] |
| Rust | tiktoken-rs | o200k_base | wiki-es | repeat | 11 | 3.33x | [3.27, 3.39] |
| Rust | tiktoken-rs | o200k_base | wiki-es | stream | 11 | 1.94x | [1.91, 1.97] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | cold | 11 | 1.46x | [1.45, 1.47] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | repeat | 11 | 2.70x | [2.66, 2.73] |
| Rust | tiktoken-rs | o200k_base | wiki-fr | stream | 11 | 1.92x | [1.90, 1.94] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | cold | 11 | 1.94x | [1.92, 1.95] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | repeat | 11 | 3.74x | [3.61, 3.89] |
| Rust | tiktoken-rs | o200k_base | wiki-hi | stream | 11 | 2.46x | [2.38, 2.54] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | cold | 11 | 3.90x | [3.87, 3.92] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | repeat | 11 | 9.15x | [8.97, 9.26] |
| Rust | tiktoken-rs | o200k_base | wiki-ja | stream | 11 | 4.73x | [4.71, 4.75] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | cold | 11 | 2.12x | [2.09, 2.15] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | repeat | 11 | 6.07x | [5.75, 6.30] |
| Rust | tiktoken-rs | o200k_base | wiki-ko | stream | 11 | 2.15x | [2.12, 2.20] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | cold | 11 | 2.00x | [1.98, 2.01] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | repeat | 11 | 3.94x | [3.91, 3.97] |
| Rust | tiktoken-rs | o200k_base | wiki-ru | stream | 11 | 2.32x | [2.30, 2.33] |
| Rust | tiktoken-rs | o200k_base | wiki-th | cold | 11 | 2.99x | [2.96, 3.02] |
| Rust | tiktoken-rs | o200k_base | wiki-th | repeat | 11 | 5.65x | [5.56, 5.77] |
| Rust | tiktoken-rs | o200k_base | wiki-th | stream | 11 | 3.65x | [3.60, 3.72] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | cold | 11 | 4.09x | [4.03, 4.16] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | repeat | 11 | 10.09x | [9.91, 10.31] |
| Rust | tiktoken-rs | o200k_base | wiki-zh | stream | 11 | 4.58x | [4.47, 4.70] |
| Rust | bpe-openai | cl100k_base | book-gosrc | repeat | 11 | 5.65x | [5.44, 5.86] |
| Rust | bpe-openai | cl100k_base | book-gosrc | stream | 11 | 1.78x | [1.70, 1.87] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | repeat | 11 | 5.28x | [5.23, 5.33] |
| Rust | bpe-openai | cl100k_base | book-hongloumeng | stream | 11 | 4.98x | [4.85, 5.11] |
| Rust | bpe-openai | cl100k_base | book-mobydick | repeat | 11 | 7.70x | [7.29, 8.06] |
| Rust | bpe-openai | cl100k_base | book-mobydick | stream | 11 | 1.76x | [1.67, 1.83] |
| Rust | bpe-openai | cl100k_base | chat-512B | cold | 11 | 1.39x | [1.39, 1.40] |
| Rust | bpe-openai | cl100k_base | chat-512B | repeat | 11 | 1.72x | [1.71, 1.74] |
| Rust | bpe-openai | cl100k_base | chat-512B | stream | 11 | 1.50x | [1.44, 1.53] |
| Rust | bpe-openai | cl100k_base | td-code-files | cold | 11 | 2.84x | [2.82, 2.86] |
| Rust | bpe-openai | cl100k_base | td-code-files | repeat | 11 | 4.32x | [4.28, 4.36] |
| Rust | bpe-openai | cl100k_base | td-code-files | stream | 11 | 3.03x | [2.99, 3.06] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | repeat | 11 | 8.03x | [7.96, 8.11] |
| Rust | bpe-openai | cl100k_base | td-synthetic-1MB * | stream | 11 | 6.76x | [6.61, 6.92] |
| Rust | bpe-openai | cl100k_base | wiki-ar | cold | 11 | 1.34x | [1.33, 1.35] |
| Rust | bpe-openai | cl100k_base | wiki-ar | repeat | 11 | 3.06x | [3.03, 3.09] |
| Rust | bpe-openai | cl100k_base | wiki-ar | stream | 11 | 1.92x | [1.91, 1.92] |
| Rust | bpe-openai | cl100k_base | wiki-de | cold | 11 | 1.25x | [1.24, 1.27] |
| Rust | bpe-openai | cl100k_base | wiki-de | repeat | 11 | 1.95x | [1.94, 1.97] |
| Rust | bpe-openai | cl100k_base | wiki-de | stream | 11 | 1.58x | [1.56, 1.60] |
| Rust | bpe-openai | cl100k_base | wiki-en | cold | 11 | 1.75x | [1.74, 1.77] |
| Rust | bpe-openai | cl100k_base | wiki-en | repeat | 11 | 3.68x | [3.63, 3.73] |
| Rust | bpe-openai | cl100k_base | wiki-en | stream | 11 | 2.18x | [2.16, 2.20] |
| Rust | bpe-openai | cl100k_base | wiki-es | cold | 11 | 1.21x | [1.20, 1.22] |
| Rust | bpe-openai | cl100k_base | wiki-es | repeat | 11 | 3.09x | [3.04, 3.13] |
| Rust | bpe-openai | cl100k_base | wiki-es | stream | 11 | 1.82x | [1.81, 1.84] |
| Rust | bpe-openai | cl100k_base | wiki-fr | cold | 11 | 1.24x | [1.23, 1.25] |
| Rust | bpe-openai | cl100k_base | wiki-fr | repeat | 11 | 2.60x | [2.58, 2.62] |
| Rust | bpe-openai | cl100k_base | wiki-fr | stream | 11 | 1.87x | [1.86, 1.89] |
| Rust | bpe-openai | cl100k_base | wiki-hi | cold | 11 | 1.76x | [1.76, 1.76] |
| Rust | bpe-openai | cl100k_base | wiki-hi | repeat | 11 | 4.06x | [4.03, 4.08] |
| Rust | bpe-openai | cl100k_base | wiki-hi | stream | 11 | 2.62x | [2.59, 2.64] |
| Rust | bpe-openai | cl100k_base | wiki-ja | cold | 11 | 2.66x | [2.63, 2.70] |
| Rust | bpe-openai | cl100k_base | wiki-ja | repeat | 11 | 5.95x | [5.79, 6.09] |
| Rust | bpe-openai | cl100k_base | wiki-ja | stream | 11 | 3.09x | [3.07, 3.11] |
| Rust | bpe-openai | cl100k_base | wiki-ko | cold | 11 | 1.47x | [1.43, 1.51] |
| Rust | bpe-openai | cl100k_base | wiki-ko | repeat | 11 | 4.96x | [4.68, 5.24] |
| Rust | bpe-openai | cl100k_base | wiki-ko | stream | 11 | 1.71x | [1.64, 1.78] |
| Rust | bpe-openai | cl100k_base | wiki-ru | cold | 11 | 1.25x | [1.24, 1.26] |
| Rust | bpe-openai | cl100k_base | wiki-ru | repeat | 11 | 2.64x | [2.61, 2.66] |
| Rust | bpe-openai | cl100k_base | wiki-ru | stream | 11 | 1.61x | [1.59, 1.62] |
| Rust | bpe-openai | cl100k_base | wiki-th | cold | 11 | 2.86x | [2.83, 2.89] |
| Rust | bpe-openai | cl100k_base | wiki-th | repeat | 11 | 5.59x | [5.55, 5.63] |
| Rust | bpe-openai | cl100k_base | wiki-th | stream | 11 | 3.39x | [3.35, 3.43] |
| Rust | bpe-openai | cl100k_base | wiki-zh | cold | 11 | 3.36x | [3.29, 3.43] |
| Rust | bpe-openai | cl100k_base | wiki-zh | repeat | 11 | 8.36x | [8.15, 8.58] |
| Rust | bpe-openai | cl100k_base | wiki-zh | stream | 11 | 3.63x | [3.58, 3.69] |
| Rust | bpe-openai | o200k_base | book-gosrc | repeat | 11 | 5.68x | [5.45, 5.87] |
| Rust | bpe-openai | o200k_base | book-gosrc | stream | 11 | 2.31x | [2.21, 2.39] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | repeat | 11 | 2.74x | [2.72, 2.75] |
| Rust | bpe-openai | o200k_base | book-hongloumeng | stream | 11 | 2.61x | [2.52, 2.67] |
| Rust | bpe-openai | o200k_base | book-mobydick | repeat | 11 | 8.07x | [7.79, 8.34] |
| Rust | bpe-openai | o200k_base | book-mobydick | stream | 11 | 1.70x | [1.64, 1.77] |
| Rust | bpe-openai | o200k_base | chat-512B | cold | 11 | 1.31x | [1.30, 1.31] |
| Rust | bpe-openai | o200k_base | chat-512B | repeat | 11 | 1.53x | [1.52, 1.53] |
| Rust | bpe-openai | o200k_base | chat-512B | stream | 11 | 1.38x | [1.37, 1.39] |
| Rust | bpe-openai | o200k_base | td-code-files | cold | 11 | 3.00x | [2.95, 3.05] |
| Rust | bpe-openai | o200k_base | td-code-files | repeat | 11 | 4.60x | [4.52, 4.67] |
| Rust | bpe-openai | o200k_base | td-code-files | stream | 11 | 3.19x | [3.11, 3.26] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | repeat | 11 | 8.93x | [8.82, 9.04] |
| Rust | bpe-openai | o200k_base | td-synthetic-1MB * | stream | 11 | 7.20x | [7.07, 7.32] |
| Rust | bpe-openai | o200k_base | wiki-ar | cold | 11 | 1.12x | [1.11, 1.13] |
| Rust | bpe-openai | o200k_base | wiki-ar | repeat | 11 | 1.97x | [1.95, 2.00] |
| Rust | bpe-openai | o200k_base | wiki-ar | stream | 11 | 1.28x | [1.26, 1.30] |
| Rust | bpe-openai | o200k_base | wiki-de | cold | 11 | 1.17x | [1.16, 1.17] |
| Rust | bpe-openai | o200k_base | wiki-de | repeat | 11 | 1.75x | [1.74, 1.76] |
| Rust | bpe-openai | o200k_base | wiki-de | stream | 11 | 1.42x | [1.41, 1.43] |
| Rust | bpe-openai | o200k_base | wiki-en | cold | 11 | 1.77x | [1.73, 1.82] |
| Rust | bpe-openai | o200k_base | wiki-en | repeat | 11 | 3.89x | [3.78, 4.00] |
| Rust | bpe-openai | o200k_base | wiki-en | stream | 11 | 2.15x | [2.11, 2.19] |
| Rust | bpe-openai | o200k_base | wiki-es | cold | 11 | 1.30x | [1.28, 1.32] |
| Rust | bpe-openai | o200k_base | wiki-es | repeat | 11 | 2.87x | [2.81, 2.92] |
| Rust | bpe-openai | o200k_base | wiki-es | stream | 11 | 1.67x | [1.64, 1.70] |
| Rust | bpe-openai | o200k_base | wiki-fr | cold | 11 | 1.27x | [1.26, 1.28] |
| Rust | bpe-openai | o200k_base | wiki-fr | repeat | 11 | 2.34x | [2.31, 2.37] |
| Rust | bpe-openai | o200k_base | wiki-fr | stream | 11 | 1.67x | [1.66, 1.68] |
| Rust | bpe-openai | o200k_base | wiki-hi | cold | 11 | 1.07x | [1.06, 1.07] |
| Rust | bpe-openai | o200k_base | wiki-hi | repeat | 11 | 2.05x | [1.98, 2.12] |
| Rust | bpe-openai | o200k_base | wiki-hi | stream | 11 | 1.35x | [1.32, 1.39] |
| Rust | bpe-openai | o200k_base | wiki-ja | cold | 11 | 1.88x | [1.87, 1.89] |
| Rust | bpe-openai | o200k_base | wiki-ja | repeat | 11 | 4.43x | [4.33, 4.48] |
| Rust | bpe-openai | o200k_base | wiki-ja | stream | 11 | 2.29x | [2.28, 2.29] |
| Rust | bpe-openai | o200k_base | wiki-ko | cold | 11 | 1.10x | [1.09, 1.12] |
| Rust | bpe-openai | o200k_base | wiki-ko | repeat | 11 | 3.16x | [2.99, 3.28] |
| Rust | bpe-openai | o200k_base | wiki-ko | stream | 11 | 1.13x | [1.11, 1.14] |
| Rust | bpe-openai | o200k_base | wiki-ru | cold | 11 | 1.13x | [1.12, 1.13] |
| Rust | bpe-openai | o200k_base | wiki-ru | repeat | 11 | 2.22x | [2.21, 2.24] |
| Rust | bpe-openai | o200k_base | wiki-ru | stream | 11 | 1.30x | [1.30, 1.31] |
| Rust | bpe-openai | o200k_base | wiki-th | cold | 11 | 1.05x | [1.04, 1.06] |
| Rust | bpe-openai | o200k_base | wiki-th | repeat | 11 | 1.98x | [1.95, 2.01] |
| Rust | bpe-openai | o200k_base | wiki-th | stream | 11 | 1.28x | [1.26, 1.30] |
| Rust | bpe-openai | o200k_base | wiki-zh | cold | 11 | 2.13x | [2.10, 2.15] |
| Rust | bpe-openai | o200k_base | wiki-zh | repeat | 11 | 5.23x | [5.14, 5.30] |
| Rust | bpe-openai | o200k_base | wiki-zh | stream | 11 | 2.38x | [2.35, 2.42] |
| Python | tiktoken | cl100k_base | book-gosrc | stream | 11 | 4.45x | [4.30, 4.58] |
| Python | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 5.18x | [5.13, 5.23] |
| Python | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.08x | [2.96, 3.18] |
| Python | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.46x | [2.44, 2.47] |
| Python | tiktoken | cl100k_base | chat-512B | stream | 11 | 2.76x | [2.74, 2.79] |
| Python | tiktoken | cl100k_base | td-code-files | cold | 11 | 7.21x | [7.01, 7.41] |
| Python | tiktoken | cl100k_base | td-code-files | stream | 11 | 7.49x | [7.25, 7.74] |
| Python | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 9.54x | [9.40, 9.69] |
| Python | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.94x | [2.91, 2.98] |
| Python | tiktoken | cl100k_base | wiki-ar | stream | 11 | 4.05x | [3.98, 4.13] |
| Python | tiktoken | cl100k_base | wiki-de | cold | 11 | 1.92x | [1.89, 1.94] |
| Python | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.45x | [2.40, 2.49] |
| Python | tiktoken | cl100k_base | wiki-en | cold | 11 | 3.11x | [3.02, 3.21] |
| Python | tiktoken | cl100k_base | wiki-en | stream | 11 | 3.67x | [3.54, 3.81] |
| Python | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.08x | [2.03, 2.13] |
| Python | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.00x | [2.92, 3.09] |
| Python | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.03x | [1.99, 2.07] |
| Python | tiktoken | cl100k_base | wiki-fr | stream | 11 | 2.93x | [2.85, 3.01] |
| Python | tiktoken | cl100k_base | wiki-hi | cold | 11 | 3.80x | [3.77, 3.82] |
| Python | tiktoken | cl100k_base | wiki-hi | stream | 11 | 5.00x | [4.95, 5.06] |
| Python | tiktoken | cl100k_base | wiki-ja | cold | 11 | 4.42x | [4.31, 4.52] |
| Python | tiktoken | cl100k_base | wiki-ja | stream | 11 | 4.93x | [4.81, 5.07] |
| Python | tiktoken | cl100k_base | wiki-ko | cold | 11 | 2.69x | [2.62, 2.76] |
| Python | tiktoken | cl100k_base | wiki-ko | stream | 11 | 3.04x | [2.93, 3.15] |
| Python | tiktoken | cl100k_base | wiki-ru | cold | 11 | 2.55x | [2.51, 2.59] |
| Python | tiktoken | cl100k_base | wiki-ru | stream | 11 | 3.17x | [3.12, 3.24] |
| Python | tiktoken | cl100k_base | wiki-th | cold | 11 | 5.67x | [5.59, 5.75] |
| Python | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.39x | [6.27, 6.52] |
| Python | tiktoken | cl100k_base | wiki-zh | cold | 11 | 4.55x | [4.45, 4.65] |
| Python | tiktoken | cl100k_base | wiki-zh | stream | 11 | 4.76x | [4.61, 4.91] |
| Python | tiktoken | o200k_base | book-gosrc | stream | 11 | 3.19x | [3.12, 3.26] |
| Python | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 4.58x | [4.48, 4.68] |
| Python | tiktoken | o200k_base | book-mobydick | stream | 11 | 1.98x | [1.87, 2.07] |
| Python | tiktoken | o200k_base | chat-512B | cold | 11 | 1.89x | [1.86, 1.93] |
| Python | tiktoken | o200k_base | chat-512B | stream | 11 | 1.98x | [1.95, 2.01] |
| Python | tiktoken | o200k_base | td-code-files | cold | 11 | 4.41x | [4.29, 4.53] |
| Python | tiktoken | o200k_base | td-code-files | stream | 11 | 4.66x | [4.50, 4.82] |
| Python | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 5.48x | [5.41, 5.54] |
| Python | tiktoken | o200k_base | wiki-ar | cold | 11 | 1.84x | [1.79, 1.89] |
| Python | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.09x | [2.02, 2.17] |
| Python | tiktoken | o200k_base | wiki-de | cold | 11 | 1.50x | [1.47, 1.53] |
| Python | tiktoken | o200k_base | wiki-de | stream | 11 | 1.80x | [1.76, 1.85] |
| Python | tiktoken | o200k_base | wiki-en | cold | 11 | 2.06x | [1.97, 2.15] |
| Python | tiktoken | o200k_base | wiki-en | stream | 11 | 2.44x | [2.32, 2.56] |
| Python | tiktoken | o200k_base | wiki-es | cold | 11 | 1.61x | [1.55, 1.68] |
| Python | tiktoken | o200k_base | wiki-es | stream | 11 | 2.07x | [1.98, 2.16] |
| Python | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.57x | [1.52, 1.62] |
| Python | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.04x | [1.96, 2.13] |
| Python | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.04x | [1.98, 2.09] |
| Python | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.45x | [2.36, 2.54] |
| Python | tiktoken | o200k_base | wiki-ja | cold | 11 | 3.75x | [3.65, 3.85] |
| Python | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.45x | [4.30, 4.60] |
| Python | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.23x | [2.13, 2.33] |
| Python | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.29x | [2.17, 2.42] |
| Python | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.03x | [1.98, 2.08] |
| Python | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.38x | [2.33, 2.44] |
| Python | tiktoken | o200k_base | wiki-th | cold | 11 | 3.03x | [2.98, 3.09] |
| Python | tiktoken | o200k_base | wiki-th | stream | 11 | 3.61x | [3.54, 3.69] |
| Python | tiktoken | o200k_base | wiki-zh | cold | 11 | 3.91x | [3.79, 4.05] |
| Python | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.20x | [4.05, 4.36] |
| Python, encode() | tiktoken | cl100k_base | book-gosrc | stream | 11 | 4.44x | [4.34, 4.54] |
| Python, encode() | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 5.05x | [4.99, 5.10] |
| Python, encode() | tiktoken | cl100k_base | book-mobydick | stream | 11 | 3.07x | [2.95, 3.18] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | cold | 11 | 2.51x | [2.49, 2.53] |
| Python, encode() | tiktoken | cl100k_base | chat-512B | stream | 11 | 2.80x | [2.77, 2.83] |
| Python, encode() | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 9.64x | [9.48, 9.82] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | cold | 11 | 2.93x | [2.89, 2.97] |
| Python, encode() | tiktoken | cl100k_base | wiki-ar | stream | 11 | 4.04x | [3.97, 4.12] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | cold | 11 | 1.93x | [1.90, 1.96] |
| Python, encode() | tiktoken | cl100k_base | wiki-de | stream | 11 | 2.46x | [2.41, 2.51] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | cold | 11 | 3.16x | [3.07, 3.26] |
| Python, encode() | tiktoken | cl100k_base | wiki-en | stream | 11 | 3.76x | [3.61, 3.90] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | cold | 11 | 2.10x | [2.06, 2.15] |
| Python, encode() | tiktoken | cl100k_base | wiki-es | stream | 11 | 3.02x | [2.93, 3.11] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | cold | 11 | 2.05x | [2.02, 2.09] |
| Python, encode() | tiktoken | cl100k_base | wiki-fr | stream | 11 | 2.96x | [2.88, 3.04] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | cold | 11 | 3.78x | [3.76, 3.81] |
| Python, encode() | tiktoken | cl100k_base | wiki-hi | stream | 11 | 4.95x | [4.89, 5.01] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | cold | 11 | 4.40x | [4.30, 4.50] |
| Python, encode() | tiktoken | cl100k_base | wiki-ja | stream | 11 | 4.87x | [4.74, 5.00] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | cold | 11 | 2.69x | [2.63, 2.76] |
| Python, encode() | tiktoken | cl100k_base | wiki-ko | stream | 11 | 3.03x | [2.91, 3.15] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | cold | 11 | 2.56x | [2.52, 2.59] |
| Python, encode() | tiktoken | cl100k_base | wiki-ru | stream | 11 | 3.17x | [3.11, 3.24] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | cold | 11 | 5.65x | [5.56, 5.74] |
| Python, encode() | tiktoken | cl100k_base | wiki-th | stream | 11 | 6.29x | [6.15, 6.42] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | cold | 11 | 4.52x | [4.42, 4.62] |
| Python, encode() | tiktoken | cl100k_base | wiki-zh | stream | 11 | 4.69x | [4.55, 4.83] |
| Python, encode() | tiktoken | o200k_base | book-gosrc | stream | 11 | 3.25x | [3.17, 3.31] |
| Python, encode() | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 4.49x | [4.38, 4.61] |
| Python, encode() | tiktoken | o200k_base | book-mobydick | stream | 11 | 2.03x | [1.97, 2.10] |
| Python, encode() | tiktoken | o200k_base | chat-512B | cold | 11 | 1.96x | [1.93, 1.98] |
| Python, encode() | tiktoken | o200k_base | chat-512B | stream | 11 | 2.05x | [2.01, 2.09] |
| Python, encode() | tiktoken | o200k_base | td-code-files | cold | 11 | 4.49x | [4.38, 4.61] |
| Python, encode() | tiktoken | o200k_base | td-code-files | stream | 11 | 4.66x | [4.51, 4.82] |
| Python, encode() | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 5.57x | [5.50, 5.65] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | cold | 11 | 1.86x | [1.81, 1.91] |
| Python, encode() | tiktoken | o200k_base | wiki-ar | stream | 11 | 2.11x | [2.04, 2.19] |
| Python, encode() | tiktoken | o200k_base | wiki-de | cold | 11 | 1.53x | [1.50, 1.55] |
| Python, encode() | tiktoken | o200k_base | wiki-de | stream | 11 | 1.84x | [1.79, 1.89] |
| Python, encode() | tiktoken | o200k_base | wiki-en | cold | 11 | 2.13x | [2.05, 2.23] |
| Python, encode() | tiktoken | o200k_base | wiki-en | stream | 11 | 2.51x | [2.39, 2.64] |
| Python, encode() | tiktoken | o200k_base | wiki-es | cold | 11 | 1.65x | [1.59, 1.71] |
| Python, encode() | tiktoken | o200k_base | wiki-es | stream | 11 | 2.10x | [2.01, 2.19] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | cold | 11 | 1.61x | [1.56, 1.67] |
| Python, encode() | tiktoken | o200k_base | wiki-fr | stream | 11 | 2.08x | [2.00, 2.17] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | cold | 11 | 2.06x | [2.01, 2.11] |
| Python, encode() | tiktoken | o200k_base | wiki-hi | stream | 11 | 2.46x | [2.36, 2.55] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | cold | 11 | 3.80x | [3.69, 3.92] |
| Python, encode() | tiktoken | o200k_base | wiki-ja | stream | 11 | 4.43x | [4.28, 4.59] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | cold | 11 | 2.23x | [2.13, 2.33] |
| Python, encode() | tiktoken | o200k_base | wiki-ko | stream | 11 | 2.30x | [2.17, 2.43] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | cold | 11 | 2.04x | [1.98, 2.09] |
| Python, encode() | tiktoken | o200k_base | wiki-ru | stream | 11 | 2.41x | [2.35, 2.48] |
| Python, encode() | tiktoken | o200k_base | wiki-th | cold | 11 | 3.01x | [2.97, 3.06] |
| Python, encode() | tiktoken | o200k_base | wiki-th | stream | 11 | 3.62x | [3.54, 3.70] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | cold | 11 | 3.92x | [3.80, 4.06] |
| Python, encode() | tiktoken | o200k_base | wiki-zh | stream | 11 | 4.23x | [4.06, 4.41] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 20.21x | [19.66, 20.89] |
| JavaScript | tiktoken | cl100k_base | book-gosrc | stream | 11 | 8.27x | [7.23, 9.01] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 13.14x | [13.04, 13.26] |
| JavaScript | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 11.48x | [9.81, 12.89] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 17.83x | [17.08, 18.79] |
| JavaScript | tiktoken | cl100k_base | book-mobydick | stream | 11 | 6.40x | [6.28, 6.52] |
| JavaScript | tiktoken | cl100k_base | chat-512B | cold | 11 | 4.88x | [4.83, 4.95] |
| JavaScript | tiktoken | cl100k_base | chat-512B | repeat | 11 | 5.60x | [5.46, 5.72] |
| JavaScript | tiktoken | cl100k_base | chat-512B | stream | 11 | 5.17x | [4.93, 5.36] |
| JavaScript | tiktoken | cl100k_base | td-code-files | cold | 11 | 16.82x | [16.68, 16.94] |
| JavaScript | tiktoken | cl100k_base | td-code-files | repeat | 11 | 19.86x | [18.76, 20.93] |
| JavaScript | tiktoken | cl100k_base | td-code-files | stream | 11 | 17.05x | [16.38, 17.54] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 17.29x | [15.26, 19.93] |
| JavaScript | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 21.61x | [19.83, 23.16] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | cold | 11 | 5.12x | [5.07, 5.17] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 10.89x | [10.79, 10.97] |
| JavaScript | tiktoken | cl100k_base | wiki-ar | stream | 11 | 7.40x | [7.33, 7.47] |
| JavaScript | tiktoken | cl100k_base | wiki-de | cold | 11 | 4.30x | [4.28, 4.32] |
| JavaScript | tiktoken | cl100k_base | wiki-de | repeat | 11 | 6.80x | [6.76, 6.84] |
| JavaScript | tiktoken | cl100k_base | wiki-de | stream | 11 | 5.54x | [5.48, 5.58] |
| JavaScript | tiktoken | cl100k_base | wiki-en | cold | 11 | 8.07x | [8.03, 8.11] |
| JavaScript | tiktoken | cl100k_base | wiki-en | repeat | 11 | 14.96x | [14.72, 15.18] |
| JavaScript | tiktoken | cl100k_base | wiki-en | stream | 11 | 9.66x | [9.58, 9.73] |
| JavaScript | tiktoken | cl100k_base | wiki-es | cold | 11 | 4.83x | [4.80, 4.86] |
| JavaScript | tiktoken | cl100k_base | wiki-es | repeat | 11 | 11.23x | [11.17, 11.29] |
| JavaScript | tiktoken | cl100k_base | wiki-es | stream | 11 | 7.11x | [7.06, 7.16] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | cold | 11 | 4.57x | [4.47, 4.67] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 8.79x | [8.58, 8.98] |
| JavaScript | tiktoken | cl100k_base | wiki-fr | stream | 11 | 6.77x | [6.65, 6.88] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | cold | 11 | 7.71x | [7.64, 7.77] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 16.23x | [16.11, 16.34] |
| JavaScript | tiktoken | cl100k_base | wiki-hi | stream | 11 | 11.10x | [11.03, 11.16] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | cold | 11 | 9.76x | [9.67, 9.84] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 19.12x | [18.65, 19.46] |
| JavaScript | tiktoken | cl100k_base | wiki-ja | stream | 11 | 11.26x | [11.20, 11.32] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | cold | 11 | 5.02x | [4.96, 5.09] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 13.84x | [13.25, 14.17] |
| JavaScript | tiktoken | cl100k_base | wiki-ko | stream | 11 | 5.75x | [5.67, 5.83] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | cold | 11 | 4.62x | [4.59, 4.65] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 8.97x | [8.90, 9.03] |
| JavaScript | tiktoken | cl100k_base | wiki-ru | stream | 11 | 5.78x | [5.75, 5.81] |
| JavaScript | tiktoken | cl100k_base | wiki-th | cold | 11 | 10.97x | [10.86, 11.08] |
| JavaScript | tiktoken | cl100k_base | wiki-th | repeat | 11 | 18.81x | [18.70, 18.92] |
| JavaScript | tiktoken | cl100k_base | wiki-th | stream | 11 | 12.60x | [12.44, 12.75] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | cold | 11 | 10.29x | [10.16, 10.41] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 21.43x | [20.96, 21.73] |
| JavaScript | tiktoken | cl100k_base | wiki-zh | stream | 11 | 11.00x | [10.91, 11.09] |
| JavaScript | tiktoken | o200k_base | book-gosrc | repeat | 11 | 12.65x | [10.17, 15.88] |
| JavaScript | tiktoken | o200k_base | book-gosrc | stream | 11 | 10.33x | [9.29, 11.20] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 8.57x | [8.52, 8.62] |
| JavaScript | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 7.29x | [6.85, 7.76] |
| JavaScript | tiktoken | o200k_base | book-mobydick | repeat | 11 | 13.51x | [11.65, 15.99] |
| JavaScript | tiktoken | o200k_base | book-mobydick | stream | 11 | 5.91x | [5.58, 6.24] |
| JavaScript | tiktoken | o200k_base | chat-512B | cold | 11 | 4.44x | [4.33, 4.54] |
| JavaScript | tiktoken | o200k_base | chat-512B | repeat | 11 | 4.88x | [4.73, 5.04] |
| JavaScript | tiktoken | o200k_base | chat-512B | stream | 11 | 4.64x | [4.52, 4.75] |
| JavaScript | tiktoken | o200k_base | td-code-files | cold | 11 | 15.96x | [15.29, 16.38] |
| JavaScript | tiktoken | o200k_base | td-code-files | repeat | 11 | 19.65x | [18.20, 21.05] |
| JavaScript | tiktoken | o200k_base | td-code-files | stream | 11 | 15.50x | [14.47, 16.49] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 15.88x | [14.02, 18.54] |
| JavaScript | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 21.41x | [18.47, 23.97] |
| JavaScript | tiktoken | o200k_base | wiki-ar | cold | 11 | 4.41x | [4.37, 4.44] |
| JavaScript | tiktoken | o200k_base | wiki-ar | repeat | 11 | 7.08x | [7.00, 7.15] |
| JavaScript | tiktoken | o200k_base | wiki-ar | stream | 11 | 4.82x | [4.72, 4.92] |
| JavaScript | tiktoken | o200k_base | wiki-de | cold | 11 | 4.29x | [4.25, 4.32] |
| JavaScript | tiktoken | o200k_base | wiki-de | repeat | 11 | 6.24x | [6.21, 6.27] |
| JavaScript | tiktoken | o200k_base | wiki-de | stream | 11 | 5.17x | [5.13, 5.20] |
| JavaScript | tiktoken | o200k_base | wiki-en | cold | 11 | 7.73x | [7.70, 7.76] |
| JavaScript | tiktoken | o200k_base | wiki-en | repeat | 11 | 14.86x | [14.63, 15.06] |
| JavaScript | tiktoken | o200k_base | wiki-en | stream | 11 | 9.13x | [9.04, 9.22] |
| JavaScript | tiktoken | o200k_base | wiki-es | cold | 11 | 5.28x | [5.25, 5.31] |
| JavaScript | tiktoken | o200k_base | wiki-es | repeat | 11 | 10.90x | [10.83, 10.97] |
| JavaScript | tiktoken | o200k_base | wiki-es | stream | 11 | 6.82x | [6.79, 6.85] |
| JavaScript | tiktoken | o200k_base | wiki-fr | cold | 11 | 4.92x | [4.84, 5.00] |
| JavaScript | tiktoken | o200k_base | wiki-fr | repeat | 11 | 8.71x | [8.52, 8.87] |
| JavaScript | tiktoken | o200k_base | wiki-fr | stream | 11 | 6.68x | [6.65, 6.71] |
| JavaScript | tiktoken | o200k_base | wiki-hi | cold | 11 | 4.37x | [4.31, 4.42] |
| JavaScript | tiktoken | o200k_base | wiki-hi | repeat | 11 | 7.53x | [7.45, 7.61] |
| JavaScript | tiktoken | o200k_base | wiki-hi | stream | 11 | 5.15x | [5.10, 5.20] |
| JavaScript | tiktoken | o200k_base | wiki-ja | cold | 11 | 7.98x | [7.91, 8.04] |
| JavaScript | tiktoken | o200k_base | wiki-ja | repeat | 11 | 16.65x | [16.41, 16.82] |
| JavaScript | tiktoken | o200k_base | wiki-ja | stream | 11 | 9.37x | [9.23, 9.50] |
| JavaScript | tiktoken | o200k_base | wiki-ko | cold | 11 | 4.89x | [4.85, 4.93] |
| JavaScript | tiktoken | o200k_base | wiki-ko | repeat | 11 | 12.45x | [11.79, 12.93] |
| JavaScript | tiktoken | o200k_base | wiki-ko | stream | 11 | 4.97x | [4.92, 5.03] |
| JavaScript | tiktoken | o200k_base | wiki-ru | cold | 11 | 4.35x | [4.31, 4.38] |
| JavaScript | tiktoken | o200k_base | wiki-ru | repeat | 11 | 8.10x | [8.06, 8.14] |
| JavaScript | tiktoken | o200k_base | wiki-ru | stream | 11 | 5.09x | [5.06, 5.13] |
| JavaScript | tiktoken | o200k_base | wiki-th | cold | 11 | 8.17x | [8.14, 8.19] |
| JavaScript | tiktoken | o200k_base | wiki-th | repeat | 11 | 14.28x | [14.18, 14.34] |
| JavaScript | tiktoken | o200k_base | wiki-th | stream | 11 | 9.62x | [9.57, 9.67] |
| JavaScript | tiktoken | o200k_base | wiki-zh | cold | 11 | 7.96x | [7.81, 8.09] |
| JavaScript | tiktoken | o200k_base | wiki-zh | repeat | 11 | 17.03x | [16.76, 17.19] |
| JavaScript | tiktoken | o200k_base | wiki-zh | stream | 11 | 8.58x | [8.52, 8.65] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 5.76x | [5.58, 5.98] |
| JavaScript | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 4.20x | [3.79, 4.48] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.13x | [3.08, 3.19] |
| JavaScript | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 41.26x | [35.38, 46.30] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 5.73x | [5.53, 5.97] |
| JavaScript | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 4.86x | [4.44, 5.40] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 13.05x | [12.85, 13.25] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 11.67x | [11.41, 11.94] |
| JavaScript | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 9.57x | [9.00, 10.00] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 3.71x | [3.60, 3.83] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 3.28x | [2.94, 3.80] |
| JavaScript | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 3.48x | [3.32, 3.63] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 3.89x | [2.97, 5.55] |
| JavaScript | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 4.63x | [4.12, 5.11] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 15.08x | [14.64, 15.61] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 2.55x | [2.52, 2.58] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 10.95x | [10.65, 11.30] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 6.19x | [6.05, 6.36] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 1.85x | [1.83, 1.87] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 6.30x | [6.14, 6.50] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 4.81x | [4.65, 4.97] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 5.09x | [4.99, 5.20] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 5.51x | [5.32, 5.72] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 6.35x | [6.17, 6.53] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 2.95x | [2.90, 2.99] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 6.13x | [5.96, 6.38] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 7.01x | [6.78, 7.25] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 2.60x | [2.55, 2.65] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 6.10x | [5.90, 6.33] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 9.68x | [9.05, 10.45] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 4.07x | [4.02, 4.11] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 4.80x | [4.73, 4.87] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 33.65x | [32.82, 34.57] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.29x | [3.21, 3.35] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 37.74x | [36.84, 38.79] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 13.88x | [13.47, 14.45] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.12x | [2.97, 3.22] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 12.14x | [11.92, 12.37] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 14.32x | [14.04, 14.57] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 1.88x | [1.87, 1.90] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 11.48x | [11.36, 11.60] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 33.51x | [32.36, 34.81] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 4.45x | [4.42, 4.49] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 28.11x | [27.58, 28.76] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 39.91x | [39.40, 40.37] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 4.36x | [4.25, 4.45] |
| JavaScript | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 41.47x | [40.63, 42.35] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 4.73x | [3.74, 5.93] |
| JavaScript | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 5.13x | [4.50, 5.67] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.08x | [2.06, 2.09] |
| JavaScript | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 32.45x | [30.58, 34.61] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 5.01x | [4.34, 5.98] |
| JavaScript | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 4.62x | [4.35, 4.89] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 11.79x | [11.47, 12.17] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.17x | [1.15, 1.20] |
| JavaScript | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 10.49x | [10.28, 10.69] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 3.74x | [3.56, 3.90] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 3.26x | [3.00, 3.53] |
| JavaScript | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 3.37x | [3.10, 3.65] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 3.01x | [2.56, 3.63] |
| JavaScript | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 4.63x | [3.78, 5.42] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 13.45x | [13.10, 13.79] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.09x | [2.06, 2.12] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 10.01x | [9.67, 10.42] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 6.31x | [6.06, 6.58] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 2.01x | [1.98, 2.03] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 6.52x | [6.27, 6.80] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 4.81x | [4.72, 4.89] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 5.37x | [5.26, 5.48] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 6.16x | [5.96, 6.35] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 5.93x | [5.70, 6.15] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 3.50x | [3.46, 3.55] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 6.16x | [6.04, 6.30] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 6.76x | [6.38, 7.17] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 2.97x | [2.94, 3.00] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 6.15x | [6.06, 6.24] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 11.89x | [11.59, 12.20] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 1.93x | [1.91, 1.94] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 9.26x | [9.15, 9.39] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 29.54x | [28.98, 30.14] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.04x | [2.99, 3.09] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 34.87x | [34.15, 35.67] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 14.72x | [14.10, 15.64] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.27x | [3.11, 3.37] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 11.95x | [11.73, 12.25] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 14.91x | [14.58, 15.26] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.07x | [2.06, 2.09] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 11.63x | [11.53, 11.74] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 21.65x | [21.21, 22.20] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.09x | [1.08, 1.09] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 27.03x | [26.84, 27.22] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 32.76x | [32.02, 33.40] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 3.43x | [3.37, 3.47] |
| JavaScript | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 35.89x | [34.92, 36.93] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | repeat | 11 | 54.64x | [53.07, 56.41] |
| JavaScript | js-tiktoken | cl100k_base | book-gosrc | stream | 11 | 22.71x | [19.60, 24.76] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 170.75x | [169.58, 171.95] |
| JavaScript | js-tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 149.91x | [130.32, 166.78] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | repeat | 11 | 46.00x | [43.70, 48.73] |
| JavaScript | js-tiktoken | cl100k_base | book-mobydick | stream | 11 | 17.13x | [16.71, 17.54] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | cold | 11 | 42.76x | [42.57, 42.98] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | repeat | 11 | 51.08x | [50.17, 51.97] |
| JavaScript | js-tiktoken | cl100k_base | chat-512B | stream | 11 | 45.02x | [42.45, 46.84] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | cold | 11 | 27.28x | [27.11, 27.45] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | repeat | 11 | 32.03x | [30.09, 33.98] |
| JavaScript | js-tiktoken | cl100k_base | td-code-files | stream | 11 | 27.63x | [26.50, 28.51] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 20.20x | [17.63, 23.36] |
| JavaScript | js-tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 25.12x | [23.00, 27.17] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | cold | 11 | 44.77x | [44.23, 45.28] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | repeat | 11 | 92.39x | [91.34, 93.35] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ar | stream | 11 | 62.18x | [61.60, 62.74] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | cold | 11 | 27.15x | [26.93, 27.36] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | repeat | 11 | 42.77x | [42.41, 43.20] |
| JavaScript | js-tiktoken | cl100k_base | wiki-de | stream | 11 | 35.65x | [35.32, 36.05] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | cold | 11 | 20.49x | [20.25, 20.77] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | repeat | 11 | 38.80x | [37.92, 39.80] |
| JavaScript | js-tiktoken | cl100k_base | wiki-en | stream | 11 | 24.25x | [23.88, 24.64] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | cold | 11 | 24.49x | [24.21, 24.78] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | repeat | 11 | 55.90x | [55.40, 56.56] |
| JavaScript | js-tiktoken | cl100k_base | wiki-es | stream | 11 | 35.04x | [34.72, 35.43] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | cold | 11 | 24.13x | [23.56, 24.73] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | repeat | 11 | 46.08x | [45.16, 47.16] |
| JavaScript | js-tiktoken | cl100k_base | wiki-fr | stream | 11 | 34.55x | [33.42, 35.65] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | cold | 11 | 42.92x | [42.37, 43.60] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | repeat | 11 | 92.56x | [91.19, 94.12] |
| JavaScript | js-tiktoken | cl100k_base | wiki-hi | stream | 11 | 66.00x | [64.92, 67.03] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | cold | 11 | 255.43x | [251.65, 258.92] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | repeat | 11 | 493.14x | [480.99, 502.13] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ja | stream | 11 | 293.21x | [290.06, 296.73] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | cold | 11 | 37.94x | [37.37, 38.55] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | repeat | 11 | 103.69x | [99.12, 106.61] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ko | stream | 11 | 43.10x | [42.48, 43.73] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | cold | 11 | 53.29x | [52.70, 53.71] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | repeat | 11 | 103.36x | [102.61, 104.14] |
| JavaScript | js-tiktoken | cl100k_base | wiki-ru | stream | 11 | 66.79x | [66.40, 67.19] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | cold | 11 | 148.70x | [147.02, 150.23] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | repeat | 11 | 253.77x | [252.17, 255.41] |
| JavaScript | js-tiktoken | cl100k_base | wiki-th | stream | 11 | 170.84x | [168.31, 173.26] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | cold | 11 | 204.53x | [202.15, 206.97] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | repeat | 11 | 422.07x | [413.11, 427.94] |
| JavaScript | js-tiktoken | cl100k_base | wiki-zh | stream | 11 | 216.29x | [213.88, 218.78] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | repeat | 11 | 22.56x | [18.32, 28.36] |
| JavaScript | js-tiktoken | o200k_base | book-gosrc | stream | 11 | 18.94x | [17.03, 20.57] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 130.34x | [129.38, 131.21] |
| JavaScript | js-tiktoken | o200k_base | book-hongloumeng | stream | 11 | 111.19x | [104.15, 118.40] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | repeat | 11 | 39.63x | [34.07, 47.84] |
| JavaScript | js-tiktoken | o200k_base | book-mobydick | stream | 11 | 17.61x | [16.52, 18.86] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | cold | 11 | 61.63x | [60.47, 62.86] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | repeat | 11 | 68.59x | [66.54, 70.79] |
| JavaScript | js-tiktoken | o200k_base | chat-512B | stream | 11 | 63.94x | [62.95, 64.69] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | cold | 11 | 31.24x | [29.91, 32.05] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | repeat | 11 | 33.60x | [31.04, 36.40] |
| JavaScript | js-tiktoken | o200k_base | td-code-files | stream | 11 | 26.67x | [24.84, 28.86] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 20.86x | [18.40, 24.21] |
| JavaScript | js-tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 28.21x | [24.77, 31.53] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | cold | 11 | 34.50x | [34.18, 34.81] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | repeat | 11 | 56.15x | [55.49, 56.71] |
| JavaScript | js-tiktoken | o200k_base | wiki-ar | stream | 11 | 37.67x | [37.16, 38.18] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | cold | 11 | 25.20x | [24.79, 25.55] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | repeat | 11 | 36.24x | [36.00, 36.45] |
| JavaScript | js-tiktoken | o200k_base | wiki-de | stream | 11 | 30.08x | [29.82, 30.29] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | cold | 11 | 19.14x | [18.95, 19.37] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | repeat | 11 | 38.24x | [37.09, 39.31] |
| JavaScript | js-tiktoken | o200k_base | wiki-en | stream | 11 | 24.21x | [23.89, 24.55] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | cold | 11 | 20.14x | [20.06, 20.22] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | repeat | 11 | 41.24x | [40.98, 41.50] |
| JavaScript | js-tiktoken | o200k_base | wiki-es | stream | 11 | 26.33x | [25.97, 26.86] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | cold | 11 | 19.67x | [19.21, 20.16] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | repeat | 11 | 34.53x | [33.58, 35.82] |
| JavaScript | js-tiktoken | o200k_base | wiki-fr | stream | 11 | 25.76x | [25.47, 26.15] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | cold | 11 | 51.33x | [50.72, 51.91] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | repeat | 11 | 87.26x | [86.35, 88.09] |
| JavaScript | js-tiktoken | o200k_base | wiki-hi | stream | 11 | 59.68x | [59.21, 60.11] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | cold | 11 | 214.82x | [212.51, 216.85] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | repeat | 11 | 448.52x | [441.33, 453.47] |
| JavaScript | js-tiktoken | o200k_base | wiki-ja | stream | 11 | 255.38x | [252.16, 258.64] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | cold | 11 | 37.42x | [37.12, 37.75] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | repeat | 11 | 94.92x | [89.84, 98.81] |
| JavaScript | js-tiktoken | o200k_base | wiki-ko | stream | 11 | 38.42x | [37.96, 38.83] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | cold | 11 | 49.46x | [49.03, 49.95] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | repeat | 11 | 89.16x | [88.48, 89.87] |
| JavaScript | js-tiktoken | o200k_base | wiki-ru | stream | 11 | 55.66x | [55.51, 55.81] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | cold | 11 | 408.74x | [407.67, 409.93] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | repeat | 11 | 717.99x | [712.81, 721.47] |
| JavaScript | js-tiktoken | o200k_base | wiki-th | stream | 11 | 508.09x | [501.35, 514.39] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | cold | 11 | 166.57x | [162.61, 169.30] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | repeat | 11 | 353.31x | [346.98, 358.19] |
| JavaScript | js-tiktoken | o200k_base | wiki-zh | stream | 11 | 181.62x | [178.76, 184.50] |
| Node, native | tiktoken | cl100k_base | book-gosrc | repeat | 11 | 25.26x | [24.58, 25.79] |
| Node, native | tiktoken | cl100k_base | book-gosrc | stream | 11 | 10.18x | [9.93, 10.40] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | repeat | 11 | 15.71x | [15.38, 16.05] |
| Node, native | tiktoken | cl100k_base | book-hongloumeng | stream | 11 | 15.82x | [14.99, 16.36] |
| Node, native | tiktoken | cl100k_base | book-mobydick | repeat | 11 | 23.88x | [22.58, 24.88] |
| Node, native | tiktoken | cl100k_base | book-mobydick | stream | 11 | 7.28x | [7.13, 7.44] |
| Node, native | tiktoken | cl100k_base | chat-512B | cold | 11 | 5.32x | [5.21, 5.44] |
| Node, native | tiktoken | cl100k_base | chat-512B | repeat | 11 | 6.49x | [6.27, 6.68] |
| Node, native | tiktoken | cl100k_base | chat-512B | stream | 11 | 6.10x | [5.91, 6.22] |
| Node, native | tiktoken | cl100k_base | td-code-files | cold | 11 | 23.61x | [23.37, 23.88] |
| Node, native | tiktoken | cl100k_base | td-code-files | repeat | 11 | 32.08x | [29.63, 33.69] |
| Node, native | tiktoken | cl100k_base | td-code-files | stream | 11 | 25.10x | [24.90, 25.29] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | repeat | 11 | 40.33x | [31.94, 49.10] |
| Node, native | tiktoken | cl100k_base | td-synthetic-1MB * | stream | 11 | 39.26x | [37.38, 41.81] |
| Node, native | tiktoken | cl100k_base | wiki-ar | cold | 11 | 6.25x | [6.18, 6.34] |
| Node, native | tiktoken | cl100k_base | wiki-ar | repeat | 11 | 12.57x | [12.34, 12.76] |
| Node, native | tiktoken | cl100k_base | wiki-ar | stream | 11 | 8.91x | [8.85, 8.98] |
| Node, native | tiktoken | cl100k_base | wiki-de | cold | 11 | 4.96x | [4.91, 5.01] |
| Node, native | tiktoken | cl100k_base | wiki-de | repeat | 11 | 7.86x | [7.82, 7.91] |
| Node, native | tiktoken | cl100k_base | wiki-de | stream | 11 | 6.39x | [6.34, 6.43] |
| Node, native | tiktoken | cl100k_base | wiki-en | cold | 11 | 8.03x | [7.96, 8.11] |
| Node, native | tiktoken | cl100k_base | wiki-en | repeat | 11 | 17.46x | [17.30, 17.62] |
| Node, native | tiktoken | cl100k_base | wiki-en | stream | 11 | 10.93x | [10.82, 11.02] |
| Node, native | tiktoken | cl100k_base | wiki-es | cold | 11 | 5.07x | [5.05, 5.09] |
| Node, native | tiktoken | cl100k_base | wiki-es | repeat | 11 | 12.71x | [12.57, 12.82] |
| Node, native | tiktoken | cl100k_base | wiki-es | stream | 11 | 7.92x | [7.88, 7.97] |
| Node, native | tiktoken | cl100k_base | wiki-fr | cold | 11 | 4.83x | [4.76, 4.89] |
| Node, native | tiktoken | cl100k_base | wiki-fr | repeat | 11 | 10.12x | [9.90, 10.33] |
| Node, native | tiktoken | cl100k_base | wiki-fr | stream | 11 | 7.78x | [7.64, 7.89] |
| Node, native | tiktoken | cl100k_base | wiki-hi | cold | 11 | 9.25x | [9.17, 9.34] |
| Node, native | tiktoken | cl100k_base | wiki-hi | repeat | 11 | 17.96x | [17.86, 18.04] |
| Node, native | tiktoken | cl100k_base | wiki-hi | stream | 11 | 12.35x | [12.32, 12.39] |
| Node, native | tiktoken | cl100k_base | wiki-ja | cold | 11 | 10.35x | [10.26, 10.44] |
| Node, native | tiktoken | cl100k_base | wiki-ja | repeat | 11 | 22.10x | [21.54, 22.53] |
| Node, native | tiktoken | cl100k_base | wiki-ja | stream | 11 | 12.67x | [12.59, 12.74] |
| Node, native | tiktoken | cl100k_base | wiki-ko | cold | 11 | 6.04x | [6.01, 6.07] |
| Node, native | tiktoken | cl100k_base | wiki-ko | repeat | 11 | 15.15x | [14.56, 15.50] |
| Node, native | tiktoken | cl100k_base | wiki-ko | stream | 11 | 6.55x | [6.44, 6.63] |
| Node, native | tiktoken | cl100k_base | wiki-ru | cold | 11 | 5.00x | [4.98, 5.02] |
| Node, native | tiktoken | cl100k_base | wiki-ru | repeat | 11 | 10.37x | [10.30, 10.43] |
| Node, native | tiktoken | cl100k_base | wiki-ru | stream | 11 | 6.78x | [6.76, 6.79] |
| Node, native | tiktoken | cl100k_base | wiki-th | cold | 11 | 11.73x | [11.55, 11.93] |
| Node, native | tiktoken | cl100k_base | wiki-th | repeat | 11 | 21.80x | [21.59, 21.98] |
| Node, native | tiktoken | cl100k_base | wiki-th | stream | 11 | 14.11x | [14.06, 14.15] |
| Node, native | tiktoken | cl100k_base | wiki-zh | cold | 11 | 10.92x | [10.77, 11.07] |
| Node, native | tiktoken | cl100k_base | wiki-zh | repeat | 11 | 24.11x | [23.60, 24.45] |
| Node, native | tiktoken | cl100k_base | wiki-zh | stream | 11 | 12.18x | [12.06, 12.28] |
| Node, native | tiktoken | o200k_base | book-gosrc | repeat | 11 | 22.66x | [17.89, 26.93] |
| Node, native | tiktoken | o200k_base | book-gosrc | stream | 11 | 13.52x | [13.46, 13.57] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | repeat | 11 | 10.29x | [10.24, 10.34] |
| Node, native | tiktoken | o200k_base | book-hongloumeng | stream | 11 | 10.15x | [9.89, 10.33] |
| Node, native | tiktoken | o200k_base | book-mobydick | repeat | 11 | 21.72x | [17.66, 25.06] |
| Node, native | tiktoken | o200k_base | book-mobydick | stream | 11 | 6.90x | [6.70, 7.09] |
| Node, native | tiktoken | o200k_base | chat-512B | cold | 11 | 4.83x | [4.73, 4.93] |
| Node, native | tiktoken | o200k_base | chat-512B | repeat | 11 | 5.47x | [5.36, 5.58] |
| Node, native | tiktoken | o200k_base | chat-512B | stream | 11 | 5.27x | [5.12, 5.40] |
| Node, native | tiktoken | o200k_base | td-code-files | cold | 11 | 21.09x | [20.07, 21.77] |
| Node, native | tiktoken | o200k_base | td-code-files | repeat | 11 | 32.23x | [30.05, 34.16] |
| Node, native | tiktoken | o200k_base | td-code-files | stream | 11 | 23.87x | [23.71, 24.05] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | repeat | 11 | 46.84x | [37.28, 56.68] |
| Node, native | tiktoken | o200k_base | td-synthetic-1MB * | stream | 11 | 44.77x | [43.22, 46.38] |
| Node, native | tiktoken | o200k_base | wiki-ar | cold | 11 | 5.32x | [5.27, 5.36] |
| Node, native | tiktoken | o200k_base | wiki-ar | repeat | 11 | 8.50x | [8.40, 8.58] |
| Node, native | tiktoken | o200k_base | wiki-ar | stream | 11 | 5.74x | [5.43, 5.95] |
| Node, native | tiktoken | o200k_base | wiki-de | cold | 11 | 4.87x | [4.82, 4.93] |
| Node, native | tiktoken | o200k_base | wiki-de | repeat | 11 | 7.48x | [7.43, 7.52] |
| Node, native | tiktoken | o200k_base | wiki-de | stream | 11 | 6.20x | [6.14, 6.24] |
| Node, native | tiktoken | o200k_base | wiki-en | cold | 11 | 7.96x | [7.92, 8.02] |
| Node, native | tiktoken | o200k_base | wiki-en | repeat | 11 | 17.97x | [17.82, 18.11] |
| Node, native | tiktoken | o200k_base | wiki-en | stream | 11 | 10.49x | [10.34, 10.62] |
| Node, native | tiktoken | o200k_base | wiki-es | cold | 11 | 5.57x | [5.54, 5.61] |
| Node, native | tiktoken | o200k_base | wiki-es | repeat | 11 | 12.67x | [12.45, 12.83] |
| Node, native | tiktoken | o200k_base | wiki-es | stream | 11 | 7.81x | [7.76, 7.85] |
| Node, native | tiktoken | o200k_base | wiki-fr | cold | 11 | 5.32x | [5.26, 5.38] |
| Node, native | tiktoken | o200k_base | wiki-fr | repeat | 11 | 10.32x | [10.22, 10.43] |
| Node, native | tiktoken | o200k_base | wiki-fr | stream | 11 | 7.84x | [7.79, 7.88] |
| Node, native | tiktoken | o200k_base | wiki-hi | cold | 11 | 5.35x | [5.26, 5.43] |
| Node, native | tiktoken | o200k_base | wiki-hi | repeat | 11 | 9.05x | [8.96, 9.14] |
| Node, native | tiktoken | o200k_base | wiki-hi | stream | 11 | 6.21x | [6.15, 6.27] |
| Node, native | tiktoken | o200k_base | wiki-ja | cold | 11 | 8.99x | [8.91, 9.06] |
| Node, native | tiktoken | o200k_base | wiki-ja | repeat | 11 | 19.50x | [19.21, 19.71] |
| Node, native | tiktoken | o200k_base | wiki-ja | stream | 11 | 10.95x | [10.87, 11.04] |
| Node, native | tiktoken | o200k_base | wiki-ko | cold | 11 | 6.00x | [5.97, 6.02] |
| Node, native | tiktoken | o200k_base | wiki-ko | repeat | 11 | 14.06x | [13.39, 14.55] |
| Node, native | tiktoken | o200k_base | wiki-ko | stream | 11 | 5.94x | [5.91, 5.98] |
| Node, native | tiktoken | o200k_base | wiki-ru | cold | 11 | 4.84x | [4.78, 4.90] |
| Node, native | tiktoken | o200k_base | wiki-ru | repeat | 11 | 9.58x | [9.54, 9.63] |
| Node, native | tiktoken | o200k_base | wiki-ru | stream | 11 | 6.15x | [6.12, 6.17] |
| Node, native | tiktoken | o200k_base | wiki-th | cold | 11 | 10.01x | [9.96, 10.08] |
| Node, native | tiktoken | o200k_base | wiki-th | repeat | 11 | 18.24x | [18.12, 18.34] |
| Node, native | tiktoken | o200k_base | wiki-th | stream | 11 | 12.03x | [11.95, 12.11] |
| Node, native | tiktoken | o200k_base | wiki-zh | cold | 11 | 8.81x | [8.75, 8.87] |
| Node, native | tiktoken | o200k_base | wiki-zh | repeat | 11 | 20.00x | [19.69, 20.22] |
| Node, native | tiktoken | o200k_base | wiki-zh | stream | 11 | 9.93x | [9.88, 9.98] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | repeat | 11 | 7.20x | [6.95, 7.47] |
| Node, native | gpt-tokenizer | cl100k_base | book-gosrc | stream | 11 | 5.17x | [4.95, 5.45] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | repeat | 11 | 3.74x | [3.64, 3.87] |
| Node, native | gpt-tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 56.83x | [53.48, 59.40] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | repeat | 11 | 7.67x | [7.26, 8.01] |
| Node, native | gpt-tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.52x | [5.13, 6.00] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | cold | 11 | 14.23x | [13.96, 14.52] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | repeat | 11 | 13.51x | [13.07, 14.03] |
| Node, native | gpt-tokenizer | cl100k_base | chat-512B | stream | 11 | 11.28x | [10.77, 11.74] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | cold | 11 | 5.21x | [5.06, 5.38] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | repeat | 11 | 5.30x | [4.70, 6.01] |
| Node, native | gpt-tokenizer | cl100k_base | td-code-files | stream | 11 | 5.12x | [4.97, 5.26] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | repeat | 11 | 9.09x | [6.44, 12.94] |
| Node, native | gpt-tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 8.42x | [7.76, 9.21] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | cold | 11 | 18.40x | [17.84, 19.05] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | repeat | 11 | 2.94x | [2.88, 2.99] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ar | stream | 11 | 13.20x | [12.80, 13.70] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | cold | 11 | 7.14x | [7.02, 7.30] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | repeat | 11 | 2.13x | [2.11, 2.15] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-de | stream | 11 | 7.27x | [7.08, 7.50] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | cold | 11 | 4.78x | [4.64, 4.93] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | repeat | 11 | 5.94x | [5.84, 6.06] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-en | stream | 11 | 6.24x | [6.04, 6.48] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | cold | 11 | 6.66x | [6.47, 6.84] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | repeat | 11 | 3.33x | [3.28, 3.39] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-es | stream | 11 | 6.83x | [6.65, 7.08] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | cold | 11 | 7.41x | [7.17, 7.65] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | repeat | 11 | 2.99x | [2.94, 3.04] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-fr | stream | 11 | 7.01x | [6.77, 7.26] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | cold | 11 | 11.62x | [10.83, 12.55] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | repeat | 11 | 4.50x | [4.46, 4.54] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-hi | stream | 11 | 5.34x | [5.28, 5.42] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | cold | 11 | 35.70x | [34.75, 36.70] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | repeat | 11 | 3.80x | [3.71, 3.87] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ja | stream | 11 | 42.48x | [41.49, 43.57] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | cold | 11 | 16.69x | [16.33, 17.15] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | repeat | 11 | 3.41x | [3.27, 3.51] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ko | stream | 11 | 13.83x | [13.48, 14.18] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | cold | 11 | 15.48x | [15.26, 15.71] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | repeat | 11 | 2.18x | [2.16, 2.19] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-ru | stream | 11 | 13.47x | [13.31, 13.64] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | cold | 11 | 35.84x | [34.73, 37.07] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | repeat | 11 | 5.16x | [5.10, 5.22] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-th | stream | 11 | 31.47x | [30.99, 32.14] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | cold | 11 | 42.36x | [41.53, 43.06] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | repeat | 11 | 4.91x | [4.80, 4.99] |
| Node, native | gpt-tokenizer | cl100k_base | wiki-zh | stream | 11 | 45.90x | [45.08, 46.75] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | repeat | 11 | 8.48x | [6.56, 10.30] |
| Node, native | gpt-tokenizer | o200k_base | book-gosrc | stream | 11 | 6.71x | [6.45, 7.00] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | repeat | 11 | 2.49x | [2.48, 2.50] |
| Node, native | gpt-tokenizer | o200k_base | book-hongloumeng | stream | 11 | 45.18x | [43.57, 46.64] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | repeat | 11 | 8.06x | [6.57, 9.30] |
| Node, native | gpt-tokenizer | o200k_base | book-mobydick | stream | 11 | 5.39x | [5.19, 5.60] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | cold | 11 | 12.84x | [12.44, 13.26] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | repeat | 11 | 1.31x | [1.28, 1.34] |
| Node, native | gpt-tokenizer | o200k_base | chat-512B | stream | 11 | 11.91x | [11.63, 12.17] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | cold | 11 | 4.94x | [4.72, 5.16] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | repeat | 11 | 5.35x | [4.96, 5.69] |
| Node, native | gpt-tokenizer | o200k_base | td-code-files | stream | 11 | 5.18x | [4.99, 5.41] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | repeat | 11 | 8.88x | [7.33, 10.70] |
| Node, native | gpt-tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 9.68x | [8.85, 10.63] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | cold | 11 | 16.24x | [15.79, 16.67] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | repeat | 11 | 2.51x | [2.47, 2.54] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ar | stream | 11 | 11.92x | [11.09, 12.62] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | cold | 11 | 7.16x | [6.83, 7.52] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | repeat | 11 | 2.41x | [2.37, 2.44] |
| Node, native | gpt-tokenizer | o200k_base | wiki-de | stream | 11 | 7.82x | [7.52, 8.16] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | cold | 11 | 4.95x | [4.85, 5.05] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | repeat | 11 | 6.49x | [6.40, 6.59] |
| Node, native | gpt-tokenizer | o200k_base | wiki-en | stream | 11 | 7.07x | [6.86, 7.28] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | cold | 11 | 6.25x | [6.02, 6.50] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | repeat | 11 | 4.07x | [3.98, 4.14] |
| Node, native | gpt-tokenizer | o200k_base | wiki-es | stream | 11 | 7.05x | [6.89, 7.23] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | cold | 11 | 7.31x | [6.95, 7.72] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | repeat | 11 | 3.52x | [3.47, 3.56] |
| Node, native | gpt-tokenizer | o200k_base | wiki-fr | stream | 11 | 7.22x | [7.12, 7.31] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | cold | 11 | 14.55x | [14.11, 14.98] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | repeat | 11 | 2.32x | [2.29, 2.34] |
| Node, native | gpt-tokenizer | o200k_base | wiki-hi | stream | 11 | 11.17x | [11.04, 11.30] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | cold | 11 | 33.27x | [32.66, 33.92] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | repeat | 11 | 3.56x | [3.51, 3.61] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ja | stream | 11 | 40.77x | [39.76, 41.87] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | cold | 11 | 18.07x | [17.33, 19.21] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | repeat | 11 | 3.69x | [3.52, 3.80] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ko | stream | 11 | 14.29x | [13.99, 14.61] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | cold | 11 | 16.60x | [16.14, 17.11] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | repeat | 11 | 2.45x | [2.43, 2.47] |
| Node, native | gpt-tokenizer | o200k_base | wiki-ru | stream | 11 | 14.06x | [13.86, 14.24] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | cold | 11 | 26.53x | [26.01, 27.22] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | repeat | 11 | 1.39x | [1.38, 1.39] |
| Node, native | gpt-tokenizer | o200k_base | wiki-th | stream | 11 | 33.80x | [33.59, 34.05] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | cold | 11 | 36.25x | [36.02, 36.52] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | repeat | 11 | 4.02x | [3.95, 4.08] |
| Node, native | gpt-tokenizer | o200k_base | wiki-zh | stream | 11 | 41.51x | [40.54, 42.69] |
| Go | tiktoken-go | cl100k_base | book-gosrc | stream | 11 | 12.25x | [9.88, 15.05] |
| Go | tiktoken-go | cl100k_base | book-hongloumeng | stream | 11 | 2.91x | [2.69, 3.23] |
| Go | tiktoken-go | cl100k_base | book-mobydick | stream | 11 | 8.52x | [7.45, 9.92] |
| Go | tiktoken-go | cl100k_base | chat-512B | stream | 11 | 4.58x | [4.42, 4.74] |
| Go | tiktoken-go | cl100k_base | td-code-files | stream | 11 | 21.13x | [18.90, 23.56] |
| Go | tiktoken-go | cl100k_base | td-synthetic-1MB * | stream | 11 | 20.57x | [17.10, 23.76] |
| Go | tiktoken-go | cl100k_base | wiki-ar | stream | 11 | 5.05x | [4.69, 5.49] |
| Go | tiktoken-go | cl100k_base | wiki-de | stream | 11 | 6.64x | [5.99, 7.30] |
| Go | tiktoken-go | cl100k_base | wiki-en | stream | 11 | 12.24x | [11.28, 13.23] |
| Go | tiktoken-go | cl100k_base | wiki-es | stream | 11 | 7.87x | [7.09, 8.67] |
| Go | tiktoken-go | cl100k_base | wiki-fr | stream | 11 | 8.09x | [7.26, 8.89] |
| Go | tiktoken-go | cl100k_base | wiki-hi | stream | 11 | 8.29x | [7.28, 9.37] |
| Go | tiktoken-go | cl100k_base | wiki-ja | stream | 11 | 3.33x | [3.07, 3.52] |
| Go | tiktoken-go | cl100k_base | wiki-ko | stream | 11 | 5.20x | [4.64, 5.87] |
| Go | tiktoken-go | cl100k_base | wiki-ru | stream | 11 | 4.87x | [4.55, 5.15] |
| Go | tiktoken-go | cl100k_base | wiki-th | stream | 11 | 4.39x | [3.99, 4.79] |
| Go | tiktoken-go | cl100k_base | wiki-zh | stream | 11 | 3.25x | [3.12, 3.32] |
| Go | tiktoken-go | o200k_base | book-gosrc | stream | 11 | 13.83x | [10.22, 18.19] |
| Go | tiktoken-go | o200k_base | book-hongloumeng | stream | 11 | 2.69x | [2.44, 2.99] |
| Go | tiktoken-go | o200k_base | book-mobydick | stream | 11 | 8.46x | [6.82, 10.13] |
| Go | tiktoken-go | o200k_base | chat-512B | stream | 11 | 4.63x | [4.47, 4.74] |
| Go | tiktoken-go | o200k_base | td-code-files | stream | 11 | 20.89x | [18.44, 23.40] |
| Go | tiktoken-go | o200k_base | td-synthetic-1MB * | stream | 11 | 21.02x | [17.74, 24.11] |
| Go | tiktoken-go | o200k_base | wiki-ar | stream | 11 | 7.24x | [6.83, 7.52] |
| Go | tiktoken-go | o200k_base | wiki-de | stream | 11 | 7.22x | [6.63, 7.75] |
| Go | tiktoken-go | o200k_base | wiki-en | stream | 11 | 12.20x | [11.23, 13.02] |
| Go | tiktoken-go | o200k_base | wiki-es | stream | 11 | 10.14x | [9.98, 10.32] |
| Go | tiktoken-go | o200k_base | wiki-fr | stream | 11 | 9.60x | [8.70, 10.41] |
| Go | tiktoken-go | o200k_base | wiki-hi | stream | 11 | 7.22x | [6.46, 7.92] |
| Go | tiktoken-go | o200k_base | wiki-ja | stream | 11 | 3.38x | [3.06, 3.69] |
| Go | tiktoken-go | o200k_base | wiki-ko | stream | 11 | 5.27x | [4.83, 5.80] |
| Go | tiktoken-go | o200k_base | wiki-ru | stream | 11 | 4.72x | [4.36, 5.08] |
| Go | tiktoken-go | o200k_base | wiki-th | stream | 11 | 2.90x | [2.60, 3.18] |
| Go | tiktoken-go | o200k_base | wiki-zh | stream | 11 | 2.98x | [2.84, 3.12] |
| Go | tokenizer | cl100k_base | book-gosrc | stream | 11 | 6.39x | [4.97, 8.28] |
| Go | tokenizer | cl100k_base | book-hongloumeng | stream | 11 | 2.68x | [2.24, 3.24] |
| Go | tokenizer | cl100k_base | book-mobydick | stream | 11 | 5.49x | [4.13, 7.23] |
| Go | tokenizer | cl100k_base | chat-512B | stream | 11 | 3.21x | [3.09, 3.31] |
| Go | tokenizer | cl100k_base | td-code-files | stream | 11 | 11.54x | [10.70, 12.36] |
| Go | tokenizer | cl100k_base | td-synthetic-1MB * | stream | 11 | 9.59x | [7.27, 12.49] |
| Go | tokenizer | cl100k_base | wiki-ar | stream | 11 | 3.79x | [3.47, 4.19] |
| Go | tokenizer | cl100k_base | wiki-de | stream | 11 | 4.66x | [4.10, 5.19] |
| Go | tokenizer | cl100k_base | wiki-en | stream | 11 | 6.88x | [6.21, 7.47] |
| Go | tokenizer | cl100k_base | wiki-es | stream | 11 | 5.32x | [4.78, 5.87] |
| Go | tokenizer | cl100k_base | wiki-fr | stream | 11 | 5.32x | [4.81, 5.90] |
| Go | tokenizer | cl100k_base | wiki-hi | stream | 11 | 5.76x | [5.14, 6.42] |
| Go | tokenizer | cl100k_base | wiki-ja | stream | 11 | 2.96x | [2.82, 3.10] |
| Go | tokenizer | cl100k_base | wiki-ko | stream | 11 | 3.47x | [2.84, 4.15] |
| Go | tokenizer | cl100k_base | wiki-ru | stream | 11 | 3.84x | [3.57, 4.07] |
| Go | tokenizer | cl100k_base | wiki-th | stream | 11 | 3.63x | [3.31, 3.95] |
| Go | tokenizer | cl100k_base | wiki-zh | stream | 11 | 2.85x | [2.78, 2.89] |
| Go | tokenizer | o200k_base | book-gosrc | stream | 11 | 6.84x | [5.50, 8.60] |
| Go | tokenizer | o200k_base | book-hongloumeng | stream | 11 | 2.32x | [2.14, 2.56] |
| Go | tokenizer | o200k_base | book-mobydick | stream | 11 | 4.65x | [3.75, 5.99] |
| Go | tokenizer | o200k_base | chat-512B | stream | 11 | 3.06x | [2.94, 3.14] |
| Go | tokenizer | o200k_base | td-code-files | stream | 11 | 10.46x | [9.15, 11.67] |
| Go | tokenizer | o200k_base | td-synthetic-1MB * | stream | 11 | 10.53x | [8.46, 13.17] |
| Go | tokenizer | o200k_base | wiki-ar | stream | 11 | 4.79x | [4.54, 4.97] |
| Go | tokenizer | o200k_base | wiki-de | stream | 11 | 4.30x | [3.91, 4.66] |
| Go | tokenizer | o200k_base | wiki-en | stream | 11 | 6.76x | [6.11, 7.37] |
| Go | tokenizer | o200k_base | wiki-es | stream | 11 | 6.08x | [5.92, 6.27] |
| Go | tokenizer | o200k_base | wiki-fr | stream | 11 | 5.62x | [5.18, 6.05] |
| Go | tokenizer | o200k_base | wiki-hi | stream | 11 | 4.14x | [3.73, 4.50] |
| Go | tokenizer | o200k_base | wiki-ja | stream | 11 | 2.80x | [2.55, 3.03] |
| Go | tokenizer | o200k_base | wiki-ko | stream | 11 | 4.11x | [3.31, 4.95] |
| Go | tokenizer | o200k_base | wiki-ru | stream | 11 | 3.45x | [3.18, 3.70] |
| Go | tokenizer | o200k_base | wiki-th | stream | 11 | 2.46x | [2.22, 2.69] |
| Go | tokenizer | o200k_base | wiki-zh | stream | 11 | 2.47x | [2.37, 2.55] |
