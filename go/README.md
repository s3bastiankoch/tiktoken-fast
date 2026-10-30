# tiktoken-fast for Go

A fork of [pkoukk/tiktoken-go](https://github.com/pkoukk/tiktoken-go)
(formerly published as tiktoken-go-fast) with a hand-written pre-tokeniser in
place of the backtracking regex engine, a packed vocabulary table in place of
the Go map, and a merge loop that allocates nothing and remembers what it has
merged before. Byte-identical output, two allocations per call.

On text it has not seen, measured like the rest of this repository
(`bench/fair_test.go`, run by [`bench/reproduce.sh`](../bench/reproduce.sh):
Wikipedia in eleven languages, code, books and chat, one core), with every
document's tokens checked first:

| | Intel Xeon 8481C | AMD EPYC 9B14 | Google Axion (ARM) |
|---|---:|---:|---:|
| over tiktoken-go | **6.44x** | **6.80x** | **6.72x** |
| over tiktoken-go/tokenizer | 4.37x | 4.29x | 4.17x |
| slowest workload (紅樓夢), over tiktoken-go | 2.69x | 2.94x | 2.99x |

Geometric means over 32 rows with 95% intervals of about ±2%, from the
[Intel](../bench/results/2026-10-26-c3-standard-4-a70b9ff/summary.md),
[AMD](../bench/results/2026-10-27-c3d-standard-4-3f3699c/summary.md) and
[ARM](../bench/results/2026-10-27-c4a-standard-4-3f3699c/summary.md) runs.
An earlier run with a different harness gave 7.2x
([`bench/results-fair-2026-10-27.txt`](bench/results-fair-2026-10-27.txt)).

The figures below repeat the same text in a hot loop, so they show the warm
case: 10x on English prose, 20x on generated prose, 28-31x on source code.

```go
import tiktoken "github.com/s3bastiankoch/tiktoken-fast/go"

enc, _ := tiktoken.GetEncoding("o200k_base")
ids := enc.Encode("hello world", nil, nil) // [24912 2375]
```

Not affiliated with or endorsed by the upstream project. Forked at
[`200cf60`](https://github.com/pkoukk/tiktoken-go/commit/200cf60), MIT licence
retained.

## Numbers

Measured on a GCP `c3-standard-8` (Xeon 8481C, Debian 12, Go 1.26), 2026-10-25.
Not a single run: the fork-point implementation is linked into the same binary
and both arms are measured round by round, so machine drift cancels instead of
landing on whichever arm ran while the machine was busy. Median of 31 rounds,
interquartile range in brackets. Raw output in
[`bench/results-gcp-2026-10-25.txt`](bench/results-gcp-2026-10-25.txt); see
[`bench/`](bench/) for the harness.

Speed, as paired median ratios against the fork-point implementation:

| corpus | cl100k_base | o200k_base |
|---|---|---|
| 256 KiB of *Moby-Dick* | **9.3x** [8.0-10.7] | **9.6x** [8.5-10.0] |
| 256 KiB of Go stdlib source | 11.0x [10.9-12.1] | 15.3x [13.4-15.6] |
| generated `prose_4k` | 21.2x [20.0-21.4] | 23.5x [22.7-23.7] |
| generated `prose_4m` | 19.7x [18.3-19.9] | 22.0x [21.2-22.5] |
| generated `code_4k` | 31.2x [30.9-31.9] | 30.2x [30.1-30.6] |
| UDHR, Chinese / Japanese / Korean | 13.5x / 5.8x / 13.2x | 16.8x / 7.2x / 18.6x |
| 256 KiB of 紅樓夢 | 3.1x [2.8-3.5] | 3.0x [2.8-3.6] |

The exported `Encode()` now runs within one percent of `EncodeOrdinary` on every
workload: its special-token scan is a byte search instead of a regexp2 pass over
a rune slice.

Allocation, as absolute `go test -bench` counts, which are load-independent and
need no pairing (cl100k_base, `prose_4k`):

| | fork point | this fork | |
|---|---|---|---|
| bytes allocated | 427 120 B/op | 22 045 B/op | **19x less** |
| allocations | 5 022 /op | 2 /op | **2 500x fewer** |

The two allocations are the result slice and the pointer-free cache a
goroutine borrows from a pool and returns.

### What changed

- **Vocabulary lookups** go through a packed open-addressing table instead of
  `map[string]int`. Pieces of up to eight bytes are looked up by their bytes
  packed into one integer, one multiply for the hash, one 16-byte entry per
  probe; longer pieces hash into a second table over one byte arena.
- **The merge loop** keeps boundaries, ranks and part tokens in scratch arrays
  it borrows, so a piece costs no allocation, and asks a pair table whether two
  adjacent tokens merge before it touches the vocabulary. Pieces longer than 40
  bytes merge through a heap instead of a scan, so continuous CJK is n log n
  rather than quadratic.
- **A piece cache** remembers the tokens of pieces that missed the vocabulary.
  Running text repeats its words; the cache is direct-mapped, pointer-free and
  per goroutine.
- **Character classes** for the Basic Multilingual Plane are bitmaps, one bit per
  code point, built once from the `unicode` tables; the scanner no longer binary
  searches range tables for every non-ASCII rune.
- **Special tokens** are found with `strings.Index`.

### Per encoding

All five shipped encodings take the fast path. Between them they carry four
vocabularies (`p50k_edit` is `p50k_base`'s vocabulary plus three special tokens)
and three distinct pre-tokenisation patterns. The pattern is what the scanner is
keyed on: it is compiled from the pattern string at construction time rather
than hard-coded for one encoding.

Every figure below is measured by `bench/`, which links the fork-point
implementation (`200cf60`, vendored verbatim from upstream under
[`bench/internal/upstream`](bench/internal/upstream)) into the same binary and
interleaves the two arms round by round. Median of 31 rounds on the GCP host:

| encoding | models | generated `prose_4k` | 256 KiB of *Moby-Dick* |
|---|---|---|---|
| `o200k_base` | **gpt-4o, gpt-4.1, gpt-5.x, o-series** | **23.7x** | **10.4x** |
| `cl100k_base` | gpt-4, gpt-3.5-turbo, embeddings | 21.3x | 9.9x |
| `p50k_base` | text-davinci-002/003, codex | 19.8x | 10.4x |
| `r50k_base` | gpt-3, davinci | 19.5x | 10.0x |

Moby-Dick is the number to quote for English prose, and it sits at the edge of
10x with an interquartile range of about 8x to 11x: at 256 KiB one call fills a
whole measurement burst, so the arm starts every round with cold caches after the
other arm has run. Measured sequentially on the same host, the same corpus
encodes 13.6x faster than the fork point. Generated prose runs 20x because its
fragment list repeats more than real text does and the piece cache catches it.

CJK is the weak case, as before: 3x on continuous Chinese, 6-7x on Japanese,
13-19x on Chinese and Korean with spaces and punctuation. Real CJK text misses
the vocabulary and lands in the merge, where the pair table helps but every
piece is new.

### Against the other Go port

Measured against [tiktoken-go/tokenizer](https://github.com/tiktoken-go/tokenizer)
v0.8.1, medians of 31 interleaved rounds on the GCP host, identical token IDs
gated before any timing is read:

| corpus | `o200k_base` | `cl100k_base` |
|---|---|---|
| 256 KiB of Go stdlib source | **7.0x** | **6.2x** |
| 256 KiB of *Moby-Dick* | **4.0x** | **4.1x** |
| generated `prose_4k` | 11.0x | 10.7x |
| 256 KiB of 紅樓夢 | 2.0x | 2.3x |
| UDHR, Japanese | 5.6x | 4.7x |

These are real corpora, vendored under `bench/testdata/`; the generated workloads
in the same harness go further and their extremes are artefacts of their
fragment lists. The ratios hold at scale: from 256 KiB to 4 MiB, prose stays at
10-12x against tokenizer and 18-23x against the fork point, so nothing here is a
fixed per-call cost being amortised. Method, caveats and the full tables are in
[`bench/`](bench/).

### Against the Rust reference

openai/tiktoken's own Rust `CoreBPE`, reached through
[tiktoken-rs](https://crates.io/crates/tiktoken-rs) v0.12.1, which vendors that
core verbatim and ships the same vocabularies. Both sides run the same entry
point, `EncodeOrdinary` against `encode_ordinary`, ids in and ids out, with
identical token IDs gated before any timing. Medians of 31 interleaved rounds on
the GCP host. The Rust arm is built `--release` with fat LTO, one codegen unit
and `-C target-cpu=native`, which Go has no equivalent of, so the build
asymmetry runs against this fork:

| corpus | `o200k_base` | `cl100k_base` |
|---|---|---|
| 256 KiB of Go stdlib source | **3.3x** | **6.8x** |
| 256 KiB of *Moby-Dick* | **2.7x** | **4.6x** |
| 4 KiB of generated prose | 4.5x | 8.4x |
| UDHR, Chinese / Japanese / Korean | 2.3x / 2.3x / 4.2x | 4.8x / 2.4x / 5.1x |
| 256 KiB of 紅樓夢 | 1.1x | 1.5x |

**Every cell is faster than the reference implementation**, 2.7-6.8x on English
prose and source code and 1.1-5.1x on CJK, where the earlier revision of this
fork ran up to 2.4x slower than Rust. The merge is what changed: the reference
hashes every candidate pair through `FxHashMap` and rebuilds its part list per
merge, while this fork asks a pair table, keeps parts in borrowed arrays and
merges long pieces through a heap. Tables, method and the harness are in
[`bench/vsrust/`](bench/vsrust/); raw output in
[`bench/vsrust/results-gcp-2026-10-25.md`](bench/vsrust/results-gcp-2026-10-25.md).

## Correctness

The upstream implementation is the oracle. `equiv_test.go` keeps a verbatim copy
of the original `bytePairMerge` and `encodeOrdinaryNative` from the fork point;
`allenc_test.go` runs that comparison on every shipped encoding, and `bench/`
adds a second, independent implementation as a witness:

- 1 683 810 fuzz executions (`FuzzEncodeOrdinaryAllEncodings`, native
  `go test -fuzz`), each input compared against the frozen implementation under
  all four vocabularies. 6.7 M comparisons, zero divergences.
- All four vocabularies on 322 texts each: CJK, emoji and ZWJ sequences, mixed
  scripts, code, contractions in both cases, CRLF, full-width forms, and
  adversarial rune boundaries. `p50k_base` and `r50k_base` share the GPT-2
  pattern, so three scanners cover the four.
- Invalid UTF-8 and special-token segmentation, the two input classes the fuzz
  target skips: encoding around `<|endoftext|>` must equal encoding each segment
  ordinarily with the special id spliced in.
- An independent port: `bench/` asserts byte-identical token IDs against
  [tiktoken-go/tokenizer](https://github.com/tiktoken-go/tokenizer) on 17 corpora
  under four encodings before it reads a single timing number.
- The allocation-free merge, in both its scanning and its heap form, against
  the upstream merge loop on 3 000 random pieces of bytes, ASCII words and CJK
  runs, cold and with warm caches (`merge_test.go`).
- Decode round-trip invariant, plus the upstream test suite unchanged.

A custom pattern passed to `NewCoreBPE` is matched exactly against the three
known ones and otherwise takes the regexp2 path, so an encoding this fork has
never seen is slow rather than wrong. That fallback is itself tested.

```sh
go test ./...                                              # gate
go test -run xxx -fuzz FuzzEncodeOrdinaryAllEncodings      # differential fuzzing
cd bench && go test .                                      # vs an independent port
```

## Benchmarks you can trust

Upstream ships `BenchmarkEncoding` in
[`benchmark_test.go`](https://github.com/pkoukk/tiktoken-go/blob/200cf60/benchmark_test.go),
which never reads `b.N` and never resets the timer: it encodes a fixed text four
times, at 1x, 10x, 100x and 1000x its length, and `fmt.Printf`s each size to
stdout. `go test -bench` then divides whatever that cost by a `b.N` the function
ignored. Every published figure for this library traces back to it.

That file is not carried over here. `bench_test.go` replaces it. (The one
upstream benchmark this fork does keep, `test/benchmark_test.go`, is a different
function and does drive `b.N` correctly.)

`bench_test.go` provides `b.N`-driven benchmarks over held-out workloads chosen
to separate the cost terms: input length, script (ASCII vs 3-byte CJK, where
rune-indexing pays triple), punctuation density (regex backtracking), and
unbroken word length (the quadratic merge).

```sh
go test -run xxx -bench . -benchtime 2s
```

Vocabulary comes from [tiktoken-go-loader](https://github.com/pkoukk/tiktoken-go-loader)'s
embedded assets, so benchmarks and tests need no network and see identical ranks
on every run.

## Models

`EncodingForModel` follows upstream openai/tiktoken's `tiktoken/model.py`
(checked 2026-08-22), including the prefix rules that make new releases work
without a library update:

```go
enc, _ := tiktoken.EncodingForModel("gpt-5.5")   // o200k_base, no table entry needed
```

The `"gpt-5"` prefix carries no trailing dash, so every gpt-5 point release and
size variant resolves. `o1-`, `o3-`, `o4-mini-`, `chatgpt-4o-`, the Azure
`gpt-35-turbo-` names and the `ft:` fine-tune prefixes are covered too.

Two things differ from a naive port of upstream's table. The first is that the
longest prefix wins. Upstream relies on Python dict insertion order to put
`ft:gpt-4o` ahead of `ft:gpt-4`; Go randomises map iteration, so the prefixes
are sorted by length at init instead. Without that, a fine-tuned gpt-4o model
resolves to cl100k_base on roughly half of all runs.

The second is that `gpt-oss-` is deliberately absent. It needs `o200k_harmony`,
which this library does not ship. Falling through to a "no encoding for model"
error beats returning confident, wrong token counts.

## Using it

Either as a normal dependency:

```sh
go get github.com/s3bastiankoch/tiktoken-fast/go
```

```go
import tiktoken "github.com/s3bastiankoch/tiktoken-fast/go"
```

The package is named `tiktoken`, not `tiktokengofast`, so the import above is
spelled explicitly. goimports will do it for you.

Or as a drop-in for an existing `pkoukk/tiktoken-go` dependency, without touching
a single import statement:

```
// go.mod
require github.com/pkoukk/tiktoken-go v0.1.7
replace github.com/pkoukk/tiktoken-go => github.com/s3bastiankoch/tiktoken-fast/go v0.1.0
```

Go resolves imports of the replaced path to this module, so existing
`import "github.com/pkoukk/tiktoken-go"` lines keep compiling and pick up the
scanner. Note that `replace` applies only to the main module: it is ignored in
anything that depends on *you*, so a library should import this module directly
rather than relying on a replace its own consumers will not inherit.

One API note carried over unchanged: `Encode` panics when the text contains a
special token listed in its `disallowedSpecial` argument. Passing `nil` for both
lists, or calling `EncodeOrdinary`, never panics. That is upstream's contract and
this fork keeps it, so a drop-in replacement stays drop-in. Wrap the call if a
panic on untrusted input is not acceptable in your caller.

## Licence

MIT, as upstream. `LICENSE` is unmodified, so copyright remains with the
original authors. The scanner, harness and benchmarks added here are offered
under the same terms. Upstream's own README is kept as `README-upstream.md`.
