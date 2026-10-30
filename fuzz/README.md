# Fuzzing

One libFuzzer target, `diff`, compares this crate with the reference core
(tiktoken-rs) on arbitrary input. The first two bytes pick the encoding and
which calls to compare (`encode_ordinary`, `encode` with a random set of
allowed special tokens, `encode_with_special_tokens`, the parallel encoder at
a small chunk size, decoding); the rest is the text. The encoders persist
across inputs, so cache state builds up as it does in a long-running process;
every 4096th input drops the caches. `FUZZ_ENC=r50k|p50k|cl100k|o200k` pins
the encoding.

```sh
python3 fuzz/make_seeds.py            # writes fuzz/seeds/ and fuzz/tricky.dict
cargo +nightly fuzz run diff fuzz/seeds -- -dict=fuzz/tricky.dict -jobs=4
```

The two o200k bugs fixed before the first release were found this way; their inputs are
in `TRICKY` in `tests/parity.rs`.
