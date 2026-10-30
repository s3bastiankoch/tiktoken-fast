# tiktoken-fast for Python

A drop-in for [tiktoken](https://github.com/openai/tiktoken) that returns the
same token ids, faster. It keeps tiktoken's Python layer, its encoding files and
model names, and replaces only the core that turns text into tokens.

```python
import tiktoken_fast as tiktoken

enc = tiktoken.get_encoding("o200k_base")
enc.encode("hello world")
enc.encode_batch(texts, num_threads=8)
```

Or, for code that already imports tiktoken:

```python
import tiktoken_fast
tiktoken_fast.patch()   # tiktoken.get_encoding and encoding_for_model now return fast encodings
```

Two additions that tiktoken does not have:

```python
enc.count_tokens(text)          # len(enc.encode(text)) without building the list
```

```sh
python -m tiktoken_fast.bench   # compare with tiktoken on this machine, about 15 s
```

The bench uses text every Python installation has (the standard library's
source and docstrings) unless you pass `--file`. It checks that every call
returns tiktoken's tokens before it times anything.

Every encoding tiktoken ships (`r50k_base`, `p50k_base`, `p50k_edit`,
`cl100k_base`, `o200k_base`, `o200k_harmony`) runs on the new core. An
`Encoding` built with a split pattern the core does not know keeps tiktoken's
core. `tiktoken_fast.Encoding` is a subclass of `tiktoken.Encoding`, so type
checks and pickling work as before.

## Speed

Against tiktoken 0.14 through the same Python calls, one core of a Xeon
Platinum 8481C, Python 3.11, 2026-10-26. Geometric means over Wikipedia in
eleven languages, 512-byte chat chunks, code and two books, under cl100k_base
and o200k_base; slowest workload in brackets. No document's tokens differed.
On AMD EPYC 9B14 and Google Axion `encode_ordinary()` is 3.78x and 3.50x
([all three machines](../README.md#numbers)).

| call | new text | fresh caches per document |
|---|---|---|
| `encode()` | **3.21x** [1.84x] | **2.64x** |
| `encode_ordinary()` | **3.28x** [1.80x] | **2.72x** |

Short texts gain more, because tiktoken spends most of a short call in Python:
`encode()` on 16-byte strings is 3.3x faster, and a batch of 16-byte strings
on four vCPUs 20x to 24x faster. Raw output is in
[`bench/results/2026-10-26-python`](../bench/results/2026-10-26-python).

## Build

```sh
pip install maturin
cd python && maturin build --release && pip install target/wheels/*.whl
python -m pytest tests
```

Needs Python 3.10 or later and a Rust toolchain. The tests compare every public
method with tiktoken.
