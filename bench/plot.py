"""Draws the README charts from one results directory of run_all.sh.

    python plot.py <results dir> <out dir>

Writes speedup.svg and speedup-dark.svg (by kind of text, new text, o200k_base)
and scenarios.svg and scenarios-dark.svg (all rows, by scenario). Speed-ups are
geometric means of per-workload medians, over tiktoken: tiktoken-rs for the
Rust arms, the tiktoken package for TokenDagger, which only has a Python API.
"""
import math
import os
import re
import sys

GROUPS = [
    ("English prose", ["wiki-en", "book-mobydick"]),
    ("Source code", ["td-code-files", "book-gosrc"]),
    ("Chinese", ["wiki-zh", "book-hongloumeng"]),
    ("Japanese, Korean", ["wiki-ja", "wiki-ko"]),
    ("German, French, Spanish, Russian", ["wiki-de", "wiki-fr", "wiki-es", "wiki-ru"]),
    ("Arabic, Hindi, Thai", ["wiki-ar", "wiki-hi", "wiki-th"]),
    ("Chat-sized messages", ["chat-512B"]),
    ("Low-repetition text", ["adv-"]),
]
ARMS = [("tiktoken-fast", "ours"), ("bpe-openai", "bpe"), ("TokenDagger", "td")]
THEMES = {
    "light": dict(bg="#ffffff", ink="#0e0e11", muted="#6d6d75", hair="#e2e2e5", ours="#c50e1f", bpe="#8b8b93", td="#c9c9ce"),
    "dark": dict(bg="#0e0e11", ink="#f2f2f3", muted="#a0a0a8", hair="#2c2c31", ours="#ff8a94", bpe="#8b8b93", td="#4a4a51"),
}
SANS = "Mulish, 'Helvetica Neue', Helvetica, Arial, sans-serif"
MONO = "'IBM Plex Mono', ui-monospace, SFMono-Regular, Menlo, monospace"


def gm(v):
    return math.exp(sum(map(math.log, v)) / len(v)) if v else None


def parse(results):
    """(arm, encoding, workload, scenario) -> speed-up over tiktoken."""
    rows = {}
    for name in ("rust-main.txt", "rust-edge.txt"):
        for line in open(os.path.join(results, name)):
            m = re.match(r"(\S+)\s+(\S+)\s+(\w+)\s.*?ours\s+([\d.]+)x", line)
            if not m:
                continue
            e, w, sc, o = m.groups()
            rows[("ours", e, w, sc)] = float(o)
            b = re.search(r"bpe-openai\s+([\d.]+)x", line)
            if b:
                rows[("bpe", e, w, sc)] = float(b.group(1))
    for line in open(os.path.join(results, "python-main.txt")):
        m = re.match(r"(\S+)\s+(\S+)\s+(\w+)\s.*?tokendagger\s+([\d.]+)x", line)
        if m:
            e, w, sc, t = m.groups()
            rows[("td", e, w, sc)] = float(t)
    return rows


def pick(rows, arm, enc, scen, prefixes):
    return [v for (a, e, w, s), v in rows.items()
            if a == arm and e == enc and s == scen and any(w == p or (p.endswith("-") and w.startswith(p)) for p in prefixes)]


def bars_svg(title, subtitle, labels, series, theme, note):
    """Horizontal grouped bars on a log axis; series is [(name, key, [value or None per label])]."""
    t = THEMES[theme]
    left, right, top = 300, 70, 116
    row_h, bar_h, gap = 46, 10, 3
    width = 1000
    plot_w = width - left - right
    height = top + row_h * len(labels) + 84
    vals_all = [v for _, _, vals in series for v in vals if v]
    hi = 2 ** math.ceil(math.log2(max(vals_all) * 1.15))
    lo = min(1.0, 2 ** math.floor(math.log2(min(vals_all))))
    x = lambda v: left + plot_w * math.log2(v / lo) / math.log2(hi / lo)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">',
           f'<rect width="100%" height="100%" fill="{t["bg"]}"/>',
           f'<text x="40" y="48" font-family="{SANS}" font-weight="900" font-size="26" fill="{t["ink"]}">{title}</text>',
           f'<text x="40" y="76" font-family="{SANS}" font-weight="700" font-size="15" fill="{t["muted"]}">{subtitle}</text>']
    lx = left
    for name, key, _ in series:
        out.append(f'<rect x="{lx}" y="92" width="12" height="12" fill="{t[key]}"/>')
        out.append(f'<text x="{lx + 18}" y="103" font-family="{SANS}" font-weight="700" font-size="14" fill="{t["ink"]}">{name}</text>')
        lx += 30 + 8.2 * len(name)
    v = lo
    while v <= hi:
        gx = x(v)
        out.append(f'<line x1="{gx:.1f}" y1="{top}" x2="{gx:.1f}" y2="{top + row_h * len(labels)}" stroke="{t["hair"]}" stroke-width="1"/>')
        tick = f"{v:g}x"
        out.append(f'<text x="{gx:.1f}" y="{top + row_h * len(labels) + 22}" text-anchor="middle" font-family="{MONO}" font-size="13" fill="{t["muted"]}">{tick}</text>')
        v *= 2
    for i, lab in enumerate(labels):
        y0 = top + i * row_h + (row_h - len(series) * (bar_h + gap)) / 2
        out.append(f'<text x="{left - 16}" y="{y0 + 1.5 * (bar_h + gap) + 4:.1f}" text-anchor="end" font-family="{SANS}" font-weight="700" font-size="15" fill="{t["ink"]}">{lab}</text>')
        for k, (_, key, vals) in enumerate(series):
            val = vals[i]
            if not val:
                continue
            y = y0 + k * (bar_h + gap)
            x0, x1 = x(1), x(max(val, 1.0))
            if val < 1:
                x0, x1 = x(val), x(1)
            out.append(f'<rect x="{x0:.1f}" y="{y:.1f}" width="{max(x1 - x0, 1.5):.1f}" height="{bar_h}" fill="{t[key]}"/>')
            if key == "ours" and val >= 1:
                out.append(f'<text x="{x1 + 6:.1f}" y="{y + bar_h - 1:.1f}" font-family="{MONO}" font-weight="600" font-size="12" fill="{t["ink"]}">{val:.1f}x</text>')
    out.append(f'<line x1="{x(1):.1f}" y1="{top - 6}" x2="{x(1):.1f}" y2="{top + row_h * len(labels)}" stroke="{t["ink"]}" stroke-width="2"/>')
    for j, line in enumerate(note):
        out.append(f'<text x="40" y="{height - 30 + 18 * j}" font-family="{SANS}" font-weight="700" font-size="13" fill="{t["muted"]}">{line}</text>')
    out.append("</svg>")
    return "\n".join(out)


def main():
    results, out_dir = sys.argv[1], sys.argv[2]
    os.makedirs(out_dir, exist_ok=True)
    rows = parse(results)
    note = ["Speed-up over tiktoken on one core, log scale. Identical token ids in every run.",
            "TokenDagger has only a Python API and is measured against tiktoken's Python package."]
    labels = [g for g, _ in GROUPS]
    series = [(name, key, [gm(pick(rows, key, "o200k_base", "stream", p)) for _, p in GROUPS]) for name, key in ARMS]
    for theme in THEMES:
        suffix = "" if theme == "light" else "-dark"
        open(os.path.join(out_dir, f"speedup{suffix}.svg"), "w").write(
            bars_svg("New text, o200k_base", "Each document encoded once, caches kept across documents", labels, series, theme, note))
    scen = [("New text", "stream"), ("Same text again", "repeat"), ("Fresh caches per document", "cold")]
    series = []
    for name, key in ARMS:
        vals = []
        for _, s in scen:
            v = [val for (a, e, w, sc), val in rows.items() if a == key and sc == s and not w.startswith("adv-")]
            vals.append(gm(v))
        series.append((name, key, vals))
    for theme in THEMES:
        suffix = "" if theme == "light" else "-dark"
        open(os.path.join(out_dir, f"scenarios{suffix}.svg"), "w").write(
            bars_svg("All main workloads", "Geometric mean over 11 languages, code, books and chat; cl100k, o200k and Llama 4", [l for l, _ in scen], series, theme, note))


if __name__ == "__main__":
    main()
