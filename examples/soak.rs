//! Randomised differential soak against the reference core.
//!
//!     cargo run --release --example soak -- <seconds> [corpus dir ...]
//!
//! All threads share one encoder per encoding, so the cache pool is used
//! concurrently; every thread also keeps a private encoder now and then and
//! drops the shared caches at random. Texts mix slices of real text with
//! random code points from every plane, fragments that sit on pattern
//! boundaries, and long repeats. A mismatch is minimised and written to
//! soak-fail-<encoding>.txt.

use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n.max(1) as u64) as usize
    }
}

const FRAGMENTS: &[&str] = &[
    "'s", "'S", "'t", "'T", "'re", "'RE", "'ve", "'VE", "'m", "'M", "'ll", "'LL", "'Ll", "'d", "'D", "'", "''", "’s",
    " ", "  ", "   ", "\t", "\n", "\r\n", "\n\n", " \n", "\r", "\u{a0}", "\u{3000}", "\u{2003}", "\u{b}", "\u{c}", "\u{85}", "\u{2028}", "\u{feff}",
    "0", "12", "123", "1234", "12345", "٣", "١٢٣", "੫", "①", "Ⅷ", "½", "²",
    ".", "..", "...", ",", "/", "//", "///", "-", "--", "_", "__", "(", ")", "{", "}", "[", "]", "<", ">", "=", "==", "!", "?", "?!", "#", "@", "$", "%", "&", "*", "+", "|", "\\", "~", "`", "^", ":", ";", "\"",
    "a", "A", "ab", "AB", "Ab", "aB", "the", "The", "THE", "hello", "World", "iPhone", "McD", "x",
    "é", "É", "e\u{301}", "\u{301}", "\u{308}", "\u{20dd}", "ß", "ẞ", "İ", "ı", "ǅ", "ǈ", "Ω", "ω", "Ж", "ж", "Ա", "ա", "Ꭰ", "ꭰ",
    "🙂", "👍🏽", "❤️", "❤️‍🔥", "👨‍👩‍👧", "🇩🇪", "\u{200d}", "\u{fe0f}", "\u{fffd}", "\u{200b}",
    "你", "好", "世界", "中文", "。", "，", "「", "」", "：", "、", "我们", "的", "了",
    "こんにちは", "カタカナ", "ー", "ｱ", "ﾞ", "안녕", "하세요", "ᄀ", "ᅡ", "ᆨ",
    "مرحبا", "ال", "\u{64b}", "नमस्ते", "\u{94d}", "สวัสดี", "\u{e31}", "שלום", "Γειά", "Привет",
    "<|endoftext|>", "<|fim_prefix|>", "<|fim_middle|>", "<|fim_suffix|>", "<|endofprompt|>", "<|endof", "|>", "<|",
    "http://x.y/z?a=b", "fn main() {", "}\n", "    ", "\t\t", "#include <x>", "{\"k\": [1, 2]}", "<div class=\"a\">", "</p>",
];

const RANGES: &[(u32, u32)] = &[
    (0x20, 0x7e), (0x0, 0x20), (0x7f, 0xff), (0x100, 0x24f), (0x250, 0x2ff), (0x300, 0x36f), (0x370, 0x3ff), (0x400, 0x52f),
    (0x530, 0x58f), (0x590, 0x5ff), (0x600, 0x6ff), (0x900, 0x97f), (0x980, 0xdff), (0xe00, 0xe7f), (0x10a0, 0x10ff), (0x1100, 0x11ff),
    (0x13a0, 0x13ff), (0x1e00, 0x1fff), (0x2000, 0x206f), (0x2070, 0x24ff), (0x2500, 0x2bff), (0x2c00, 0x2dff), (0x2e80, 0x2fff),
    (0x3000, 0x30ff), (0x3100, 0x33ff), (0x3400, 0x4dbf), (0x4e00, 0x9fff), (0xa000, 0xabff), (0xac00, 0xd7a3), (0xd7b0, 0xd7ff),
    (0xe000, 0xf8ff), (0xf900, 0xfaff), (0xfb00, 0xfdff), (0xfe00, 0xfe6f), (0xff00, 0xffef), (0xfff0, 0xffff),
    (0x10000, 0x1ffff), (0x1f300, 0x1faff), (0x20000, 0x2ffff), (0x30000, 0x3ffff), (0xe0000, 0xe01ef), (0xf0000, 0x10ffff),
];

fn push_random_char(r: &mut Rng, s: &mut String) {
    let (lo, hi) = RANGES[r.below(RANGES.len())];
    loop {
        let c = lo + (r.next() % (hi - lo + 1) as u64) as u32;
        if let Some(ch) = char::from_u32(c) {
            s.push(ch);
            return;
        }
    }
}

fn slice_at_char(t: &str, mut a: usize, mut b: usize) -> &str {
    while !t.is_char_boundary(a) {
        a -= 1;
    }
    while b < t.len() && !t.is_char_boundary(b) {
        b += 1;
    }
    &t[a..b.min(t.len())]
}

fn generate(r: &mut Rng, real: &[String]) -> String {
    // log-uniform target length up to 64 KiB
    let target = (1usize << r.below(17)) + r.below(64);
    let target = if r.below(8) == 0 { r.below(8) } else { target };
    let mut s = String::new();
    while s.len() < target {
        match r.below(10) {
            0..=2 if !real.is_empty() => {
                let t = &real[r.below(real.len())];
                if !t.is_empty() {
                    let a = r.below(t.len());
                    let cap = if r.below(4) == 0 { 4096 } else { 64 };
                    let n = 1 + r.below(cap);
                    s.push_str(slice_at_char(t, a, a + n));
                }
            }
            3..=5 => s.push_str(FRAGMENTS[r.below(FRAGMENTS.len())]),
            6 | 7 => {
                for _ in 0..1 + r.below(8) {
                    push_random_char(r, &mut s);
                }
            }
            8 => {
                // a long run of one fragment or one character
                let cap = if r.below(4) == 0 { 2000 } else { 40 };
                let k = 2 + r.below(cap);
                let mut unit = String::new();
                if r.below(2) == 0 {
                    unit.push_str(FRAGMENTS[r.below(FRAGMENTS.len())]);
                } else {
                    push_random_char(r, &mut unit);
                }
                for _ in 0..k {
                    s.push_str(&unit);
                }
            }
            _ => {
                // a run from one script, the shape CJK cuts and spans see
                let (lo, hi) = RANGES[r.below(RANGES.len())];
                for _ in 0..1 + r.below(200) {
                    if let Some(ch) = char::from_u32(lo + (r.next() % (hi - lo + 1) as u64) as u32) {
                        s.push(ch);
                    }
                }
            }
        }
    }
    s
}

struct Pair {
    name: &'static str,
    fast: tiktoken_fast::CoreBPE,
    reference: tiktoken_rs::CoreBPE,
    make: fn() -> tiktoken_fast::CoreBPE,
}

fn pairs() -> Vec<Pair> {
    vec![
        Pair { name: "r50k_base", fast: tiktoken_fast::r50k_base().unwrap(), reference: tiktoken_rs::r50k_base().unwrap(), make: || tiktoken_fast::r50k_base().unwrap() },
        Pair { name: "p50k_base", fast: tiktoken_fast::p50k_base().unwrap(), reference: tiktoken_rs::p50k_base().unwrap(), make: || tiktoken_fast::p50k_base().unwrap() },
        Pair { name: "cl100k_base", fast: tiktoken_fast::cl100k_base().unwrap(), reference: tiktoken_rs::cl100k_base().unwrap(), make: || tiktoken_fast::cl100k_base().unwrap() },
        Pair { name: "o200k_base", fast: tiktoken_fast::o200k_base().unwrap(), reference: tiktoken_rs::o200k_base().unwrap(), make: || tiktoken_fast::o200k_base().unwrap() },
    ]
}

/// Shrinks a failing text by dropping char ranges while a fresh encoder still disagrees.
fn minimise(p: &Pair, text: &str) -> (String, bool) {
    let fresh = (p.make)();
    let fails = |t: &str| fresh.encode_ordinary(t) != p.reference.encode_ordinary(t);
    if !fails(text) {
        return (text.to_string(), false);
    }
    let mut chars: Vec<char> = text.chars().collect();
    let mut n = chars.len() / 2;
    while n >= 1 {
        let mut i = 0;
        while i < chars.len() {
            let mut c = chars.clone();
            c.drain(i..(i + n).min(c.len()));
            let t: String = c.iter().collect();
            if fails(&t) {
                chars = c;
            } else {
                i += n;
            }
        }
        n /= 2;
    }
    (chars.iter().collect(), true)
}

fn read_corpora(dirs: &[String]) -> Vec<String> {
    fn walk(p: &Path, out: &mut Vec<String>) {
        if p.is_dir() {
            if let Ok(rd) = std::fs::read_dir(p) {
                for e in rd.flatten() {
                    walk(&e.path(), out);
                }
            }
        } else if let Ok(b) = std::fs::read(p) {
            if b.len() < 4 << 20 && !b.contains(&0) {
                out.push(String::from_utf8_lossy(&b).into_owned());
            }
        }
    }
    let mut out = Vec::new();
    for d in dirs {
        walk(Path::new(d), &mut out);
    }
    out
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let secs: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(60);
    let mut dirs: Vec<String> = args[2..].to_vec();
    if dirs.is_empty() {
        dirs.push(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/corpora").to_string());
    }
    let real = Arc::new(read_corpora(&dirs));
    let ps = Arc::new(pairs());
    let stop = Arc::new(AtomicBool::new(false));
    let texts = Arc::new(AtomicU64::new(0));
    let bytes = Arc::new(AtomicU64::new(0));
    let failed = Arc::new(AtomicBool::new(false));
    let deadline = Instant::now() + Duration::from_secs(secs);
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    eprintln!("soak: {threads} threads, {secs} s, {} corpus files", real.len());
    let mut hs = Vec::new();
    for t in 0..threads {
        let (real, ps, stop, texts, bytes, failed) = (real.clone(), ps.clone(), stop.clone(), texts.clone(), bytes.clone(), failed.clone());
        hs.push(std::thread::spawn(move || {
            let mut r = Rng(0x9e3779b97f4a7c15 ^ ((t as u64 + 1) * 0x2545f4914f6cdd1d) ^ std::process::id() as u64);
            let mut private: Vec<Option<tiktoken_fast::CoreBPE>> = (0..ps.len()).map(|_| None).collect();
            while !stop.load(Ordering::Relaxed) {
                let k = r.below(ps.len());
                let p = &ps[k];
                if r.below(20000) == 0 {
                    p.fast.reset_caches();
                }
                if r.below(5000) == 0 {
                    private[k] = if private[k].is_some() { None } else { Some((p.make)()) };
                }
                let text = generate(&mut r, &real);
                let want = p.reference.encode_ordinary(&text);
                let enc = private[k].as_ref().unwrap_or(&p.fast);
                let mut bad = enc.encode_ordinary(&text) != want;
                if !bad && r.below(8) == 0 {
                    bad = enc.encode_ordinary_parallel(&text, 1 + r.below(300)) != want;
                }
                if !bad && r.below(8) == 0 {
                    bad = enc.encode_with_special_tokens(&text) != p.reference.encode_with_special_tokens(&text);
                }
                if !bad && r.below(8) == 0 {
                    bad = enc.decode_bytes(&want).ok().as_deref() != Some(text.as_bytes());
                }
                if bad && !failed.swap(true, Ordering::SeqCst) {
                    stop.store(true, Ordering::SeqCst);
                    std::fs::write(format!("soak-fail-{}-raw.txt", p.name), &text).unwrap();
                    let (m, fresh) = minimise(p, &text);
                    std::fs::write(format!("soak-fail-{}.txt", p.name), &m).unwrap();
                    eprintln!("MISMATCH {} (reproduces on fresh encoder: {fresh}) minimised {:?}", p.name, m);
                }
                texts.fetch_add(1, Ordering::Relaxed);
                bytes.fetch_add(text.len() as u64, Ordering::Relaxed);
            }
        }));
    }
    let mut last = Instant::now();
    while Instant::now() < deadline && !stop.load(Ordering::Relaxed) {
        std::thread::sleep(Duration::from_millis(200));
        if last.elapsed() > Duration::from_secs(60) {
            eprintln!("soak: {} texts, {} MB", texts.load(Ordering::Relaxed), bytes.load(Ordering::Relaxed) >> 20);
            last = Instant::now();
        }
    }
    stop.store(true, Ordering::SeqCst);
    for h in hs {
        h.join().unwrap();
    }
    println!("soak done: {} texts, {} MB, {}", texts.load(Ordering::Relaxed), bytes.load(Ordering::Relaxed) >> 20, if failed.load(Ordering::SeqCst) { "MISMATCH" } else { "no mismatch" });
    if failed.load(Ordering::SeqCst) {
        std::process::exit(1);
    }
}
