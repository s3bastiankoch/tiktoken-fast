"""Draws the README's headline figures from the summary.json files
bench/summarize.py writes, one per machine:

    python plot_languages.py <summary.json> [<summary.json> ...] <out dir>

languages-{light,dark}.png: speed-up on new text per language, over the most
used library and over the fastest other one. Each bar ends at the slowest
machine; a lighter segment reaches the fastest.
headline-{light,dark}.png: three stat tiles for the top of the README.
Needs Chrome (see cards.py).
"""
import json
import sys

from cards import ICON_BOLT, card, render, tiles

# (language, summary label, other arm, shown as, colour: c1 most used, c2 fastest other)
PICK = [
    ("Rust", "Rust", "tiktoken-rs", "tiktoken", "--c1"),
    ("Rust", "Rust", "bpe-openai", "bpe-openai", "--c2"),
    ("Python", "Python", "tiktoken", "tiktoken", "--c1"),
    ("JavaScript", "JavaScript", "tiktoken", "tiktoken (npm)", "--c1"),
    ("JavaScript", "JavaScript", "gpt-tokenizer", "gpt-tokenizer", "--c2"),
    ("Node, native", "Node, native", "tiktoken", "tiktoken (npm)", "--c1"),
    ("Node, native", "Node, native", "gpt-tokenizer", "gpt-tokenizer", "--c2"),
    ("Go", "Go", "tiktoken-go", "tiktoken-go", "--c1"),
    ("Go", "Go", "tokenizer", "tiktoken-go/tokenizer", "--c2"),
]


def point(s, label, arm):
    h = next((h for h in s["headlines"] if h["label"] == label and h["other"] == arm), None)
    return h["stream"]["point"] if h and h.get("stream") else None


def rows_from(summaries):
    rows = []
    for lang, label, arm, shown, colour in PICK:
        vals = [v for v in (point(s, label, arm) for s in summaries) if v]
        if vals:
            rows.append((lang, shown, colour, min(vals), max(vals)))
    return rows


def rng(lo, hi):
    return f"{lo:.1f}x" if f"{lo:.1f}" == f"{hi:.1f}" else f"{lo:.1f}–{hi:.1f}x"


def chart(rows):
    top = max(r[4] for r in rows)
    out, prev = [], None
    for lang, shown, colour, lo, hi in rows:
        gap = "22px" if prev and lang != prev else "10px" if prev else "34px"
        name = lang if lang != prev else ""
        prev = lang
        out.append(
            f'<div style="display:flex;align-items:center;margin-top:{gap};font-size:19px">'
            f'<div style="width:160px;font-weight:500">{name}</div>'
            f'<div style="width:280px;color:var(--muted)">over {shown}</div>'
            f'<div style="flex:1;display:flex;align-items:center;gap:14px">'
            f'<div style="display:flex;width:{hi / top * 80:.2f}%">'
            f'<div style="width:{lo / hi * 100:.2f}%;height:16px;border-radius:5px 0 0 5px;background:var({colour})"></div>'
            f'<div style="flex:1;height:16px;border-radius:0 5px 5px 0;background:var({colour});opacity:.3"></div></div>'
            f'<div style="font-weight:500;white-space:nowrap;font-feature-settings:\'tnum\'">{rng(lo, hi)}</div></div></div>')
    return "".join(out)


def main():
    *paths, out = sys.argv[1:]
    summaries = [json.load(open(p)) for p in paths]
    rows = rows_from(summaries)
    most = [r for r in rows if r[2] == "--c1"]
    body = card(
        ICON_BOLT, "Speed-up on new text",
        "Over the most used library in each language, slowest machine",
        f"{min(r[3] for r in most):.1f}x to {max(r[3] for r in most):.1f}x",
        [("--c1", "over the <b>most used</b> library"), ("--c2", "over the <b>fastest other</b> one")],
        chart(rows),
        f"Bars end at the slowest of {len(summaries)} servers, the lighter part reaches the fastest. "
        "One core, text the library has not seen, every token checked against tiktoken first.")
    render(body, out, "languages")

    pick = lambda lang: next(r for r in most if r[0] == lang)
    docs = min(s["documents_checked"] for s in summaries)
    diff = sum(s["mismatches"] for s in summaries)
    render(tiles([
        (f"{pick('Python')[3]:.1f}x", "faster in Python", "than tiktoken, on the slowest of three servers"),
        (f"{pick('JavaScript')[3]:.1f}x", "faster in JavaScript",
         f"than the tiktoken npm package; {pick('Node, native')[3]:.1f}x with the native addon"),
        (f"{diff}", "tokens different", f"{docs:,} documents per machine, checked against tiktoken before timing"),
    ]), out, "headline")


if __name__ == "__main__":
    main()
