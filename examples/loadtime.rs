//! Time to build each shipped encoding, here and in tiktoken's core
//! (tiktoken-rs, which vendors it): best of 5.
use std::time::Instant;

fn best(f: impl Fn()) -> f64 {
    (0..5)
        .map(|_| {
            let s = Instant::now();
            f();
            s.elapsed().as_secs_f64() * 1e3
        })
        .fold(f64::MAX, f64::min)
}

fn main() {
    let ours = best(|| drop(std::hint::black_box(tiktoken_fast::cl100k_base().unwrap())));
    let refr = best(|| drop(std::hint::black_box(tiktoken_rs::cl100k_base().unwrap())));
    println!("cl100k_base: build {ours:.1} ms, tiktoken's core {refr:.1} ms (best of 5)");
    let ours = best(|| drop(std::hint::black_box(tiktoken_fast::o200k_base().unwrap())));
    let refr = best(|| drop(std::hint::black_box(tiktoken_rs::o200k_base().unwrap())));
    println!("o200k_base: build {ours:.1} ms, tiktoken's core {refr:.1} ms (best of 5)");
}
