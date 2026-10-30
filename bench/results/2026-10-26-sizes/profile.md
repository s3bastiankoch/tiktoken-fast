# Where the time goes: o200k_base, English Wikipedia, new text

Xeon 8481C at 2.7 GHz, one core, `perf record` at 10 kHz (the VM exposes no
hardware counters, so stalls are read from annotated samples). 1.48 MB per
pass, caches dropped before each pass: 75.7 MB/s, 36 cycles per byte, 167 per
token. tiktoken-rs runs at 27 MB/s, so 10x would be 270 MB/s, 10 cycles per byte.

| share | where | what |
|---|---|---|
| 26% | segment loop (inlined into the caller) | cut bitmap, memo keys; about 30% of its samples sit on the memo probe load |
| 23% | `Cache::merge_core` | byte-pair merges of pieces seen for the first time; compute, flat scans |
| 17% | `Vocab::lookup` | whole-piece lookup; about 58% of its samples sit right after the table load |
| 9% | `encode_pieces` | scanning, dispatch |
| 4% | `merge_cached` | piece cache |
| 4% | backtracking | the few non-ASCII pieces |

Cache misses on table probes account for roughly a fifth of the time. Removing
all of them would give about 1.25x; removing all merges of new pieces as well
(a shipped table of frequent pieces) another 1.3x or so. That puts the ceiling
for this path near 150 MB/s, about 5.5x over tiktoken-rs, not 10x. More would
need a different design: SIMD splitting of whole runs of words and one probe
per word instead of per segment and per piece.
