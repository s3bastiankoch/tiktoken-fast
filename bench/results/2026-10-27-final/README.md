# Final run with the JavaScript build, 2026-10-27

A GCP c3-standard-4 (Xeon 8481C), one core pinned. Rust 1.99.1, Node
22.23.4, Python 3.11.2. Zero token mismatches in every file.

- `tests.txt`: Rust parity tests, the Python parity tests (12) and the
  JavaScript parity test against the tiktoken npm package.
- `rust-main.txt`: the main workloads; against the 2026-10-26 run, 4.5%
  faster on new text, 4.6% with fresh caches, 29% on repeated text (the
  long-segment memo); ahead of bpe-openai on every workload.
- `js-seven-*.txt`: gpt-tokenizer's headline benchmark rebuilt from its
  published method (`js/bench/seven.mjs`): seven inputs encoded, counted and
  decoded repeatedly with default caches, five processes per library.
  o200k_base mean ms per call: encode 1.00 (gpt-tokenizer 7.28, bpe-lite
  6.24), count 0.87 (5.34), decode 0.67 (0.95), load 143 ms (153). Every
  input is faster. The benchmark repeats each input, so caches decide it:
  UnicodeRandom takes 2.5 ms because the long-segment memo holds it after the
  second call; on new text the gap is the one in `js-main.txt`.
- `js-main.txt`: `js/bench/bench.mjs` on the main workloads, tiktoken npm as
  the reference, gpt-tokenizer and tiktoken-fast with their caches emptied
  alike. Geometric means over tiktoken npm: new text 7.9x (gpt-tokenizer
  0.77x), fresh caches 6.2x (0.53x), warm caches 13.3x (4.1x). Against
  gpt-tokenizer every row is faster; closest o200k_base Thai with warm caches,
  1.16x.
- `python-main.txt`, `python-bench-module.txt`: the Python drop-in against
  tiktoken (3.3x new text, 2.7x fresh caches, 3.4x batch) and the output of
  `python -m tiktoken_fast.bench` (4.3x geometric mean).
