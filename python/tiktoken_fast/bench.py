"""Compare tiktoken-fast with tiktoken on this machine.

    python -m tiktoken_fast.bench [--encoding NAME ...] [--file PATH ...] [--rounds N]

Without --file it uses text every Python installation has: the standard
library's source files (code) and their docstrings (English prose), whole and
cut into 512-byte chat-sized pieces. Each call is checked to return the same
tokens as tiktoken before it is timed. Rounds alternate the two libraries; the
table shows medians. tiktoken-fast starts every round with empty caches, so
the numbers are for text it has not seen; "fresh caches" also drops them
before every document, the worst case for it.
"""
from __future__ import annotations

import argparse
import ast
import os
import statistics
import sys
import sysconfig
import time

import tiktoken

import tiktoken_fast


def stdlib_texts(limit_bytes: int = 2_000_000) -> tuple[list[str], list[str]]:
    root = sysconfig.get_paths()["stdlib"]
    code, prose, size = [], [], 0
    for dirpath, dirnames, filenames in os.walk(root):
        dirnames[:] = sorted(d for d in dirnames if d not in ("test", "tests", "idlelib", "site-packages", "__pycache__"))
        for f in sorted(filenames):
            if not f.endswith(".py"):
                continue
            try:
                src = open(os.path.join(dirpath, f), encoding="utf-8").read()
                tree = ast.parse(src)
            except (OSError, UnicodeDecodeError, SyntaxError, ValueError):
                continue
            code.append(src)
            for node in ast.walk(tree):
                if isinstance(node, (ast.Module, ast.ClassDef, ast.FunctionDef, ast.AsyncFunctionDef)):
                    doc = ast.get_docstring(node)
                    if doc and len(doc) > 200:
                        prose.append(doc)
            size += len(src)
            if size > limit_bytes:
                return code, prose
    return code, prose


def chunks(texts: list[str], size: int = 512) -> list[str]:
    out = []
    for t in texts:
        for i in range(0, len(t), size):
            out.append(t[i : i + size])
    return out


def median_time(fn, rounds):
    ts = []
    for _ in range(rounds):
        s = time.perf_counter()
        fn()
        ts.append(time.perf_counter() - s)
    return statistics.median(ts)


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(prog="python -m tiktoken_fast.bench", description=__doc__.split("\n\n")[0])
    ap.add_argument("--encoding", action="append", help="default: cl100k_base and o200k_base")
    ap.add_argument("--file", action="append", help="a text file to use instead of the standard library; repeatable")
    ap.add_argument("--rounds", type=int, default=5)
    args = ap.parse_args(argv)

    if args.file:
        texts = [open(p, encoding="utf-8").read() for p in args.file]
        workloads = {"your files": texts, "your files, 512 B pieces": chunks(texts)}
    else:
        code, prose = stdlib_texts()
        workloads = {
            "English prose": prose,
            "Python source": code,
            "chat-sized, 512 B": chunks(prose + code)[:4000],
        }

    print(f"tiktoken {tiktoken.__version__}, tiktoken-fast {tiktoken_fast.__version__}, Python {sys.version.split()[0]}, {sys.platform}")
    rows = []
    for name in args.encoding or ["cl100k_base", "o200k_base"]:
        ref, fast = tiktoken.get_encoding(name), tiktoken_fast.get_encoding(name)
        reset = fast._core_bpe.reset_caches if fast._fast else (lambda: None)
        for wname, docs in workloads.items():
            docs = [d for d in docs if d]
            nbytes = sum(len(d.encode("utf-8")) for d in docs)
            ok = all(ref.encode(d, disallowed_special=()) == fast.encode(d, disallowed_special=()) for d in docs)
            if not ok:
                print(f"{name} {wname}: tokens differ from tiktoken; please report this")
                return 1
            # every tiktoken-fast round starts with empty caches, so its
            # numbers are for text it has not seen
            enc_r = lambda: [ref.encode(d, disallowed_special=()) for d in docs]

            def enc_f():
                reset()
                return [fast.encode(d, disallowed_special=()) for d in docs]

            def fresh():
                for d in docs:
                    reset()
                    fast.encode(d, disallowed_special=())

            cnt_r = lambda: [len(ref.encode(d, disallowed_special=())) for d in docs]
            def cnt_f():
                reset()
                return [fast.count_tokens(d, disallowed_special=()) for d in docs]

            bat_r = lambda: ref.encode_batch(docs, disallowed_special=())

            def bat_f():
                reset()
                return fast.encode_batch(docs, disallowed_special=())

            enc_r(), enc_f()
            pairs = [
                ("encode", enc_r, enc_f),
                ("encode, fresh caches", enc_r, fresh),
                ("count tokens", cnt_r, cnt_f),
                ("encode_batch, 8 threads", bat_r, bat_f),
            ]
            for cname, fr, ff in pairs:
                tr, tf = [], []
                for _ in range(args.rounds):
                    tr.append(median_time(fr, 1))
                    tf.append(median_time(ff, 1))
                r, f = statistics.median(tr), statistics.median(tf)
                rows.append((name, wname, cname, nbytes / r / 1e6, nbytes / f / 1e6, r / f))
                print(f"  {name:12} {wname:20} {cname:24} tiktoken {nbytes / r / 1e6:7.1f} MB/s   tiktoken-fast {nbytes / f / 1e6:7.1f} MB/s   {r / f:5.2f}x", flush=True)
    sp = [r[5] for r in rows]
    geo = 2 ** (sum(__import__("math").log2(x) for x in sp) / len(sp))
    print(f"\nsame tokens in every call; tiktoken-fast is {geo:.2f}x faster (geometric mean), {min(sp):.2f}x to {max(sp):.2f}x")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
