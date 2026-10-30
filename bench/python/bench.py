"""Single-threaded comparison through Python: tiktoken, TokenDagger and
tiktoken-fast (the binding in ./binding), in one harness.

    python bench.py <workload dir> [encoding ...]

Encodings are cl100k_base, o200k_base and llama4 (from the workload dir's
llama4.tiktoken). Scenarios and statistics follow bench/compare: every round
times each arm on the whole workload in rotated order; a row is the median
speed-up over tiktoken with a 95% bootstrap interval, plus the number of
documents whose tokens differ from tiktoken's.
"""
import base64
import glob
import json
import os
import random
import statistics
import sys
import time

import tiktoken
import tkpy
from tokendagger import wrapper as tokendagger

ROUNDS = int(os.environ.get("ROUNDS", "11"))
rng = random.Random(7)


def encodings(work):
    for name in ["cl100k_base", "o200k_base"]:
        e = tiktoken.get_encoding(name)
        yield name, e._pat_str, e._mergeable_ranks
    ranks = {}
    for line in open(os.path.join(work, "llama4.tiktoken"), encoding="utf-8"):
        b, r = line.split()
        ranks[base64.b64decode(b)] = int(r)
    yield "llama4", tiktoken.get_encoding("o200k_base")._pat_str, ranks


def boot_ci(xs, n=2000):
    meds = sorted(statistics.median(rng.choices(xs, k=len(xs))) for _ in range(n))
    return meds[int(0.025 * n)], meds[int(0.975 * n)]


def time_docs(fn, docs, reset=None, per_call=False):
    if reset and not per_call:
        reset()
    total, clock = 0.0, time.perf_counter
    for t in docs:
        if per_call and reset:
            reset()
        s = clock()
        fn(t)
        total += clock() - s
    return total


def main():
    work, only = sys.argv[1], sys.argv[2:]
    loads = {os.path.basename(p)[:-5]: json.load(open(p)) for p in sorted(glob.glob(os.path.join(work, "*.json")))}
    for ename, pat, ranks in encodings(work):
        if only and ename not in only:
            continue
        tk = tiktoken.Encoding(f"tk-{ename}", pat_str=pat, mergeable_ranks=ranks, special_tokens={})
        td = tokendagger.Encoding(f"td-{ename}", pat_str=pat, mergeable_ranks=ranks, special_tokens={})
        us = tkpy.Encoding(ranks, {}, pat)
        arms = {"tiktoken": tk.encode_ordinary, "tokendagger": td.encode_ordinary, "ours": us.encode_ordinary}
        for wname, docs in loads.items():
            bad = {a: sum(1 for t in docs if f(t) != tk.encode_ordinary(t)) for a, f in arms.items() if a != "tiktoken"}
            nbytes = sum(len(t.encode()) for t in docs)
            for sc in ["stream", "repeat"] + (["cold"] if len(docs) > 1 else []):
                times = {a: [] for a in arms}
                names = list(arms)
                for r in range(ROUNDS + (sc == "repeat")):
                    for a in names[r % 3:] + names[:r % 3]:
                        reset = us.reset_caches if a == "ours" and sc != "repeat" else None
                        times[a].append(time_docs(arms[a], docs, reset, per_call=(sc == "cold")))
                    if sc == "repeat" and r == 0:
                        for a in arms:
                            times[a].pop()
                row = f"{ename:11} {wname:18} {sc:6} tiktoken {nbytes / statistics.median(times['tiktoken']) / 1e6:6.1f} MB/s"
                for a in ("tokendagger", "ours"):
                    sp = [x / y for x, y in zip(times["tiktoken"], times[a])]
                    lo, hi = boot_ci(sp)
                    row += f" | {a} {statistics.median(sp):5.2f}x [{lo:.2f},{hi:.2f}]"
                print(row + f" | mism {bad}", flush=True)


if __name__ == "__main__":
    main()
