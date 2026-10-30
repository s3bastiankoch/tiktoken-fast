# Benchmarks

The published numbers come from `reproduce.sh`; [`docs/benchmarks.md`](../docs/benchmarks.md)
explains it. This page describes the individual harnesses it calls and the
older `run_all.sh`.

Everything here compares four encoders on the same documents and checks that
each returns the reference token ids before any time is read:

- tiktoken is the reference: openai/tiktoken 0.14.0 from Python, and its core
  through tiktoken-rs 0.12.1, which vendors it verbatim.
- tiktoken-fast, this crate, from Rust and through `python/binding`.
- bpe-openai 0.3.2, GitHub's backtracking encoder (Rust; cl100k_base and
  o200k_base only).
- TokenDagger at commit 1eed815, a C++ reimplementation with a Python API.

## Run

On a Linux machine with Rust, Python 3.11, a C++ compiler and the PCRE2 and
Python headers:

```sh
bench/run_all.sh
```

It fetches the Wikipedia sample, builds every arm, writes the workloads, runs
everything pinned to one core (`CORE`, default 3) and writes the results to
`results/<date>-<commit>/`. A full run takes about two hours. Then

```sh
python bench/plot_languages.py bench/results/<run>/summary.json ... docs/img
```

redraws the README figures (needs Chrome; set `CHROME` if it is not found).

## What is measured

| harness | arms | encodings |
|---|---|---|
| `compare` (Rust) | tiktoken-rs, tiktoken-fast, bpe-openai | cl100k_base, o200k_base, Llama 4; p50k_base and r50k_base without bpe-openai |
| `python/bench.py` | tiktoken, tiktoken-fast, TokenDagger | cl100k_base, o200k_base, Llama 4 |
| `threads` | the Rust arms, one encoder shared by 1 to 4 threads | cl100k_base, o200k_base |

Llama 4 uses o200k_base's split pattern with its own vocabulary, taken from
TokenDagger's repository, so TokenDagger runs on its own test vocabulary.

Workloads (`data/make_workloads.py`):

- Wikipedia: 60 articles of at least 2 KB in each of English, German, French,
  Spanish, Russian, Chinese, Japanese, Korean, Arabic, Hindi and Thai, drawn
  with a fixed seed from the 2023-11-01 dump.
- Chat: 6 000 chunks of about 512 bytes cut from those articles and from code.
- Code: TokenDagger's own source files, and 256 KiB of the Go standard library.
- Books: 256 KiB each of *Moby-Dick* and 紅樓夢.
- TokenDagger's synthetic throughput text, 1 MB, from its own generator.
- Low-repetition text: base64, hex, UUIDs, minified JavaScript, random
  Unicode from every plane, emoji sequences, numeric JSON.

Scenarios, for every workload:

- `stream`: each document is encoded once per round; tiktoken-fast's caches
  start empty each round and persist across its documents, as in a server.
- `repeat`: the same documents again with warm caches.
- `cold`: tiktoken-fast's caches are dropped before every document.

Each round times every arm on the whole workload, in an order that rotates
from round to round; 11 rounds. A row reports the geometric mean of the
per-round speed-ups over the reference with a 95% bootstrap interval, and the number of documents
whose ids differ from the reference.

## Document size

`data/make_sizes.py` cuts English, Chinese, Russian and code into documents of
16 B to 1 MiB in doubling steps; `compare` on that directory gives the
speed-up by size, and `plot_sizes.py` draws it. `compare/src/bin/tiny.rs`
times single calls on one workload. Results: `results/2026-10-26-sizes/`.

`results/2026-10-25-hot-loop/` holds an earlier measurement with the paired
driver of tiktoken-go-fast, which repeats the same text and so reflects warm
caches only.
