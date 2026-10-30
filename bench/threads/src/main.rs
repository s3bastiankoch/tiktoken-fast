//! N threads share one encoder and encode chat-sized messages; total throughput per arm.
//!     mt <messages.json> <encoding> <seconds per arm> <threads,...>
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};
fn run<F: Fn(&str) -> usize + Sync>(msgs: &[String], threads: usize, secs: f64, f: F) -> f64 {
    let stop = AtomicBool::new(false);
    let bytes = AtomicU64::new(0);
    let t0 = Instant::now();
    std::thread::scope(|s| {
        for k in 0..threads {
            let (stop, bytes, f) = (&stop, &bytes, &f);
            s.spawn(move || {
                let mut i = k * 997;
                let mut local = 0u64;
                while !stop.load(Ordering::Relaxed) {
                    let m = &msgs[i % msgs.len()];
                    std::hint::black_box(f(m));
                    local += m.len() as u64;
                    i += 1;
                }
                bytes.fetch_add(local, Ordering::Relaxed);
            });
        }
        std::thread::sleep(Duration::from_secs_f64(secs));
        stop.store(true, Ordering::Relaxed);
    });
    bytes.load(Ordering::Relaxed) as f64 / t0.elapsed().as_secs_f64() / 1e6
}
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let msgs: Vec<String> = serde_json::from_str(&std::fs::read_to_string(&a[1]).unwrap()).unwrap();
    let secs: f64 = a[3].parse().unwrap();
    let (r, o, g) = if a[2] == "o200k" {
        (tiktoken_rs::o200k_base().unwrap(), tiktoken_fast::o200k_base().unwrap(), bpe_openai::o200k_base())
    } else {
        (tiktoken_rs::cl100k_base().unwrap(), tiktoken_fast::cl100k_base().unwrap(), bpe_openai::cl100k_base())
    };
    for t in a[4].split(',').map(|x| x.parse::<usize>().unwrap()) {
        let tr = run(&msgs, t, secs, |m| r.encode_ordinary(m).len());
        let to = run(&msgs, t, secs, |m| o.encode_ordinary(m).len());
        let tg = run(&msgs, t, secs, |m| g.encode(m).len());
        println!("{} threads {t}: tiktoken-rs {tr:7.1} MB/s | ours {to:7.1} MB/s ({:.2}x) | bpe-openai {tg:7.1} MB/s ({:.2}x)", a[2], to / tr, tg / tr);
    }
}
