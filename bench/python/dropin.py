"""tiktoken against tiktoken-fast through the public Python API.

    python dropin.py <workload dir> [encoding ...]

Per workload and encoding: encode() (with tiktoken's default special-token
check) and encode_ordinary() document by document, in the scenarios of
bench/compare (stream: tiktoken-fast's caches dropped before each round; cold:
before each document); and encode_ordinary_batch() over the whole workload with
8 threads. Rounds rotate the order; a row is the median speed-up over tiktoken
with a 95% bootstrap interval. Documents whose tokens differ are counted first.

With RAW=<file>, every timed round and every check is appended to that file as
one JSON line for bench/summarize.py.
"""
import glob
import json
import os
import random
import statistics
import sys
import time

import tiktoken
import tiktoken_fast

ROUNDS = int(os.environ.get("ROUNDS", "11"))
rng = random.Random(7)
RAW = open(os.environ["RAW"], "a") if os.environ.get("RAW") else None


def raw(**kv):
    if RAW:
        RAW.write(json.dumps({"lang": "python", **kv}) + "\n")
        RAW.flush()


def boot_ci(xs, n=2000):
    meds = sorted(statistics.median(rng.choices(xs, k=len(xs))) for _ in range(n))
    return meds[int(0.025 * n)], meds[int(0.975 * n)]


def timed(fn, docs, reset=None, per_doc=False):
    clock = time.perf_counter
    if reset and not per_doc:
        reset()
    total = 0.0
    for d in docs:
        if per_doc and reset:
            reset()
        s = clock()
        fn(d)
        total += clock() - s
    return total


def row(label, nbytes, t_ref, t_fast, key=None):
    if key:
        for arm, ts in (("tiktoken", t_ref), ("tiktoken-fast", t_fast)):
            for r, x in enumerate(ts):
                raw(kind="time", arm=arm, round=r, seconds=x, bytes=nbytes, **key)
    sp = [x / y for x, y in zip(t_ref, t_fast)]
    lo, hi = boot_ci(sp)
    print(f"{label} tiktoken {nbytes / statistics.median(t_ref) / 1e6:6.1f} MB/s  tiktoken-fast {nbytes / statistics.median(t_fast) / 1e6:6.1f} MB/s  {statistics.median(sp):5.2f}x [{lo:.2f},{hi:.2f}]", flush=True)


def main():
    work, only = sys.argv[1], sys.argv[2:]
    loads = {os.path.basename(p)[:-5]: json.load(open(p)) for p in sorted(glob.glob(os.path.join(work, "*.json")))}
    for name in only or ["cl100k_base", "o200k_base"]:
        ref, fast = tiktoken.get_encoding(name), tiktoken_fast.get_encoding(name)
        reset = fast._core_bpe.reset_caches
        for wname, docs in loads.items():
            bad = sum(1 for d in docs if ref.encode(d, disallowed_special=()) != fast.encode(d, disallowed_special=()))
            nbytes = sum(len(d.encode()) for d in docs)
            raw(kind="check", enc=name, workload=wname, arm="tiktoken-fast", docs=len(docs), bytes=nbytes, mism=bad)
            calls = [("encode_ordinary", ref.encode_ordinary, fast.encode_ordinary)]
            try:
                for d in docs:
                    ref.encode(d)
                calls.insert(0, ("encode", ref.encode, fast.encode))
            except ValueError:
                # text that spells a special token: encode() raises in both, as it should
                pass
            for cname, fr, ff in calls:
                for sc in ["stream", "cold"] if len(docs) > 1 else ["stream"]:
                    t = {0: [], 1: []}
                    for r in range(ROUNDS):
                        for arm in ([0, 1] if r % 2 == 0 else [1, 0]):
                            if arm == 0:
                                t[0].append(timed(fr, docs))
                            else:
                                t[1].append(timed(ff, docs, reset, per_doc=(sc == "cold")))
                    row(f"{name:11} {wname:18} {cname:16} {sc:6}", nbytes, t[0], t[1],
                        dict(enc=name, workload=wname, scenario=sc, call=cname))
            if len(docs) >= 16:
                t = {0: [], 1: []}
                for r in range(ROUNDS):
                    for arm in ([0, 1] if r % 2 == 0 else [1, 0]):
                        s = time.perf_counter()
                        if arm == 0:
                            ref.encode_ordinary_batch(docs, num_threads=8)
                        else:
                            reset()
                            fast.encode_ordinary_batch(docs, num_threads=8)
                        t[arm].append(time.perf_counter() - s)
                row(f"{name:11} {wname:18} {'batch x8':16} {'stream':6}", nbytes, t[0], t[1],
                    dict(enc=name, workload=wname, scenario="stream", call="batch8"))
            print(f"{name:11} {wname:18} mismatches {bad}", flush=True)


if __name__ == "__main__":
    main()
