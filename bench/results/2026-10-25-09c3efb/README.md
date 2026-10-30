Run of `bench/run_all.sh` at commit 09c3efb on a GCP `c3-standard-4`
(Xeon Platinum 8481C), Debian 12, Rust 1.99.1, 2026-10-25. Single-threaded
runs on core 3; `threads.txt` uses cores 0 to 3.

- `rust-main.txt`: tiktoken-rs, tiktoken-fast, bpe-openai on the main workloads
- `rust-edge.txt`: the same on low-repetition text
- `rust-p50k-r50k.txt`: p50k_base and r50k_base (no bpe-openai)
- `python-main.txt`: tiktoken, TokenDagger, tiktoken-fast from Python
- `threads.txt`: one shared encoder, 1 to 4 threads, chat-sized messages

Every row's `mism` count is 0.
