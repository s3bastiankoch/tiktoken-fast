"""Fetches the Wikipedia sample: N articles of at least 2 KB per language,
drawn with a fixed seed from the first row group of the first shard of the
wikimedia/wikipedia 20231101 dump on Hugging Face (no full download).

    python fetch_wikipedia.py <out dir> [N=60]

Needs pyarrow, fsspec and aiohttp.
"""
import os, sys, random, fsspec, pyarrow.parquet as pq
out, per = sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 60
rng = random.Random(1)
for lang in "en de fr es ru zh ja ko ar hi th".split():
    d = os.path.join(out, lang); os.makedirs(d, exist_ok=True)
    for f in os.listdir(d): os.remove(os.path.join(d, f))
    url = f"https://huggingface.co/datasets/wikimedia/wikipedia/resolve/main/20231101.{lang}/train-00000-of-"
    fs = fsspec.filesystem("https")
    n = None
    for total in range(1, 60):
        try:
            fs.info(url + f"{total:05d}.parquet"); n = total; break
        except Exception:
            pass
    pf = pq.ParquetFile(fs.open(url + f"{n:05d}.parquet", block_size=8 << 20))
    rows = pf.read_row_group(0, columns=["id", "text"]).to_pylist()
    rows = [r for r in rows if len(r["text"].encode()) >= 2000]
    for r in rng.sample(rows, min(per, len(rows))):
        open(os.path.join(d, f"{r['id']}.txt"), "w", encoding="utf-8").write(r["text"])
    print(lang, n, len(rows), flush=True)
