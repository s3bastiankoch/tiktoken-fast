//! Single-threaded comparison of tiktoken-rs (the reference core), tiktoken-fast
//! and bpe-openai on the workloads written by `bench/data/make_workloads.py`.
//!
//!     cargo run --release -- <workload dir>
//!
//! ENCS picks encodings (default cl100k_base,o200k_base,llama4), ROUNDS the
//! number of rounds (default 11). Each round times every arm on the whole
//! workload, rotating the arm order; a row reports each arm's median speed-up
//! over the reference with a 95% bootstrap interval, and how many documents
//! differ from the reference (`mism`).
//!
//! With RAW=<file>, every timed round of every arm is appended to that file as
//! one JSON line, and every arm's check against the reference as another;
//! `bench/summarize.py` turns those into the published tables.
//!
//! Scenarios: `stream` encodes each document once per round, with
//! tiktoken-fast's caches dropped before the round; `repeat` encodes the same
//! documents again with warm caches (round 0 dropped); `cold` drops the caches
//! before every document.
use base64::Engine;
use std::collections::HashMap;
use std::hint::black_box;
use std::io::Write;
use std::time::Instant;

fn med(v: &[f64]) -> f64 { let mut v = v.to_vec(); v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v[v.len() / 2] }
struct Lcg(u64);
impl Lcg { fn next(&mut self) -> usize { self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407); (self.0 >> 33) as usize } }
fn ci(xs: &[f64], rng: &mut Lcg) -> (f64, f64) {
    let mut m: Vec<f64> = (0..2000).map(|_| { let s: Vec<f64> = (0..xs.len()).map(|_| xs[rng.next() % xs.len()]).collect(); med(&s) }).collect();
    m.sort_by(|a, b| a.partial_cmp(b).unwrap()); (m[50], m[1949])
}

fn main() {
    let dir = std::env::args().nth(1).unwrap();
    let rounds: usize = std::env::var("ROUNDS").ok().and_then(|v| v.parse().ok()).unwrap_or(11);
    let mut names: Vec<String> = std::fs::read_dir(&dir).unwrap().filter_map(|e| { let n = e.unwrap().file_name().into_string().unwrap(); n.strip_suffix(".json").map(String::from) }).collect();
    names.sort();
    let works: Vec<(String, Vec<String>)> = names.iter().map(|n| (n.clone(), serde_json::from_str(&std::fs::read_to_string(format!("{dir}/{n}.json")).unwrap()).unwrap())).collect();
    let mut rng = Lcg(7);
    let mut raw = std::env::var("RAW").ok().map(|p| std::fs::OpenOptions::new().create(true).append(true).open(p).unwrap());
    let arm_names = ["tiktoken-rs", "tiktoken-fast", "bpe-openai"];
    for enc in std::env::var("ENCS").unwrap_or("cl100k_base,o200k_base,llama4".into()).split(',').map(String::from).collect::<Vec<_>>().iter().map(|s| s.as_str()) {
        let (reference, ours, gh): (tiktoken_rs::CoreBPE, tiktoken_fast::CoreBPE, Option<&bpe_openai::Tokenizer>) = match enc {
            "cl100k_base" => (tiktoken_rs::cl100k_base().unwrap(), tiktoken_fast::cl100k_base().unwrap(), Some(bpe_openai::cl100k_base())),
            "o200k_base" => (tiktoken_rs::o200k_base().unwrap(), tiktoken_fast::o200k_base().unwrap(), Some(bpe_openai::o200k_base())),
            "p50k_base" => (tiktoken_rs::p50k_base().unwrap(), tiktoken_fast::p50k_base().unwrap(), None),
            "r50k_base" => (tiktoken_rs::r50k_base().unwrap(), tiktoken_fast::r50k_base().unwrap(), None),
            _ => {
                let mut ranks: HashMap<Vec<u8>, u32> = HashMap::new();
                for l in std::fs::read_to_string(format!("{dir}/llama4.tiktoken")).unwrap().lines() {
                    let mut it = l.split(' ');
                    let b = base64::engine::general_purpose::STANDARD.decode(it.next().unwrap()).unwrap();
                    ranks.insert(b, it.next().unwrap().parse().unwrap());
                }
                let pat_str = o200k_pattern();
                let rref = tiktoken_rs::CoreBPE::new(ranks.iter().map(|(k, v)| (k.clone(), *v)).collect::<rustc_hash::FxHashMap<_, _>>(), Default::default(), &pat_str).unwrap();
                (rref, tiktoken_fast::CoreBPE::new(ranks, HashMap::new(), &pat_str).unwrap(), None)
            }
        };
        for (wname, docs) in &works {
            let nbytes: usize = docs.iter().map(|d| d.len()).sum();
            let mism_ours = docs.iter().filter(|d| ours.encode_ordinary(d) != reference.encode_ordinary(d)).count();
            let mism_gh = gh.map(|g| docs.iter().filter(|d| g.encode(d) != reference.encode_ordinary(d)).count());
            if let Some(f) = raw.as_mut() {
                writeln!(f, r#"{{"kind":"check","lang":"rust","enc":"{enc}","workload":"{wname}","arm":"tiktoken-fast","docs":{},"bytes":{nbytes},"mism":{mism_ours}}}"#, docs.len()).unwrap();
                if let Some(m) = mism_gh { writeln!(f, r#"{{"kind":"check","lang":"rust","enc":"{enc}","workload":"{wname}","arm":"bpe-openai","docs":{},"bytes":{nbytes},"mism":{m}}}"#, docs.len()).unwrap(); }
            }
            let mut scen = vec!["stream", "repeat"]; if docs.len() > 1 { scen.push("cold"); }
            for sc in scen {
                let narms = if gh.is_some() { 3 } else { 2 };
                let mut t = vec![Vec::new(); narms];
                let extra = if sc == "repeat" { 1 } else { 0 };
                for r in 0..rounds + extra {
                    for k in 0..narms {
                        let a = (k + r) % narms;
                        if a == 1 && sc != "repeat" { ours.reset_caches(); }
                        let mut tot = 0.0;
                        for d in docs {
                            if a == 1 && sc == "cold" { ours.reset_caches(); }
                            let s = Instant::now();
                            let n = match a { 0 => reference.encode_ordinary(black_box(d)).len(), 1 => ours.encode_ordinary(black_box(d)).len(), _ => gh.unwrap().encode(black_box(d)).len() };
                            tot += s.elapsed().as_secs_f64(); black_box(n);
                        }
                        if !(sc == "repeat" && r == 0) {
                            t[a].push(tot);
                            if let Some(f) = raw.as_mut() {
                                writeln!(f, r#"{{"kind":"time","lang":"rust","enc":"{enc}","workload":"{wname}","scenario":"{sc}","call":"encode_ordinary","arm":"{}","round":{},"seconds":{tot:e},"bytes":{nbytes}}}"#, arm_names[a], r - extra).unwrap();
                            }
                        }
                    }
                }
                let sp = |a: usize| -> Vec<f64> { t[0].iter().zip(&t[a]).map(|(x, y)| x / y).collect() };
                let so = sp(1); let (lo, hi) = ci(&so, &mut rng);
                let mut line = format!("{enc:11} {wname:18} {sc:6} tiktoken-rs {:6.1} MB/s | ours {:5.2}x [{lo:.2},{hi:.2}] {:6.0} MB/s mism {mism_ours}",
                    nbytes as f64 / med(&t[0]) / 1e6, med(&so), nbytes as f64 / med(&t[1]) / 1e6);
                if narms == 3 { let sg = sp(2); let (l2, h2) = ci(&sg, &mut rng); line += &format!(" | bpe-openai {:5.2}x [{l2:.2},{h2:.2}] mism {}", med(&sg), mism_gh.unwrap()); }
                println!("{line}");
            }
        }
    }
}

/// o200k_base's split pattern, which Llama 4 uses as well.
fn o200k_pattern() -> String {
    [r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]*[\p{Ll}\p{Lm}\p{Lo}\p{M}]+(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
     r"[^\r\n\p{L}\p{N}]?[\p{Lu}\p{Lt}\p{Lm}\p{Lo}\p{M}]+[\p{Ll}\p{Lm}\p{Lo}\p{M}]*(?i:'s|'t|'re|'ve|'m|'ll|'d)?",
     r"\p{N}{1,3}", r" ?[^\s\p{L}\p{N}]+[\r\n/]*", r"\s*[\r\n]+", r"\s+(?!\S)", r"\s+"].join("|")
}
