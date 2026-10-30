# How the numbers are made

Every speed-up in the README comes out of one script, `bench/reproduce.sh`,
run unchanged on each machine listed. This page says what it measures, what
each library gets, and what the numbers do not show.

## Run it yourself

On Debian 12 (x86-64 or arm64):

```sh
bench/setup-debian.sh      # pinned toolchains, about 10 minutes
bench/reproduce.sh         # every language, about 2 hours
QUICK=1 bench/reproduce.sh # 5 rounds on 5 workloads, about 15 minutes
```

The result directory holds `summary.md` (the tables), `summary.json`, the
raw time of every round of every library in `raw/*.jsonl`, the plain harness
output, and `machine.json`: CPU, instance type, kernel, frequency governor,
every toolchain and library version, the commit, the CPU time the hypervisor
took during the run, and the SHA-256 of every workload file. Those hashes are
checked against [`bench/data/workloads.sha256`](../bench/data/workloads.sha256)
before anything is timed, so a run on different text stops and says so.

`python bench/summarize.py <run> [<run> ...] --combined out.md` rebuilds every
table from the raw rounds, and `python bench/plot_languages.py
<run>/summary.json ... docs/img` the headline figures (rendered with headless
Chrome). No number in the README is
typed in by hand.

## Inputs

Seventeen workloads, built by `bench/data/make_workloads.py` with fixed seeds:

- Wikipedia: 60 articles of at least 2 KB in each of Arabic, Chinese, English,
  French, German, Hindi, Japanese, Korean, Russian, Spanish and Thai, drawn
  from the 2023-11-01 dump.
- Chat: 6 000 pieces of about 512 bytes cut from those articles and from code.
- Code: 22 of TokenDagger's source files, named in the script (two of them
  twice, as the first runs picked up copies from a build folder), and 256 KiB
  of the Go standard library.
- Books: 256 KiB each of *Moby-Dick* and 紅樓夢.
- TokenDagger's generated benchmark text, 1 MB. It repeats far more than real
  text, so it is in the full tables but not in any headline.

A headline is the geometric mean over the other sixteen workloads, in
`cl100k_base` and `o200k_base`: 32 rows, each weighted equally, whatever its
size.

## Timing

- One core, pinned with `taskset`, on an otherwise idle cloud VM.
- Every library is built with default compiler flags, as a user installing it
  gets it: no `target-cpu=native` anywhere. tiktoken-fast picks its AVX2 path
  at run time.
- A round times every library on the whole workload, one after another; the
  order rotates from round to round, so slow drift (thermal, frequency, a
  neighbour on the host) hits every library alike. 11 rounds.
- The speed-up of a round is the other library's time divided by
  tiktoken-fast's in that same round. A row is the geometric mean over its
  rounds, with a 95% bootstrap interval over those rounds. A headline is the
  geometric mean of its rows; its interval resamples every row's rounds
  together, 2000 times.
- The intervals cover noise within a run. How much a number moves between
  machines is what the per-machine table in the README shows.

## Scenarios

- **New text**: each document encoded once per round. Libraries with caches
  (tiktoken-fast, gpt-tokenizer) start each round empty and keep what they
  learn across that round's documents, as a server does.
- **Fresh caches**: those caches emptied before every document.
- **Same text**: the same documents again with warm caches; the first round is
  dropped. This favours caching libraries and is never a headline.

## What each library gets

| Language | Reference | Others | Call timed |
|---|---|---|---|
| Rust | tiktoken's core, through tiktoken-rs 0.12.1 (vendored verbatim) | bpe-openai 0.3.2 | `encode_ordinary` |
| Python | tiktoken 0.14.0 from PyPI | | `encode_ordinary`; `encode` with tiktoken's special-token check in the full table |
| JavaScript | tiktoken 1.0.22 (npm, WebAssembly) | gpt-tokenizer 4.1.0, js-tiktoken 1.0.21 | `encode_ordinary` or the library's equivalent, special tokens off |
| Go | pkoukk/tiktoken-go at 200cf60 (the fork point, vendored) | tiktoken-go/tokenizer 0.8.1 | `EncodeOrdinary` / `Encode` |

All packages are pinned: `Cargo.lock`, `bench/requirements.txt`,
`js/bench/package-lock.json`, `go/bench/go.sum`. Rust arms share one binary
built with the same profile. gpt-tokenizer's cache is emptied with its own
`clearMergeCache()` wherever tiktoken-fast's is.

## Correctness before speed

Before a workload is timed, every library's tokens for every document are
compared with the reference. A run in which tiktoken-fast differs on any
document fails. Other libraries are timed even where they differ, and the
summary lists how often: tiktoken-go/tokenizer returns different tokens on
some documents of most workloads.

## What these numbers do not show

- Multi-threaded throughput. Everything here is one core.
- Encoder build time and memory, which the README states separately.
- Other CPUs. The script has run on Intel (Xeon Platinum 8481C), AMD (EPYC
  9B14) and ARM (Google Axion) cloud servers; a headline moves by up to 1.27x
  between them ([table](benchmarks-machines.md)). Laptops and other cores are
  not measured.
- Cloud VMs share hosts. Rotation and pairing keep a slow neighbour from
  favouring one library, and `machine.json` records the stolen CPU time.
