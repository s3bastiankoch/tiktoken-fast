"""tiktoken-fast against tiktoken through the public API. Run with pytest or
plain python."""
import pickle
import random

import tiktoken
import tiktoken_fast

NAMES = ["r50k_base", "p50k_base", "p50k_edit", "cl100k_base", "o200k_base", "o200k_harmony"]
SAMPLES = [
    "",
    "hello world",
    "Hello, World! It's 2026 and we're testing tokenizers... 1234567890",
    "  leading and trailing spaces  \n\n\ttabs\r\nwindows",
    "naïve café déjà vu — “quotes” ‘single’ … ellipsis",
    "Привет, мир! Это проверка токенизатора.",
    "مرحبا بالعالم، هذا اختبار",
    "नमस्ते दुनिया, यह एक परीक्षण है",
    "สวัสดีชาวโลก นี่คือการทดสอบ",
    "你好，世界！这是一个测试。日本語のテキストも。한국어 텍스트도.",
    "emoji 🙂👍🏽👨‍👩‍👧‍👦 flags 🇩🇪🇺🇸 and ZWJ",
    "def f(x):\n    return x ** 2  # comment\n\nclass A:\n    pass\n",
    '{"a": [1, 2.5e-3, null, true], "b": "\\u00e9"}',
    "a" * 3000,
    " " * 500 + "x",
    "1" * 100,
    "<|endoftext|> and <|fim_prefix|> inside",
    "\x00\x01\x02 control \x7f",
]


def random_text(rng, n):
    pools = [
        "abcdefghijklmnopqrstuvwxyz      ",
        "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,;:!?'\"()[]{}<>-_/\\|@#$%^&*+=~`\n\t",
        "äöüßéèêàçñøåæœ",
        "абвгдежзийклмнопрстуфхцчшщыэюя",
        "的一是不了人我在有他这中大来上国个到说们",
        "😀🙂👍🏽🎉🔥",
    ]
    out = []
    for _ in range(n):
        p = rng.choice(pools)
        out.append(rng.choice(p))
    if rng.random() < 0.3:
        out.append(chr(rng.randrange(0x20, 0x2FFFF)) if rng.random() < 0.5 else chr(rng.randrange(0x80, 0x800)))
    return "".join(out)


def pairs():
    for n in NAMES:
        yield n, tiktoken.get_encoding(n), tiktoken_fast.get_encoding(n)


def corpus(seed=1, count=400):
    rng = random.Random(seed)
    return SAMPLES + [random_text(rng, rng.randrange(1, 400)) for _ in range(count)]


def test_uses_fast_core():
    for n, _, b in pairs():
        assert b._fast, n
        assert isinstance(b, tiktoken.Encoding)


def test_encode_variants():
    texts = corpus()
    for n, a, b in pairs():
        for t in texts:
            assert a.encode_ordinary(t) == b.encode_ordinary(t), (n, t)
            assert a.encode(t, disallowed_special=()) == b.encode(t, disallowed_special=()), (n, t)
            assert a.encode(t, allowed_special="all") == b.encode(t, allowed_special="all"), (n, t)


def test_count_tokens():
    texts = corpus(seed=4, count=200)
    for n, a, b in pairs():
        for t in texts:
            assert b.count_tokens(t) == len(a.encode(t)) if "<|" not in t else True, (n, t)
            assert b.count_tokens(t, disallowed_special=()) == len(a.encode(t, disallowed_special=())), (n, t)
            assert b.count_tokens(t, allowed_special="all") == len(a.encode(t, allowed_special="all")), (n, t)
        try:
            b.count_tokens("x <|endoftext|>")
        except ValueError:
            pass
        else:
            raise AssertionError(n)


def test_disallowed_special_raises():
    for n, a, b in pairs():
        for text in ["x <|endoftext|> y"]:
            for enc in (a, b):
                try:
                    enc.encode(text)
                except ValueError as e:
                    assert "endoftext" in str(e)
                else:
                    raise AssertionError(n)


def test_batch():
    texts = corpus(seed=2, count=300)
    for n, a, b in pairs():
        assert a.encode_ordinary_batch(texts) == b.encode_ordinary_batch(texts), n
        assert a.encode_batch(texts, disallowed_special=()) == b.encode_batch(texts, disallowed_special=()), n
        assert a.encode_batch(texts, allowed_special="all") == b.encode_batch(texts, allowed_special="all"), n
        assert b.encode_ordinary_batch(texts, num_threads=1) == b.encode_ordinary_batch(texts, num_threads=13), n
    b = tiktoken_fast.get_encoding("cl100k_base")
    try:
        b.encode_batch(["fine", "<|endoftext|>"])
    except ValueError:
        pass
    else:
        raise AssertionError("batch did not raise")


def test_decode():
    texts = corpus(seed=3, count=100)
    for n, a, b in pairs():
        for t in texts:
            toks = a.encode(t, allowed_special="all")
            assert a.decode_bytes(toks) == b.decode_bytes(toks), n
            assert a.decode(toks) == b.decode(toks), n
            assert a.decode_with_offsets(toks) == b.decode_with_offsets(toks), n
        for enc in (a, b):
            try:
                enc.decode([10**7])
            except KeyError:
                pass
            else:
                raise AssertionError(n)


def test_helpers_and_attributes():
    for n, a, b in pairs():
        assert a.n_vocab == b.n_vocab and a.max_token_value == b.max_token_value, n
        assert a.special_tokens_set == b.special_tokens_set, n
        if "<|endoftext|>" in a.special_tokens_set:
            assert a.eot_token == b.eot_token, n
        assert a.encode_single_token("hello") == b.encode_single_token("hello"), n
        assert a.decode_single_token_bytes(1000) == b.decode_single_token_bytes(1000), n
        (sa, ua), (sb, ub) = a.encode_with_unstable("hello fan"), b.encode_with_unstable("hello fan")
        assert sa == sb and sorted(ua) == sorted(ub), n  # tiktoken returns the completions unordered
        assert a._encode_single_piece("hello") == b._encode_single_piece("hello"), n
        assert a.token_byte_values()[:50] == b.token_byte_values()[:50], n


def test_numpy():
    try:
        import numpy  # noqa: F401
    except ImportError:
        return
    for n, a, b in pairs():
        t = "numpy check, ünïcode 世界"
        assert a.encode_to_numpy(t).tolist() == b.encode_to_numpy(t).tolist(), n


def test_surrogates():
    t = "lone \ud800 surrogate and pair \ud83d\ude00"
    for n, a, b in pairs():
        assert a.encode_ordinary(t) == b.encode_ordinary(t), n
        assert a.encode(t) == b.encode(t), n
        assert a.encode_ordinary_batch([t, "x"]) == b.encode_ordinary_batch([t, "x"]), n


def test_pickle_and_model():
    b = tiktoken_fast.get_encoding("o200k_base")
    c = pickle.loads(pickle.dumps(b))
    assert c is not None and c.encode("hi there") == b.encode("hi there")
    assert tiktoken_fast.encoding_for_model("gpt-4o") is b
    assert tiktoken_fast.encoding_for_model("gpt-4").name == "cl100k_base"
    custom = tiktoken_fast.Encoding("custom", pat_str=b._pat_str, mergeable_ranks=b._mergeable_ranks, special_tokens={})
    d = pickle.loads(pickle.dumps(custom))
    assert d.encode("hi there") == b.encode("hi there")


def test_unknown_pattern_falls_back():
    base = tiktoken.get_encoding("r50k_base")
    e = tiktoken_fast.Encoding("odd", pat_str=r"\S+|\s+", mergeable_ranks=base._mergeable_ranks, special_tokens={})
    ref = tiktoken.Encoding("odd", pat_str=r"\S+|\s+", mergeable_ranks=base._mergeable_ranks, special_tokens={})
    assert not e._fast
    assert e.encode("hello  world") == ref.encode("hello  world")


def test_patch():
    tiktoken_fast.patch()
    try:
        assert isinstance(tiktoken.get_encoding("cl100k_base"), tiktoken_fast.Encoding)
        assert isinstance(tiktoken.encoding_for_model("gpt-4o"), tiktoken_fast.Encoding)
    finally:
        tiktoken_fast.unpatch()
    assert not isinstance(tiktoken.get_encoding("cl100k_base"), tiktoken_fast.Encoding)


if __name__ == "__main__":
    for name, fn in list(globals().items()):
        if name.startswith("test_"):
            fn()
            print("ok", name)
