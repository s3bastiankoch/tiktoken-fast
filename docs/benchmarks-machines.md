# Headlines on three machines

Written by `python bench/summarize.py <intel> <arm> <amd> --combined docs/benchmarks-machines.md` from the runs in `bench/results/`. New text, geometric mean and 95% interval per machine; spread is the largest over the smallest.

| | over | c3-standard-4 (Intel(R) Xeon(R) Platinum 8481C CPU) | c4a-standard-4 (Neoverse-V2) | c3d-standard-4 (AMD EPYC 9B14) | spread |
|---|---|---|---|---|---:|
| Rust | tiktoken's core (tiktoken-rs 0.12.1) | **3.55x** [3.54, 3.57] | **3.69x** [3.67, 3.71] | **4.13x** [4.11, 4.15] | 1.16 |
| Rust | bpe-openai 0.3.2 | **1.98x** [1.97, 1.98] | **2.30x** [2.29, 2.31] | **2.30x** [2.29, 2.31] | 1.16 |
| Python | tiktoken 0.14.0 | **3.28x** [3.26, 3.30] | **3.50x** [3.48, 3.52] | **3.78x** [3.76, 3.80] | 1.15 |
| Python, encode() | tiktoken 0.14.0 | **3.21x** [3.19, 3.23] | **3.39x** [3.36, 3.42] | **3.68x** [3.66, 3.71] | 1.15 |
| JavaScript | tiktoken 1.0.22 (npm) | **7.66x** [7.60, 7.71] | **8.76x** [8.66, 8.86] | **9.71x** [9.63, 9.77] | 1.27 |
| JavaScript | gpt-tokenizer 4.1.0 | **10.24x** [10.14, 10.32] | **11.09x** [10.96, 11.22] | **10.56x** [10.46, 10.64] | 1.08 |
| JavaScript | js-tiktoken 1.0.21 | **54.39x** [53.93, 54.79] | **54.44x** [53.75, 55.15] | **56.62x** [56.15, 57.06] | 1.04 |
| Node, native | tiktoken 1.0.22 (npm) | **9.19x** [9.16, 9.21] | **10.62x** [10.51, 10.72] | **11.46x** [11.42, 11.50] | 1.25 |
| Node, native | gpt-tokenizer 4.1.0 | **12.28x** [12.21, 12.35] | **13.44x** [13.29, 13.59] | **12.46x** [12.39, 12.53] | 1.09 |
| Go | pkoukk/tiktoken-go (200cf60, vendored) | **6.44x** [6.31, 6.57] | **6.72x** [6.60, 6.84] | **6.80x** [6.67, 6.93] | 1.06 |
| Go | tiktoken-go/tokenizer 0.8.1 | **4.37x** [4.27, 4.47] | **4.17x** [4.09, 4.26] | **4.29x** [4.20, 4.37] | 1.05 |
