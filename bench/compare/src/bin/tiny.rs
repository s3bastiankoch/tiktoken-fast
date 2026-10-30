//! Per-call times on one workload, nanoseconds per document: tiktoken-rs,
//! bpe-openai, and tiktoken-fast with caches kept (stream) and dropped before
//! every document (cold).
//!
//!     cargo run --release --bin tiny -- <workload.json> [cl100k_base|o200k_base]
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let enc = std::env::args().nth(2).unwrap_or("cl100k_base".into());
    let docs: Vec<String> = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    let (r, o, g) = if enc == "o200k_base" {
        (tiktoken_rs::o200k_base().unwrap(), tiktoken_fast::o200k_base().unwrap(), bpe_openai::o200k_base())
    } else {
        (tiktoken_rs::cl100k_base().unwrap(), tiktoken_fast::cl100k_base().unwrap(), bpe_openai::cl100k_base())
    };
    let n = docs.len() as f64;
    for _ in 0..5 {
        let time = |f: &dyn Fn(&str) -> usize| { let s = Instant::now(); for d in &docs { black_box(f(black_box(d))); } s.elapsed().as_secs_f64() / n * 1e9 };
        let tr = time(&|d| r.encode_ordinary(d).len());
        let tg = time(&|d| g.encode(d).len());
        o.reset_caches();
        let ts = time(&|d| o.encode_ordinary(d).len());
        let mut tc = 0.0;
        for d in &docs { o.reset_caches(); let s = Instant::now(); black_box(o.encode_ordinary(black_box(d))); tc += s.elapsed().as_secs_f64(); }
        let ts2 = time(&|d| o.encode_ordinary(d).len());
        println!("ns/doc  tiktoken-rs {tr:.0}  bpe-openai {tg:.0}  ours stream {ts:.0}  ours cold {:.0}  stream-again {ts2:.0}", tc / n * 1e9);
    }
}
