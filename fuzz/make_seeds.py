"""Writes fuzz/seeds/ (two selector bytes, then text) and fuzz/tricky.dict."""
import os, random, sys
here = os.path.dirname(os.path.abspath(__file__))
corp = os.path.join(here, "..", "tests", "corpora")
out = os.path.join(here, "seeds"); os.makedirs(out, exist_ok=True)
frags = [
    "'s", "'S", "'t", "'T", "'re", "'RE", "'ve", "'m", "'ll", "'LL", "'Ll", "'d", "'D", "’s",
    " ", "  ", "\t", "\n", "\r\n", "\r\n\r\n", " \n", "\n\n\n", " ", "　", " ", "\u0085", " ", "﻿",
    "1", "12", "123", "1234", "12345", "١٢٣", "①", "Ⅷ",
    "/", "//", "///", "...", "?!", "--", "__", "==", "{}", "()", "[]", "<|endoftext|>", "<|fim_prefix|>", "<|endofprompt|>", "<|endof",
    "é", "́", "̈", "ß", "ẞ", "İ", "ı", "ǅ", "ǈ", "Ω", "Ж", "ж", "É", "é",
    "\U0001f642", "❤️‍\U0001f525", "\U0001f468‍\U0001f469‍\U0001f467", "\U0001f1e9\U0001f1ea", "‍", "️", "�", "​",
    "你好", "世界", "。", "，", "「", "こんにちは", "カタカナ", "ー", "ｱ",
    "안녕", "각", "مرحبا", "नमस्ते", "สวัสดี",
    "iPhone", "McDonald's", "HTTPServer", "x86_64", "a1b2", "ABCdef",
]
with open(os.path.join(here, "tricky.dict"), "w") as d:
    for i, f in enumerate(frags):
        b = f.encode("utf-8")
        d.write('f%d="%s"\n' % (i, "".join("\\x%02x" % c for c in b)))
rnd = random.Random(1)
n = 0
def seed(text):
    global n
    data = bytes([rnd.randrange(256), rnd.randrange(256)]) + text.encode("utf-8")
    open(os.path.join(out, "s%05d" % n), "wb").write(data); n += 1
for f in frags:
    for g in rnd.sample(frags, 4):
        seed(f + g + f)
for name in sorted(os.listdir(corp)):
    t = open(os.path.join(corp, name), encoding="utf-8", errors="replace").read()
    for _ in range(60):
        a = rnd.randrange(len(t)); seed(t[a:a + rnd.choice([16, 64, 256, 1024])])
print(n, "seeds")
