//! Run with `cargo run --release --example fastarm`.
//! Round server for the cross-implementation benchmark driver in
//! tiktoken-go-fast/bench/vsrust: the same line protocol as its two arms.
//!
//!     load <encoding>   -> ok
//!     open <path>       -> ok <bytes>
//!     check             -> ok <ntokens> <fnv1a64 of the ids>
//!     run <iters>       -> ok <elapsed_ns> <sink>
//!     quit

use std::hint::black_box;
use std::io::{self, BufRead, Write};
use std::time::Instant;

use tiktoken_fast::{cl100k_base, o200k_base, p50k_base, r50k_base, CoreBPE};

fn digest(ids: &[u32]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for id in ids {
        for byte in id.to_le_bytes() {
            h ^= byte as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

fn load(name: &str) -> Option<CoreBPE> {
    match name {
        "cl100k_base" => cl100k_base().ok(),
        "o200k_base" => o200k_base().ok(),
        "p50k_base" => p50k_base().ok(),
        "r50k_base" => r50k_base().ok(),
        _ => None,
    }
}

fn main() {
    // FASTARM_CHUNK=bytes makes `run` and `check` use encode_ordinary_parallel
    // with that target chunk size on rayon's pool (RAYON_NUM_THREADS sizes it)
    let chunk: usize = std::env::var("FASTARM_CHUNK").ok().and_then(|v| v.parse().ok()).unwrap_or(0);
    let encode = |enc: &CoreBPE, text: &str| if chunk > 0 { enc.encode_ordinary_parallel(text, chunk) } else { enc.encode_ordinary(text) };
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut bpe: Option<CoreBPE> = None;
    let mut text = String::new();
    for line in stdin.lock().lines() {
        let line = line.expect("read stdin");
        let mut fields = line.split_whitespace();
        let Some(cmd) = fields.next() else { continue };
        match cmd {
            "load" => {
                let name = fields.next().unwrap_or("");
                match load(name) {
                    Some(enc) => {
                        bpe = Some(enc);
                        writeln!(stdout, "ok").unwrap();
                    }
                    None => writeln!(stdout, "err unknown encoding {name}").unwrap(),
                }
            }
            "open" => {
                let path = fields.next().unwrap_or("");
                match std::fs::read_to_string(path) {
                    Ok(s) => {
                        let n = s.len();
                        text = s;
                        writeln!(stdout, "ok {n}").unwrap();
                    }
                    Err(e) => writeln!(stdout, "err {e}").unwrap(),
                }
            }
            "check" => {
                let enc = bpe.as_ref().expect("load first");
                let ids = encode(enc, &text);
                writeln!(stdout, "ok {} {:016x}", ids.len(), digest(&ids)).unwrap();
            }
            "run" => {
                let enc = bpe.as_ref().expect("load first");
                let iters: usize = fields.next().and_then(|s| s.parse().ok()).unwrap_or(1);
                let start = Instant::now();
                let mut sink = 0usize;
                for _ in 0..iters {
                    sink = sink.wrapping_add(black_box(encode(enc, &text)).len());
                }
                writeln!(stdout, "ok {} {}", start.elapsed().as_nanos(), sink).unwrap();
            }
            "quit" => break,
            other => writeln!(stdout, "err unknown command {other}").unwrap(),
        }
        stdout.flush().unwrap();
    }
}
