//! Token ids must equal the reference core's, encoding by encoding.

use std::collections::HashSet;

fn pairs() -> Vec<(&'static str, tiktoken_fast::CoreBPE, tiktoken_rs::CoreBPE)> {
    vec![
        ("cl100k_base", tiktoken_fast::cl100k_base().unwrap(), tiktoken_rs::cl100k_base().unwrap()),
        ("o200k_base", tiktoken_fast::o200k_base().unwrap(), tiktoken_rs::o200k_base().unwrap()),
        ("p50k_base", tiktoken_fast::p50k_base().unwrap(), tiktoken_rs::p50k_base().unwrap()),
        ("r50k_base", tiktoken_fast::r50k_base().unwrap(), tiktoken_rs::r50k_base().unwrap()),
    ]
}

fn check(name: &str, fast: &tiktoken_fast::CoreBPE, reference: &tiktoken_rs::CoreBPE, text: &str) {
    let got = fast.encode_ordinary(text);
    let want = reference.encode_ordinary(text);
    if got != want {
        let n = got.iter().zip(&want).take_while(|(a, b)| a == b).count();
        panic!("{name}: mismatch at token {n} of {}: got {:?} want {:?} for {:?}", want.len(), &got[n.saturating_sub(3)..(n + 5).min(got.len())], &want[n.saturating_sub(3)..(n + 5).min(want.len())], text.chars().take(200).collect::<String>());
    }
}

const TRICKY: &[&str] = &[
    "", " ", "  ", "\n", "\n\n", " \n", "\n \n\t", "a\n\nb", "x  y", "x   y", "hello world", "Hello World",
    "it's It'S IT'S don't DON'T we're I'll I'd I've I'm 'sx 'tx 'llx", "'s's's", "don'tDon'T",
    "123 1234 12345 ١٢٣٤ ①②③", "a1b2c3 ABC123def", "snake_case camelCase PascalCase SCREAMING_CASE", "iPhone eBay McDonald's",
    "ΑΒΓ αβγ Ωμέγα Ωmega Ǆ ǅ ǆ ʰ ˢ", "straße STRASSE İstanbul", "你好世界 こんにちは 안녕하세요", "東京タワー  大阪城",
    "🙂🎉🚀 ❤️‍🔥 👨‍👩‍👧‍👦 🇩🇪", "a🙂b", "e\u{301} e\u{301}x \u{301}x", "café CAFÉ Café",
    "http://example.com/a/b?c=d&e=f", "foo//bar ///baz //\n", "path/to/file.go /usr/bin", "x /y /\n/", " / / /",
    "SELECT id, name FROM users WHERE created_at > '2024-01-01' ORDER BY id;", "fn main() {\n\tprintln!(\"hi\");\n}\n",
    "\r\n\r\n", "a\r\nb", " \r\n", "\t\t\n  \t", "tab\tsep", "\u{a0}nbsp\u{a0}", "\u{2003}em space\u{2003}x", "\u{3000}ideographic",
    "....", " ....", "?!?!", " ?!", "a.b.c", "$100.00", "-1", " -1", "+1", "a-b", "ab--cd", "@user #tag",
    "ABCdef", "ABCDEF", "abcDEF", "AbC", "aBC", " ABCdef", " ABCDEF", ".ABCdef", "ÀBCdef", "ÀÁÂ", "àáâ", "ÀÁâ",
    " \0\u{9be}\u{995}", " \0\u{94d}x", " \0", "\0 \0a", "AᅡՏ", " AᅡՏ", ".AᅡՏ", "AᅡՏ's", "AᅡՏx", "ABᅡᅡCD", "Aʰ", "AʰB", "Ae\u{301}B", "Xカ", "Xカ中", "ǅᅡՏ",
    "ⅰⅱⅲ ⅠⅡⅢ", "ʻokina", "Ꜳ ꜳ ᾈ ᾀ", "ǈ ǉ ǋ ǌ", "Ⓐⓐ", "𝐀𝐛𝐜 𝟏𝟐𝟑 𝒳", "😀\u{fe0f}", "\u{200d}", "x\u{200d}y",
];

#[test]
fn tricky_strings_match_reference() {
    for (name, fast, reference) in pairs() {
        for t in TRICKY {
            check(name, &fast, &reference, t);
        }
        // every pair of tricky strings joined, so boundaries get exercised
        for a in TRICKY.iter().take(40) {
            for b in TRICKY.iter().take(40) {
                let s = format!("{a}{b}");
                check(name, &fast, &reference, &s);
            }
        }
    }
}

#[test]
fn corpora_match_reference() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/corpora");
    for (name, fast, reference) in pairs() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            check(name, &fast, &reference, &text);
            // and a slice that starts mid-way, so warm caches see new boundaries
            let mid = text.char_indices().nth(text.chars().count() / 3).map(|(i, _)| i).unwrap_or(0);
            check(name, &fast, &reference, &text[mid..]);
        }
    }
}

/// A small deterministic generator over a mixed alphabet, so random pieces,
/// long runs and odd boundaries all occur.
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn random_text(rng: &mut Lcg, n: usize) -> String {
    const PARTS: &[&str] = &[
        "a", "b", "e", "t", "s", "Z", "Q", " ", "  ", "\n", "\t", "'", "'s", "'LL", "1", "22", "333", "4444", ".", ",", "!", "/", "//", "-", "_",
        "é", "ß", "Ω", "ω", "ʰ", "́", "你", "好", "の", "한", "🙂", "❤️", "\u{200d}", "\u{a0}", "\u{3000}", "ⅷ", "①", "𝐀", "ǅ", "İ", "\r", "\r\n",
        "the", "The", "THE", "quick", "Quick", "QUICK", "fox", "jumps", "over", "lazy", "dog", "wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww",
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "東京タワー大阪城京都奈良神戸横浜名古屋札幌福岡仙台広島", "안녕하세요반갑습니다",
        "，", "。", "「", "」", "、", "：", "！", "？", "…", "\u{3000}", "１２", "三", "の", "ア", "ー", "\u{0301}", "\u{3099}", "Ⓐ", "ⓐ", "Ａ", "ａ", "\u{2160}", "₂", "€", "™", "ПриветМир", "مرحبا", "שלום", "नमस्ते", "สวัสดี",
    ];
    let mut s = String::new();
    while s.len() < n {
        s.push_str(PARTS[(rng.next() as usize) % PARTS.len()]);
    }
    s
}

#[test]
fn random_texts_match_reference() {
    let mut rng = Lcg(42);
    for (name, fast, reference) in pairs() {
        for i in 0..400 {
            let n = if i % 10 == 0 { 3000 } else { 1 + (rng.next() as usize) % 120 };
            let text = random_text(&mut rng, n);
            check(name, &fast, &reference, &text);
        }
    }
}

#[test]
fn special_tokens_match_reference() {
    for (name, fast, reference) in pairs() {
        let texts = [
            "hello <|endoftext|> world", "<|endoftext|>", "a<|endoftext|><|endoftext|>b", "<|fim_prefix|>x<|fim_suffix|>y<|fim_middle|>",
            "no specials here", "<|endofprompt|> and <|endoftext|>", "<|endof", "text|>", "<|endoftext|",
        ];
        let all: HashSet<&str> = reference.special_tokens();
        for t in texts {
            let want = reference.encode_with_special_tokens(t);
            let got = fast.encode_with_special_tokens(t);
            assert_eq!(got, want, "{name}: {t:?}");
            let want2 = reference.encode(t, &all).unwrap().0;
            let got2 = fast.encode(t, &all);
            assert_eq!(got2, want2, "{name}: {t:?}");
            let none: HashSet<&str> = HashSet::new();
            assert_eq!(fast.encode(t, &none), reference.encode(t, &none).unwrap().0, "{name}: none allowed, {t:?}");
            let one: HashSet<&str> = ["<|endoftext|>"].into_iter().collect();
            assert_eq!(fast.encode(t, &one), reference.encode(t, &one).unwrap().0, "{name}: one allowed, {t:?}");
        }
    }
}

#[test]
fn decode_round_trips() {
    for (name, fast, _) in pairs() {
        for t in TRICKY {
            let ids = fast.encode_ordinary(t);
            let bytes = fast.decode_bytes(&ids).unwrap();
            assert_eq!(bytes, t.as_bytes(), "{name}: {t:?}");
        }
    }
}

#[test]
fn parallel_matches_single() {
    let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/corpora");
    for (name, fast, _) in pairs() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            for chunk in [4096, 20_000, 100_000] {
                assert_eq!(fast.encode_ordinary_parallel(&text, chunk), fast.encode_ordinary(&text), "{name}: chunk {chunk} on {}", path.display());
            }
        }
        let mut rng = Lcg(7);
        for _ in 0..50 {
            let text = random_text(&mut rng, 300_000);
            assert_eq!(fast.encode_ordinary_parallel(&text, 4096), fast.encode_ordinary(&text), "{name}: random");
        }
    }
}

#[test]
fn cjk_spans_match_reference() {
    const PARTS: &[&str] = &[
        "你", "好", "的", "是", "了", "宝", "玉", "黛", "賈", "母", "說", "道", "の", "は", "ア", "ー", "カ", "한", "국", "어",
        "，", "。", "「", "」", "、", "：", "！", "？", "…", "—", "\u{3000}", "１", "２", "三", "Ⅳ", "\u{0301}", "\u{3099}",
        "Ⓐ", "ⓐ", "Ａ", "ａ", "ǅ", "ǆ", "Ω", "ω", "a", "B", " ", "\n", "'", "'s", "1", ".", "/", "é", "🙂",
    ];
    for seed in [2026u64, 7, 99] {
    let mut rng = Lcg(seed);
    for (name, fast, reference) in pairs() {
        for _ in 0..2500 {
            let n = 1 + (rng.next() as usize) % 60;
            let mut text = String::new();
            for _ in 0..n {
                // mostly CJK, sometimes anything
                let k = if rng.next() % 4 == 0 { (rng.next() as usize) % PARTS.len() } else { (rng.next() as usize) % 31 };
                text.push_str(PARTS[k]);
            }
            check(name, &fast, &reference, &text);
        }
    }
    }
}
