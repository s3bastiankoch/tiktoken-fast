"""Writes size-sweep workloads: for each kind of text and each size from 16 B
to 1 MiB in doubling steps, distinct consecutive chunks of that size.

    python make_sizes.py <main workload dir> <out dir>

Chunks end at a space near the target size where one exists, and always on a
character boundary. Each workload holds up to 4 MiB, at most 8192 documents and
at least 2; sizes with less text than two chunks are skipped.
"""
import json
import os
import sys

KINDS = {
    "en": ["wiki-en", "book-mobydick"],
    "zh": ["wiki-zh", "book-hongloumeng"],
    "ru": ["wiki-ru"],
    "code": ["td-code-files", "book-gosrc"],
}
SIZES = [16 << i for i in range(17)]


def chunks(b, size, n):
    out, i = [], 0
    while i < len(b) and len(out) < n:
        j = min(len(b), i + size)
        if j < len(b):
            k = b.rfind(b" ", i + size * 3 // 4, j)
            if k > i:
                j = k
            while j > i and (b[j] & 0xC0) == 0x80:
                j -= 1
        if j - i < size // 2:
            break
        out.append(b[i:j].decode())
        i = j
    return out


def main():
    src, out = sys.argv[1], sys.argv[2]
    os.makedirs(out, exist_ok=True)
    for kind, names in KINDS.items():
        b = "\n\n".join(t for n in names for t in json.load(open(os.path.join(src, n + ".json")))).encode()
        for s in SIZES:
            docs = chunks(b, s, min(8192, max(2, (4 << 20) // s)))
            if len(docs) < 2:
                continue
            json.dump(docs, open(os.path.join(out, f"{kind}-{s:07d}B.json"), "w"), ensure_ascii=False)
            print(kind, s, len(docs), sum(len(d.encode()) for d in docs))


if __name__ == "__main__":
    main()
