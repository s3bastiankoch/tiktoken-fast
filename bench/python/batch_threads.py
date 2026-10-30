"""encode_ordinary_batch with 8 threads, not pinned: python batch_threads.py <dir> <workload ...>"""
import json, statistics, sys, time, tiktoken, tiktoken_fast
for w in sys.argv[2:]:
    docs = json.load(open(f"{sys.argv[1]}/{w}.json"))
    nb = sum(len(d.encode()) for d in docs)
    for name in ["cl100k_base", "o200k_base"]:
        a, b = tiktoken.get_encoding(name), tiktoken_fast.get_encoding(name)
        assert a.encode_ordinary_batch(docs) == b.encode_ordinary_batch(docs)
        ta, tb = [], []
        for r in range(9):
            s = time.perf_counter(); a.encode_ordinary_batch(docs, num_threads=8); ta.append(time.perf_counter() - s)
            b._core_bpe.reset_caches()
            s = time.perf_counter(); b.encode_ordinary_batch(docs, num_threads=8); tb.append(time.perf_counter() - s)
        ma, mb = statistics.median(ta), statistics.median(tb)
        print(f"{name:11} {w:18} batch x8, 4 vCPU: tiktoken {nb/ma/1e6:6.1f} MB/s  tiktoken-fast {nb/mb/1e6:6.1f} MB/s  {ma/mb:.2f}x", flush=True)
