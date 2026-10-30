# How it works

tiktoken encodes in two steps. A regular expression splits the text into
pieces (words with their leading space, runs of digits, runs of punctuation,
whitespace), and each piece that is not itself a token is byte-pair merged:
start from single bytes and keep joining the adjacent pair whose joined bytes
form the lowest-ranked token, until no pair is a token. This crate returns the
same ids and changes how much of that work has to run, and how fast the rest
runs.

## Splitting without a regex engine

Each of the three split patterns the shipped encodings use (cl100k, o200k,
p50k/r50k) has a hand-written scanner in `src/scan.rs`. It reproduces the
regex engine's choices, including the leftmost-first alternation and the
places where a greedy run gives characters back. ASCII is classified from one
table, letter runs are checked eight bytes at a time, and the Unicode classes
(`\p{L}`, `\p{N}`, the upper- and lowercase-like classes of o200k, `\s`) are
bitmaps over the Basic Multilingual Plane built from the `regex-syntax`
crate's own tables, so they agree with the reference by construction.

A pattern without a scanner is an error (`Error::UnknownPattern`), never a
silent slow path.

## Segments and the memo

Most text repeats. The text is cut wherever every pattern resolves the same
way with or without the other side: a single space between two ASCII
letters, or ASCII punctuation right after a letter. A segment's tokens then
depend on its bytes alone. One AVX2 pass (eight bytes per step elsewhere)
marks the cuts in a bitmap. A segment of up to 15 bytes is answered from a
three-way set that fits one cache line, holding the keys and the first token;
16 to 31 bytes go to a second memo. Keys hold the full bytes, so a hash
collision cannot return wrong tokens.

## Vocabulary lookups

A packed open-addressing table answers a piece of up to eight bytes with one
load, a mask, one multiply and one 12-byte entry; longer pieces hash into a
second table over one byte arena. A 64K-entry table answers every two-byte
string directly.

## Merging

Three paths, chosen per piece:

- ASCII pieces are short. They merge in scratch arrays that stay in cache:
  parts form a linked list, the lowest rank is found by a flat scan the
  compiler vectorises, and a per-caller pair table remembers whether two
  adjacent tokens join. Merged pieces are cached.
- Pieces with characters beyond ASCII are first cut wherever the two
  bytes on either side of a boundary occur next to each other inside no
  token. No merge can then join the two sides, so they encode, and are
  cached, apart. Lone emoji between such cuts come from a precomputed table.
  What remains encodes by backtracking, after the
  [bpe](https://github.com/github/rust-gems/tree/main/crates/bpe) crate: take
  the longest token at the position, keep it if it forms a valid pair with
  the token before it, otherwise try the next shorter prefix token, and step
  back when nothing fits. A pair is valid when byte-pair encoding the two
  tokens' bytes gives back the same two tokens, which a split table and a
  table of all merges decide by unwinding the merges that built them. The
  longest-token search uses the vocabulary table itself: masks keyed by a
  token's first three and first six bytes say which lengths exist, and Bloom
  filters in front of the merge table and of the token strings keep most
  misses out of the large tables.
- Pieces with four-byte characters that the cuts leave whole take the
  ASCII path, where their mostly fragmentary tokens merge faster.

## Chinese, Japanese and Korean

These scripts have no spaces, so a clause is one piece and rarely repeats.
Byte-pair merging never joins across a position that no token spans. Every
place a token crosses a character boundary is recorded from the vocabulary
with up to three bytes of context on each side; per-character flags, a Bloom
filter and a per-caller verdict cache decide most boundaries with one load. A
span of three-byte characters is encoded in one pass that applies the
pattern's piece rules from a per-character class, cuts where safe, and writes
each character's precomputed tokens. A part produced by a cut gets its
byte-pair merge and only a whole piece the direct vocabulary match, as in the
reference.

## Caches

Each `CoreBPE` keeps a pool of cache sets, one per concurrent caller, so
calls never share mutable state. A fresh set holds no tables at all: the
first 512 bytes it encodes skip the memos, and the merge scratch arrays are
allocated on first use, so a short call on a fresh set costs about 80 ns of
setup. The tables then appear at 512 bytes of input and grow at 4 KiB and
64 KiB, so a long stream gets tables sized for its vocabulary. Tables are
allocated zeroed, so the operating system maps their pages only when they are
touched.

Segments of 32 bytes to 16 KiB, mostly runs without ASCII spaces (Thai or
Chinese paragraphs, long identifiers), go to a third memo keyed by a hash of
the whole segment. A segment is stored on its second sighting, so text that
never repeats pays one hash per segment, and the memo stays within 2 MB.

## JavaScript and Python

`js/` compiles the crate to WebAssembly with the API of the tiktoken npm
package; `python/` wraps it in tiktoken's own Python `Encoding`, so the
encoding files, model names and special-token checks are tiktoken's. Decode
sizes its output first and copies each token with one eight-byte store.

## Costs

- Building an encoder takes about 26 ms for cl100k_base and 46 ms for
  o200k_base on an Apple M-series laptop (tiktoken-rs: 44 ms and 90 ms;
  `examples/loadtime.rs`). The
  shipped vocabularies skip the check that every token encodes to itself (a
  unit test runs it), read their files without a map, and compute each
  three-byte character's merge from its two byte pairs. The backtracking
  tables wait until 1 MiB of non-ASCII pieces has been merged the plain way.
- The backtracking tables take about 8 MB (cl100k_base) and 16 MB
  (o200k_base). Peak memory of a process that builds an encoder and encodes
  one English article is 15 MB and 27 MB, against 28 MB and 52 MB for
  tiktoken-rs and 33 MB and 69 MB for bpe-openai; with 3.6 MB of Thai and
  Russian, which builds the backtracking tables, it is 25 MB and 57 MB,
  against 36/56 MB and 43/76 MB (`bench/mem`).
