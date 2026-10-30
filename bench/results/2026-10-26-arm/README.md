# ARM: Google Axion (Neoverse V2), 2026-10-26

GCP `c4a-standard-4`, Debian 12 arm64, rustc 1.99.1, Python 3.11.2, tiktoken
0.14.0. Same workloads as the x86 runs (cl100k_base and o200k_base), one core
pinned with taskset, 11 rounds. The code has no ARM-specific paths: where x86
uses AVX2 for the cut bitmap, ARM takes the eight-bytes-at-a-time path.

- `tests.txt`: parity tests, a 120 s differential soak (588 380 texts, no
  mismatch) and the Python parity tests (11 passed).
- `rust-main.txt`: against tiktoken-rs, 4.16x new text, 5.60x repeat, 3.04x
  fresh caches (slowest workload 1.78x, 1.84x, 1.48x). bpe-openai is 1.63x,
  1.64x, 1.66x, so this crate is ahead of it on every workload (2.55x, 3.42x,
  1.83x; closest o200k_base Arabic Wikipedia with fresh caches, 1.13x). The
  x86 run of the same commit has 3.65x, 5.07x and 2.75x over tiktoken-rs.
- `python-main.txt`: the Python drop-in against tiktoken: encode() 3.81x new
  text, 2.94x fresh caches; encode_ordinary() 3.89x and 3.02x; batch 4.11x.
  Slowest: o200k_base German Wikipedia with fresh caches, 1.76x. No mismatch.
