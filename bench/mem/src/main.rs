//! Builds one encoder and encodes one file, so the process's peak memory is
//! the encoder's plus what encoding that file needs. Run under
//! `/usr/bin/time -l` (macOS) or `/usr/bin/time -v` (Linux):
//!
//!     mem <tiktoken|bpe|ours> <cl100k|o200k> <file> [<file>...]
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (arm, enc) = (a[1].as_str(), a[2].as_str());
    let texts: Vec<String> = a[3..].iter().map(|p| std::fs::read_to_string(p).unwrap()).collect();
    let n: usize = match (arm, enc) {
        ("tiktoken", "cl100k") => { let e = tiktoken_rs::cl100k_base().unwrap(); texts.iter().map(|t| e.encode_ordinary(t).len()).sum() }
        ("tiktoken", "o200k") => { let e = tiktoken_rs::o200k_base().unwrap(); texts.iter().map(|t| e.encode_ordinary(t).len()).sum() }
        ("bpe", "cl100k") => { let e = bpe_openai::cl100k_base(); texts.iter().map(|t| e.encode(t).len()).sum() }
        ("bpe", "o200k") => { let e = bpe_openai::o200k_base(); texts.iter().map(|t| e.encode(t).len()).sum() }
        ("ours", "cl100k") => { let e = tiktoken_fast::cl100k_base().unwrap(); texts.iter().map(|t| e.encode_ordinary(t).len()).sum() }
        ("ours", "o200k") => { let e = tiktoken_fast::o200k_base().unwrap(); texts.iter().map(|t| e.encode_ordinary(t).len()).sum() }
        _ => panic!("usage: mem <tiktoken|bpe|ours> <cl100k|o200k> <file>..."),
    };
    println!("{n} tokens");
}
