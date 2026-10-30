"""Writes the benchmark workloads as JSON lists of documents, one file each.

    python make_workloads.py --wiki WIKI --tokendagger TD --out WORK [--edge-out EDGE]

WIKI is the directory fetch_wikipedia.py wrote; TD a checkout of
github.com/M4THYOU/TokenDagger (its code files, its synthetic-text generator
and its Llama 4 vocabulary are used as they are). WORK receives the main set
and llama4.tiktoken; EDGE, when given, the low-repetition set.
"""
import argparse
import ast
import base64
import glob

import json
import os
import random
import textwrap
import urllib.request
import uuid

# The 22 files of TokenDagger 1eed815 behind the published workloads, in order.
# The first runs globbed a checkout that pip had built, which added copies of
# tokendagger/__init__.py and wrapper.py from build/; a clean clone has no
# build/, so the list is fixed here to keep the hashes in workloads.sha256.
TD_CODE_FILES = [
    "tokendagger/__init__.py", "tokendagger/wrapper.py",  # the build/ copies
    "build_tokendagger.py", "setup.py", "tests/autogenned_test.py",
    "tests/code_performance_benchmark.py", "tests/performance_benchmark.py",
    "tests/test_tokendagger_vs_tiktoken.py", "tests/throughput_test.py",
    "tokendagger/__init__.py", "tokendagger/wrapper.py", "src/main.cpp",
    "src/mistral_main.cpp", "src/py_binding.cpp", "src/tiktoken/tiktoken.cpp",
    "src/nlohmann/json.hpp", "src/tiktoken/hash_set8.hpp",
    "src/tiktoken/hash_table8.hpp", "src/tiktoken/tiktoken.hpp",
    "src/tokenizer_config.json", "throughput_llama_1024mb.json",
    "throughput_llama_256mb.json",
]

HERE = os.path.dirname(os.path.abspath(__file__))
CORPORA = os.path.join(HERE, "..", "..", "tests", "corpora")
MINIFIED = [
    "https://unpkg.com/terser@5.16.9/dist/bundle.min.js",
    "https://unpkg.com/date-fns@4.1.0/cdn.min.js",
]


def chunks(text, size):
    """Splits text into pieces of about size characters, at a space where one is near."""
    out, i = [], 0
    while i < len(text):
        j = min(len(text), i + size)
        k = text.rfind(" ", i + size // 2, j) if j < len(text) else j
        j = k if k > i else j
        out.append(text[i:j])
        i = j
    return out


def tokendagger_synthetic(td, size_mb):
    """TokenDagger's own generator for its throughput test, run as written."""
    src = open(os.path.join(td, "tests", "throughput_test.py")).read()
    fn = next(n for n in ast.walk(ast.parse(src)) if isinstance(n, ast.FunctionDef) and n.name == "generate_test_text")
    ns = {"random": random}
    exec(textwrap.dedent(ast.get_source_segment(src, fn)), ns)
    random.seed(0)
    return ns["generate_test_text"](None, size_mb)


def main_set(wiki, td, out):
    os.makedirs(out, exist_ok=True)
    w = {"td-synthetic-1MB": [tokendagger_synthetic(td, 1.0)]}
    code = [os.path.join(td, p) for p in TD_CODE_FILES]
    w["td-code-files"] = [open(p, encoding="utf-8", errors="replace").read() for p in code]
    for b in ["mobydick", "gosrc", "hongloumeng"]:
        w[f"book-{b}"] = [open(os.path.join(CORPORA, f"{b}.txt"), encoding="utf-8").read()]
    articles = []
    for d in sorted(glob.glob(os.path.join(wiki, "*"))):
        docs = [open(p, encoding="utf-8").read() for p in sorted(glob.glob(os.path.join(d, "*.txt")))]
        w[f"wiki-{os.path.basename(d)}"] = docs
        articles += docs
    msgs = [c for t in articles + w["td-code-files"] for c in chunks(t, 512)]
    random.Random(7).shuffle(msgs)
    w["chat-512B"] = msgs[:6000]
    for name, docs in w.items():
        json.dump(docs, open(os.path.join(out, name + ".json"), "w"))
    with open(os.path.join(out, "llama4.tiktoken"), "w") as f:
        for line in open(os.path.join(td, "src", "tokenizer.model"), encoding="utf-8"):
            if line.strip():
                f.write(line if line.endswith("\n") else line + "\n")
    print({k: (len(v), sum(len(t.encode()) for t in v)) for k, v in w.items()})


def edge_set(out):
    """Text with little repetition, where caches help least."""
    os.makedirs(out, exist_ok=True)
    rng = random.Random(3)

    def save(name, text, kib=64):
        n = kib * 1024
        json.dump([text[i:i + n] for i in range(0, len(text), n)], open(os.path.join(out, name + ".json"), "w"))

    raw = bytes(rng.getrandbits(8) for _ in range(600_000))
    save("adv-base64", base64.b64encode(raw).decode())
    save("adv-hex", raw[:400_000].hex())
    save("adv-uuid", "\n".join(str(uuid.UUID(int=rng.getrandbits(128))) for _ in range(20000)))

    def code_point():
        while True:
            c = rng.choice([rng.randrange(0x20, 0x7F), rng.randrange(0xA0, 0x800), rng.randrange(0x800, 0xD800),
                            rng.randrange(0xE000, 0x10000), rng.randrange(0x10000, 0x30000)])
            if not 0xD800 <= c < 0xE000:
                return chr(c)

    save("adv-random-unicode", "".join(code_point() if rng.random() > 0.15 else " " for _ in range(200_000)), 32)
    emoji = [chr(c) for c in range(0x1F300, 0x1FAFF)] + ["👨‍👩‍👧‍👦", "🏳️‍🌈", "👍🏽", "❤️"]
    save("adv-emoji", "".join(rng.choice(emoji) + (" " if rng.random() < 0.3 else "") for _ in range(100_000)), 32)
    records = [{"id": rng.getrandbits(40), "lat": rng.uniform(-90, 90), "lon": rng.uniform(-180, 180),
                "v": [round(rng.gauss(0, 1e3), 4) for _ in range(6)]} for _ in range(8000)]
    save("adv-numeric-json", json.dumps(records))
    js = "".join(urllib.request.urlopen(u, timeout=60).read().decode("utf-8", "replace") for u in MINIFIED)
    save("adv-minified-js", js)


if __name__ == "__main__":
    ap = argparse.ArgumentParser()
    ap.add_argument("--wiki", required=True)
    ap.add_argument("--tokendagger", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--edge-out")
    a = ap.parse_args()
    main_set(a.wiki, a.tokendagger, a.out)
    if a.edge_out:
        edge_set(a.edge_out)
