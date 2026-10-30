"""Turns the raw rounds of one or more benchmark runs into the published tables.

    python bench/summarize.py <run dir> [<run dir> ...] [--combined OUT.md]

A run directory holds raw/*.jsonl, written by the harnesses with RAW=<file>,
and machine.json, written by bench/fingerprint.sh. For each run this writes
summary.md and summary.json next to them.

A row is one encoding, workload, scenario and call. Its speed-up is the
geometric mean over rounds of the paired ratio (the other library's time in a
round divided by tiktoken-fast's time in the same round), with a 95% bootstrap
interval over rounds. A headline is the geometric mean of the rows over
HEADLINE_WORKLOADS in cl100k_base and o200k_base; its interval comes from
resampling every row's rounds together, 2000 times. The interval covers noise within the run, not differences
between machines; compare runs for that (--combined).

Exits with status 1 if tiktoken-fast returned different tokens from the
reference on any document.
"""
import collections
import glob
import json
import math
import os
import random
import statistics
import sys

ENCODINGS = ["cl100k_base", "o200k_base"]
WIKI = [f"wiki-{l}" for l in "ar de en es fr hi ja ko ru th zh".split()]
# Text people send to a tokenizer. td-synthetic-1MB, TokenDagger's generated
# benchmark text, repeats itself far more than real text and is reported in the
# full table only.
HEADLINE_WORKLOADS = WIKI + ["chat-512B", "td-code-files", "book-gosrc", "book-mobydick", "book-hongloumeng"]
SCENARIOS = {"stream": "new text", "cold": "fresh caches", "repeat": "same text"}

# (label, lang, call, ours, other, other's description)
COMPARISONS = [
    ("Rust", "rust", "encode_ordinary", "tiktoken-fast", "tiktoken-rs", "tiktoken's core (tiktoken-rs 0.12.1)"),
    ("Rust", "rust", "encode_ordinary", "tiktoken-fast", "bpe-openai", "bpe-openai 0.3.2"),
    ("Python", "python", "encode_ordinary", "tiktoken-fast", "tiktoken", "tiktoken 0.14.0"),
    ("Python, encode()", "python", "encode", "tiktoken-fast", "tiktoken", "tiktoken 0.14.0"),
    ("JavaScript", "js", "encode_ordinary", "tiktoken-fast", "tiktoken", "tiktoken 1.0.22 (npm)"),
    ("JavaScript", "js", "encode_ordinary", "tiktoken-fast", "gpt-tokenizer", "gpt-tokenizer 4.1.0"),
    ("JavaScript", "js", "encode_ordinary", "tiktoken-fast", "js-tiktoken", "js-tiktoken 1.0.21"),
    ("Node, native", "js", "encode_ordinary", "tiktoken-fast-native", "tiktoken", "tiktoken 1.0.22 (npm)"),
    ("Node, native", "js", "encode_ordinary", "tiktoken-fast-native", "gpt-tokenizer", "gpt-tokenizer 4.1.0"),
    ("Go", "go", "encode_ordinary", "tiktoken-fast", "tiktoken-go", "pkoukk/tiktoken-go (200cf60, vendored)"),
    ("Go", "go", "encode_ordinary", "tiktoken-fast", "tokenizer", "tiktoken-go/tokenizer 0.8.1"),
]
OURS = {"tiktoken-fast", "tiktoken-fast-native"}
B = 2000


def geomean(xs):
    return math.exp(sum(map(math.log, xs)) / len(xs))


def pct(xs, q):
    s = sorted(xs)
    return s[min(len(s) - 1, int(q * len(s)))]


def load(run):
    times = collections.defaultdict(dict)  # (lang, enc, workload, scenario, call, arm) -> {round: s}
    checks = []
    for p in sorted(glob.glob(os.path.join(run, "raw", "*.jsonl"))):
        for line in open(p):
            r = json.loads(line)
            if r["kind"] == "time":
                times[(r["lang"], r["enc"], r["workload"], r["scenario"], r["call"], r["arm"])][r["round"]] = r["seconds"]
            else:
                checks.append(r)
    return times, checks


def rows_for(times, lang, call, ours, other):
    """Paired per-round speed-ups for every row that has both arms."""
    out = {}
    for (lg, enc, wl, sc, cl, arm), t_ours in times.items():
        if (lg, cl, arm) != (lang, call, ours):
            continue
        t_other = times.get((lg, enc, wl, sc, cl, other))
        if not t_other:
            continue
        rs = sorted(set(t_ours) & set(t_other))
        out[(enc, wl, sc)] = [t_other[r] / t_ours[r] for r in rs]
    return out


def row_stats(sp, rng):
    meds = [geomean(rng.choices(sp, k=len(sp))) for _ in range(B)]
    return geomean(sp), pct(meds, 0.025), pct(meds, 0.975)


def headline(rows, scenario, rng):
    sel = [sp for (enc, wl, sc), sp in rows.items() if sc == scenario and enc in ENCODINGS and wl in HEADLINE_WORKLOADS]
    if not sel:
        return None
    point = geomean([geomean(sp) for sp in sel])
    boots = [geomean([geomean(rng.choices(sp, k=len(sp))) for sp in sel]) for _ in range(B)]
    worst = min((geomean(sp), enc, wl) for (enc, wl, sc), sp in rows.items()
                if sc == scenario and enc in ENCODINGS and wl in HEADLINE_WORKLOADS)
    slower = sum(1 for sp in sel if geomean(sp) < 1)
    return dict(point=point, lo=pct(boots, 0.025), hi=pct(boots, 0.975), rows=len(sel), slower=slower,
                worst=dict(speedup=worst[0], enc=worst[1], workload=worst[2]))


def fmt(h):
    return f"**{h['point']:.2f}x** [{h['lo']:.2f}, {h['hi']:.2f}]" if h else ""


def summarize(run):
    rng = random.Random(7)
    times, checks = load(run)
    machine = json.load(open(os.path.join(run, "machine.json"))) if os.path.exists(os.path.join(run, "machine.json")) else {}
    bad = [c for c in checks if c["arm"] in OURS and c["mism"]]
    ndocs = sum(c["docs"] for c in checks if c["arm"] in OURS)
    result = dict(run=os.path.basename(os.path.abspath(run)), machine=machine, documents_checked=ndocs,
                  mismatches=len(bad), headlines=[], rows=[])
    for label, lang, call, ours, other, desc in COMPARISONS:
        rows = rows_for(times, lang, call, ours, other)
        if not rows:
            continue
        hl = {sc: headline(rows, sc, rng) for sc in SCENARIOS}
        result["headlines"].append(dict(label=label, lang=lang, call=call, ours=ours, other=other, other_desc=desc, **hl))
        for (enc, wl, sc), sp in sorted(rows.items()):
            m, lo, hi = row_stats(sp, rng)
            result["rows"].append(dict(label=label, other=other, enc=enc, workload=wl, scenario=sc, rounds=len(sp),
                                       speedup=m, lo=lo, hi=hi, headline=enc in ENCODINGS and wl in HEADLINE_WORKLOADS))
    comp = collections.defaultdict(dict)
    for c in checks:
        if c["arm"] not in OURS:
            comp[(c["lang"], c["arm"])][(c["enc"], c["workload"])] = (c["mism"], c["docs"])
    result["competitor_mismatches"] = {f"{l}/{a}": sum(m for m, _ in v.values()) for (l, a), v in comp.items()}

    m = machine
    nwl = len({c["workload"] for c in checks if c["workload"] in HEADLINE_WORKLOADS})
    lines = [f"# Benchmark run {result['run']}", ""]
    if m:
        lines += [f"- Machine: {m.get('cpu', '?')}, {m.get('instance', '?')}, kernel {m.get('kernel', '?')}",
                  f"- Commit {m.get('commit', '?')}{' (dirty)' if m.get('dirty') else ''}, started {m.get('started', '?')}",
                  f"- Pinned to core {m.get('core', '?')}; CPU steal during the run: {m.get('steal_pct', '?')}%",
                  f"- Toolchains: {', '.join(f'{k} {v}' for k, v in m.get('versions', {}).items())}", ""]
    nbad = sum(c["mism"] for c in bad)
    lines += [f"Token check: **{ndocs} documents** compared with the reference before timing, "
              f"**{nbad} with different tokens**." if not bad else
              f"Token check FAILED: {len(bad)} workloads differ: " + ", ".join(f"{c['lang']} {c['enc']} {c['workload']}" for c in bad), ""]
    lines += ["## Headline", "",
              f"Geometric mean over {nwl} workloads × {len(ENCODINGS)} encodings, 95% interval in brackets.", "",
              "| | over | new text | fresh caches | same text | slowest row (new text) | rows slower |", "|---|---|---|---|---|---|---:|"]
    for h in result["headlines"]:
        w = h["stream"]["worst"] if h["stream"] else None
        lines.append(f"| {h['label']} | {h['other_desc']} | {fmt(h['stream'])} | {fmt(h['cold'])} | {fmt(h['repeat'])} | "
                     + (f"{w['speedup']:.2f}x {w['enc'].split('_')[0]} {w['workload']}" if w else "") + " | "
                     + (f"{h['stream']['slower']} of {h['stream']['rows']}" if h["stream"] else "") + " |")
    if result["competitor_mismatches"]:
        lines += ["", "Documents where another library's tokens differ from its reference (timed anyway): "
                  + ", ".join(f"{k} {v}" for k, v in sorted(result["competitor_mismatches"].items())) + "."]
    lines += ["", "## Every row", "", "Rows marked * are outside the headline.", "",
              "| | over | encoding | workload | scenario | rounds | speed-up | 95% interval |", "|---|---|---|---|---|---:|---:|---|"]
    for r in result["rows"]:
        lines.append(f"| {r['label']} | {r['other']} | {r['enc']} | {r['workload']}{'' if r['headline'] else ' *'} | {r['scenario']} | "
                     f"{r['rounds']} | {r['speedup']:.2f}x | [{r['lo']:.2f}, {r['hi']:.2f}] |")
    open(os.path.join(run, "summary.md"), "w").write("\n".join(lines) + "\n")
    json.dump(result, open(os.path.join(run, "summary.json"), "w"), indent=1)
    return result


def combined(results, out):
    keys = [(h["label"], h["other_desc"]) for h in results[0]["headlines"]]
    head = "| | over | " + " | ".join(r["machine"].get("short", r["run"]) for r in results) + " | spread |"
    lines = ["New text, geometric mean and 95% interval per machine; spread is the largest over the smallest.", "",
             head, "|---|---|" + "---|" * len(results) + "---:|"]
    for k in keys:
        vals = []
        for r in results:
            h = next((h for h in r["headlines"] if (h["label"], h["other_desc"]) == k), None)
            vals.append(h["stream"] if h else None)
        pts = [v["point"] for v in vals if v]
        lines.append(f"| {k[0]} | {k[1]} | " + " | ".join(fmt(v) for v in vals)
                     + f" | {max(pts) / min(pts):.2f} |" if pts else " |")
    open(out, "w").write("\n".join(lines) + "\n")


if __name__ == "__main__":
    args = sys.argv[1:]
    out = None
    if "--combined" in args:
        i = args.index("--combined")
        out = args[i + 1]
        del args[i:i + 2]
    results = [summarize(r) for r in args]
    for r in results:
        print(f"{r['run']}: {r['documents_checked']} documents checked, {r['mismatches']} mismatching workloads")
        for h in r["headlines"]:
            print(f"  {h['label']:18} over {h['other']:14} new text {fmt(h['stream'])}")
    if out:
        combined(results, out)
    sys.exit(1 if any(r["mismatches"] for r in results) else 0)
