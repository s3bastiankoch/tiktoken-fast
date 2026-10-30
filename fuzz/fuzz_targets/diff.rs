//! Differential target: every entry point of tiktoken_fast against the
//! reference core. The encoders live for the whole run, so the pooled caches
//! carry state from one input to the next; every 4096th input drops them.

#![no_main]

use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::OnceLock;

use libfuzzer_sys::fuzz_target;

struct Pair {
    name: &'static str,
    fast: tiktoken_fast::CoreBPE,
    reference: tiktoken_rs::CoreBPE,
}

static PAIRS: OnceLock<Vec<Pair>> = OnceLock::new();
static CALLS: AtomicU64 = AtomicU64::new(0);

fn pairs() -> &'static [Pair] {
    PAIRS.get_or_init(|| {
        vec![
            Pair { name: "r50k_base", fast: tiktoken_fast::r50k_base().unwrap(), reference: tiktoken_rs::r50k_base().unwrap() },
            Pair { name: "p50k_base", fast: tiktoken_fast::p50k_base().unwrap(), reference: tiktoken_rs::p50k_base().unwrap() },
            Pair { name: "cl100k_base", fast: tiktoken_fast::cl100k_base().unwrap(), reference: tiktoken_rs::cl100k_base().unwrap() },
            Pair { name: "o200k_base", fast: tiktoken_fast::o200k_base().unwrap(), reference: tiktoken_rs::o200k_base().unwrap() },
        ]
    })
}

/// FUZZ_ENC=r50k|p50k|cl100k|o200k pins the encoding; otherwise the first byte picks it.
fn pinned() -> Option<usize> {
    static PIN: OnceLock<Option<usize>> = OnceLock::new();
    *PIN.get_or_init(|| match std::env::var("FUZZ_ENC").as_deref() {
        Ok("r50k") => Some(0),
        Ok("p50k") => Some(1),
        Ok("cl100k") => Some(2),
        Ok("o200k") => Some(3),
        _ => None,
    })
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 2 {
        return;
    }
    let ps = pairs();
    let sel = data[0];
    let flags = data[1];
    let p = &ps[pinned().unwrap_or((sel & 3) as usize)];
    let text = String::from_utf8_lossy(&data[2..]);
    let text: &str = &text;

    if CALLS.fetch_add(1, Ordering::Relaxed) % 4096 == 4095 {
        p.fast.reset_caches();
    }

    let want = p.reference.encode_ordinary(text);
    let got = p.fast.encode_ordinary(text);
    assert_eq!(got, want, "{}: encode_ordinary {:?}", p.name, text);

    let back = p.fast.decode_bytes(&got).unwrap();
    assert_eq!(back, text.as_bytes(), "{}: decode {:?}", p.name, text);

    if flags & 1 != 0 {
        let chunk = 1 + (flags >> 2) as usize * 3;
        let par = p.fast.encode_ordinary_parallel(text, chunk);
        assert_eq!(par, want, "{}: parallel chunk {chunk} {:?}", p.name, text);
    }
    if flags & 2 != 0 {
        let all = p.reference.special_tokens();
        assert_eq!(p.fast.encode_with_special_tokens(text), p.reference.encode_with_special_tokens(text), "{}: with specials {:?}", p.name, text);
        let mut names: Vec<&str> = all.iter().copied().collect();
        names.sort();
        let subset: HashSet<&str> = names.iter().enumerate().filter(|(i, _)| (sel >> 2) & (1 << (i % 6)) != 0).map(|(_, s)| *s).collect();
        let want_sub = p.reference.encode(text, &subset).map(|r| r.0);
        if let Ok(w) = want_sub {
            assert_eq!(p.fast.encode(text, &subset), w, "{}: subset {:?} {:?}", p.name, subset, text);
        }
    }
});
