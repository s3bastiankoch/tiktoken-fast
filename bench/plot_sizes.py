"""Draws speed-up over tiktoken-rs against document size from a compare run on
the make_sizes.py workloads.

    python plot_sizes.py <sizes.txt> <out dir>

Writes sizes-{light,dark}.png (o200k_base) and sizes-cl100k-{light,dark}.png:
new text, one line per kind of text. Needs Chrome (see cards.py).
"""
import math
import re
import sys

from cards import ICON_RULER, card, render

KINDS = [("en", "English", "--c1"), ("zh", "Chinese", "--c2"), ("ru", "Russian", "--c3"), ("code", "Code", "--c4")]


def parse(path, enc):
    rows = {}
    for line in open(path):
        m = re.match(r"(\S+)\s+(\w+)-(\d+)B\s+stream\s.*?ours\s+([\d.]+)x", line)
        if m and m.group(1) == enc:
            rows[(m.group(2), int(m.group(3)))] = float(m.group(4))
    return rows


def size(s):
    return f"{s} B" if s < 1024 else f"{s >> 10} KiB" if s < 1 << 20 else f"{s >> 20} MiB"


def chart(rows):
    W, H = 1112, 330
    L, R, T, B = 70, 12, 10, 58
    pw, ph = W - L - R, H - T - B
    sizes = sorted({s for _, s in rows})
    hi = math.ceil(max(rows.values()))
    X = lambda s: L + pw * math.log2(s / sizes[0]) / math.log2(sizes[-1] / sizes[0])
    Y = lambda v: T + ph - v / hi * ph
    g = []
    for v in range(0, hi + 1):
        y = Y(v)
        if v:
            g.append(f'<line x1="{L}" y1="{y:.1f}" x2="{W - R}" y2="{y:.1f}" stroke="var(--grid)"/>')
        g.append(f'<line x1="{L - 7}" y1="{y:.1f}" x2="{L}" y2="{y:.1f}" stroke="var(--axis)" stroke-width="1.5"/>')
        g.append(f'<text x="{L - 14}" y="{y + 5.5:.1f}" text-anchor="end" class="tk">{v}x</text>')
    for s in sizes[::4]:
        x = X(s)
        g.append(f'<line x1="{x:.1f}" y1="{T + ph}" x2="{x:.1f}" y2="{T + ph + 7}" stroke="var(--axis)" stroke-width="1.5"/>')
        g.append(f'<text x="{x:.1f}" y="{T + ph + 29}" text-anchor="middle" class="tk">{size(s)}</text>')
    for k, _, c in KINDS:
        pts = " L".join(f"{X(s):.1f},{Y(rows[(k, s)]):.1f}" for s in sizes if (k, s) in rows)
        g.append(f'<path d="M{pts}" stroke="var({c})" stroke-width="2.5" fill="none" stroke-linejoin="round" stroke-linecap="round"/>')
    return (f'<svg width="{W}" height="{H}" viewBox="0 0 {W} {H}" style="display:block;margin-top:30px;overflow:visible">'
            f'<style>.tk{{font:16px Inter;fill:var(--muted);font-feature-settings:"tnum"}} .ax{{font:16px Inter;fill:var(--muted)}}</style>'
            f'{"".join(g)}<path d="M{L} {T} V{T + ph} H{W - R}" stroke="var(--axis)" stroke-width="1.5" fill="none"/>'
            f'<text x="{L + pw / 2:.1f}" y="{H - 2}" text-anchor="middle" class="ax">document size (log scale)</text></svg>')


def main():
    path, out = sys.argv[1], sys.argv[2]
    for enc, name in (("o200k_base", "sizes"), ("cl100k_base", "sizes-cl100k")):
        rows = parse(path, enc)
        if not rows:
            continue
        sizes = sorted({s for _, s in rows})
        body = card(
            ICON_RULER, "Speed-up by document size",
            f"Over tiktoken-rs, new text, {enc}, {size(sizes[0])} to {size(sizes[-1])}",
            f"{min(rows.values()):.1f}x to {max(rows.values()):.1f}x",
            [(c, f"<b>{label}</b>") for _, label, c in KINDS],
            chart(rows),
            "Texts under 1 KiB gain less. From 1 KiB on, the gain stays flat up to 1 MiB.")
        render(body, out, name)


if __name__ == "__main__":
    main()
