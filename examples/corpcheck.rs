//! Parity on real files: every file under the given directories, whole and
//! line by line, in all four encodings, on encoders shared by all threads.
//!
//!     cargo run --release --example corpcheck -- <dir> [dir ...]

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(m) = std::fs::symlink_metadata(p) {
        if m.is_dir() {
            if let Ok(rd) = std::fs::read_dir(p) {
                for e in rd.flatten() {
                    walk(&e.path(), out);
                }
            }
        } else if m.is_file() && m.len() > 0 && m.len() <= 1 << 20 {
            out.push(p.to_path_buf());
        }
    }
}

fn main() {
    let dirs: Vec<String> = std::env::args().skip(1).collect();
    let mut files = Vec::new();
    for d in &dirs {
        walk(Path::new(d), &mut files);
    }
    let fast = Arc::new(vec![
        ("r50k_base", tiktoken_fast::r50k_base().unwrap(), tiktoken_rs::r50k_base().unwrap()),
        ("p50k_base", tiktoken_fast::p50k_base().unwrap(), tiktoken_rs::p50k_base().unwrap()),
        ("cl100k_base", tiktoken_fast::cl100k_base().unwrap(), tiktoken_rs::cl100k_base().unwrap()),
        ("o200k_base", tiktoken_fast::o200k_base().unwrap(), tiktoken_rs::o200k_base().unwrap()),
    ]);
    let files = Arc::new(files);
    let next = Arc::new(AtomicUsize::new(0));
    let (nfiles, nbytes, nlines) = (Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)), Arc::new(AtomicU64::new(0)));
    let fails: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let threads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
    eprintln!("corpcheck: {} files, {threads} threads", files.len());
    let hs: Vec<_> = (0..threads)
        .map(|_| {
            let (fast, files, next, nfiles, nbytes, nlines, fails) = (fast.clone(), files.clone(), next.clone(), nfiles.clone(), nbytes.clone(), nlines.clone(), fails.clone());
            std::thread::spawn(move || loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(path) = files.get(i) else { break };
                let Ok(b) = std::fs::read(path) else { continue };
                if b.contains(&0) {
                    continue;
                }
                let text = String::from_utf8_lossy(&b);
                for (name, f, r) in fast.iter() {
                    if f.encode_ordinary(&text) != r.encode_ordinary(&text) {
                        fails.lock().unwrap().push(format!("{name} whole {}", path.display()));
                    }
                    for line in text.split_inclusive('\n') {
                        if f.encode_ordinary(line) != r.encode_ordinary(line) {
                            fails.lock().unwrap().push(format!("{name} line {} {:?}", path.display(), line));
                        }
                        nlines.fetch_add(1, Ordering::Relaxed);
                    }
                }
                nfiles.fetch_add(1, Ordering::Relaxed);
                nbytes.fetch_add(b.len() as u64, Ordering::Relaxed);
            })
        })
        .collect();
    for h in hs {
        h.join().unwrap();
    }
    let fails = fails.lock().unwrap();
    for f in fails.iter().take(50) {
        println!("MISMATCH {f}");
    }
    println!("corpcheck: {} files, {} MB, {} line checks, {} mismatches", nfiles.load(Ordering::Relaxed), nbytes.load(Ordering::Relaxed) >> 20, nlines.load(Ordering::Relaxed), fails.len());
}
