//! Every string of up to N units over one representative per character class
//! and byte length the patterns distinguish, in all four encodings.
//!
//!     cargo run --release --example exhaustive -- [max_units] [small|ctx]
//!
//! The pre-tokenisers decide a piece from a few characters of context, so
//! short strings over these units reach every branch of the scanners; with
//! `small` a reduced unit set goes one unit longer. With `ctx` each string is
//! also checked inside long English and Chinese text and repeated past 64
//! bytes, which reaches the vector cut pass, the segment memo and CJK cuts.

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;

const UNITS: &[&str] = &[
    // ASCII: cases, digits, spaces, newlines, apostrophe, punctuation, slash
    "a", "A", "s", "S", "0", " ", "  ", "\t", "\n", "\r", "'", "'s", "'S", "'ll", "'LL", "'ve", ".", "/", "_", "\"",
    // two bytes: Ll Lu Lt Lm Lo Mn Lu(Armenian) Nd No Zs(NBSP) NEL Pi
    "é", "É", "ǅ", "ʰ", "ƻ", "\u{301}", "Տ", "١", "²", "\u{a0}", "\u{85}", "«",
    // three bytes: Lo(jamo) Lo(CJK) Lo(kana) Lo(hangul) Lm(prolonged mark) Zs Nl No Po Me Mc Lm Lu Ll Zl So Pf
    "ᅡ", "中", "カ", "가", "ー", "\u{3000}", "Ⅷ", "①", "。", "\u{20dd}", "\u{903}", "ᴬ", "Ａ", "ａ", "\u{2028}", "\u{fffd}", "’",
    // four bytes: Lu Ll Lo So Nd Sk Lo(CJK ext B)
    "𝐀", "𝐚", "𐀀", "🙂", "𝟏", "\u{1f3fd}", "𠀀",
];
const SMALL: &[&str] = &["a", "A", " ", "\n", "'s", ".", "/", "0", "é", "É", "ʰ", "\u{301}", "Տ", "\u{a0}", "ᅡ", "中", "ー", "\u{3000}", "Ａ", "ａ", "𝐀", "𝐚", "🙂", "\r", "'"];

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let max: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(4);
    let mode = args.get(2).cloned().unwrap_or_default();
    let units: &'static [&'static str] = if mode == "small" { SMALL } else { UNITS };
    let ctx = mode == "ctx";
    let encs = Arc::new(vec![
        ("r50k_base", tiktoken_fast::r50k_base().unwrap(), tiktoken_rs::r50k_base().unwrap()),
        ("p50k_base", tiktoken_fast::p50k_base().unwrap(), tiktoken_rs::p50k_base().unwrap()),
        ("cl100k_base", tiktoken_fast::cl100k_base().unwrap(), tiktoken_rs::cl100k_base().unwrap()),
        ("o200k_base", tiktoken_fast::o200k_base().unwrap(), tiktoken_rs::o200k_base().unwrap()),
    ]);
    let n = units.len() as u64;
    let total: u64 = (1..=max).map(|k| n.pow(k)).sum();
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    let next = Arc::new(AtomicU64::new(0));
    let bad = Arc::new(AtomicUsize::new(0));
    eprintln!("exhaustive: {} units, up to {max}, {total} strings x 4 encodings, {threads} threads", n);
    let hs: Vec<_> = (0..threads)
        .map(|_| {
            let (encs, next, bad) = (encs.clone(), next.clone(), bad.clone());
            std::thread::spawn(move || {
                const EN: &str = "The quick brown fox jumps over the lazy dog, again and again and again. ";
                const ZH: &str = "中文的文本内容在这里中文的文本内容在这里中文的文本，";
                let mut s = String::new();
                let mut w = String::new();
                loop {
                    let start = next.fetch_add(4096, Ordering::Relaxed);
                    if start >= total {
                        break;
                    }
                    for idx in start..(start + 4096).min(total) {
                        // idx -> (length, digits)
                        let (mut k, mut rem) = (1u32, idx);
                        while rem >= n.pow(k) {
                            rem -= n.pow(k);
                            k += 1;
                        }
                        s.clear();
                        for _ in 0..k {
                            s.push_str(units[(rem % n) as usize]);
                            rem /= n;
                        }
                        let variants: usize = if ctx { 7 } else { 1 };
                        for v in 0..variants {
                            w.clear();
                            match v {
                                0 => w.push_str(&s),
                                1 => { w.push_str(EN); w.push_str(&s); }
                                2 => { w.push_str(&s); w.push_str(EN); }
                                3 => { w.push_str(EN); w.push_str(&s); w.push_str(EN); }
                                4 => { w.push_str(ZH); w.push_str(&s); w.push_str(ZH); }
                                5 => { w.push_str(ZH); w.push_str(&s); w.push_str(EN); }
                                _ => { while w.len() < 80 { w.push_str(&s); } }
                            }
                            for (name, f, r) in encs.iter() {
                                if f.encode_ordinary(&w) != r.encode_ordinary(&w) {
                                    if bad.fetch_add(1, Ordering::Relaxed) < 20 {
                                        println!("MISMATCH {name} {w:?}");
                                    }
                                }
                            }
                        }
                    }
                }
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    println!("exhaustive: {total} strings x 4 encodings, {} mismatches", bad.load(Ordering::Relaxed));
}
