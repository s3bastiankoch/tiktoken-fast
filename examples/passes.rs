//! Encodes one file over and over and reports the best pass, for profiling.
//!
//!     cargo run --release --example passes -- <file> <cl100k|o200k> <seconds> [cold|warm|cross <other file>]
//!
//! `cold` drops the caches before every pass, `warm` keeps them, `cross`
//! warms them on another file first and times only the pass over this one.
use std::time::Instant;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let text = std::fs::read_to_string(&a[1]).unwrap();
    let enc = match a[2].as_str() { "o200k" => tiktoken_fast::o200k_base(), _ => tiktoken_fast::cl100k_base() }.unwrap();
    let secs: f64 = a[3].parse().unwrap();
    let mode = a.get(4).map(|s| s.as_str()).unwrap_or("cold");
    let t0 = Instant::now();
    let (mut n, mut tok, mut timed) = (0u64, 0usize, 0f64);
    let mut best = f64::MAX;
    let other: String = a.get(5).map(|p| std::fs::read_to_string(p).unwrap()).unwrap_or_default();
    while t0.elapsed().as_secs_f64() < secs {
        if mode == "cold" { enc.reset_caches(); }
        if mode != "cross" {
            let s = Instant::now();
            tok += enc.encode_ordinary(&text).len();
            best = best.min(s.elapsed().as_secs_f64());
            n += 1;
            continue;
        }
        if mode == "cross" {
            enc.reset_caches();
            enc.encode_ordinary(&other);
            let s = Instant::now();
            tok += enc.encode_ordinary(&text).len();
            timed += s.elapsed().as_secs_f64();
            n += 1;
            continue;
        }
        tok += enc.encode_ordinary(&text).len();
        n += 1;
    }
    let dt = if mode == "cross" { timed } else { t0.elapsed().as_secs_f64() };
    println!("best {:.4} ms", best * 1e3);
    println!("{mode}: {n} passes, {:.2} ms/pass, {:.1} MB/s, {} tokens/pass", dt * 1e3 / n as f64, text.len() as f64 * n as f64 / dt / 1e6, tok / n as usize);
}
