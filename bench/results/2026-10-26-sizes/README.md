# Document size sweep, 2026-10-26

Xeon 8481C (c3-standard-4), one core. Workloads from `bench/data/make_sizes.py`:
English, Chinese, Russian and code, 16 B to 1 MiB in doubling steps, cl100k_base
and o200k_base, 11 rounds through `bench/compare`. Zero mismatches in every row.

- `sizes-09c3efb.txt`: before. 27 of 408 rows lost to bpe-openai, all at 128 B
  or less, 26 of them with caches dropped before every document (worst:
  o200k English 16 B, 0.58x of tiktoken-rs against bpe-openai's 1.19x). Setting
  up the per-caller tables cost about 1.4 µs per fresh call.
- `sizes-fixed.txt`: a fresh caller now starts with no tables and builds them
  after 512 B of input; the merge scratch arrays are allocated on first use.
  Setup costs about 80 ns. Two rows remain behind bpe-openai: Russian at 16 B
  with caches kept (cl100k 2.28x against 2.40x, o200k 1.44x against 1.51x).
- `rust-main-fixed.txt`: the main workloads with the change, no row behind
  bpe-openai, geometric mean 1.8% above the 2026-10-25 run.

Two attempts at the Russian 16 B rows were dropped: skipping the tables for
texts under 64 B (ties cl100k, costs 10% at 64 B) and skipping the whole-piece
lookup before backtracking (mixed across scripts).
