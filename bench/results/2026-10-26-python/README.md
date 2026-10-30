# Python drop-in against tiktoken, 2026-10-26

A GCP c3-standard-4 (Xeon 8481C), Python 3.11.2, tiktoken 0.14.0,
tiktoken-fast built from this commit. `bench/python/dropin.py` pinned to one
core with taskset, 11 rounds, arms alternating.

- `py-main.txt`: the main workloads. Geometric means: encode() 3.44x new text,
  2.69x fresh caches; encode_ordinary() 3.48x and 2.75x; batch 3.50x. Slowest
  workload o200k_base German Wikipedia (1.93x, 1.57x, 2.01x). Zero mismatches.
  encode() is skipped on td-code-files, whose text spells `<|fim_prefix|>`
  and makes encode() raise in both libraries.
- `py-tiny.txt`: 16 B, 64 B and 1 KiB documents. encode() 3.25x and 3.01x.
- `py-batch4.txt`: encode_ordinary_batch() with 8 threads on all four vCPUs
  (`bench/python/batch_threads.py`): 3.2x to 5.0x on documents, 20x to 24x on
  16-byte strings, where tiktoken's thread pool costs more than the encoding.
