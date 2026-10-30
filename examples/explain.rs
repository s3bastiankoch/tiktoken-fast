//! Shows how one string encodes here and in the reference core, token by token.
//!
//!     cargo run --release --example explain -- <encoding> <text>
//!
//! Encodings: cl100k_base, o200k_base, p50k_base, r50k_base.
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let (name, text) = (a.get(1).expect("encoding"), a.get(2).expect("text"));
    let (fast, reference) = match name.as_str() {
        "o200k_base" => (tiktoken_fast::o200k_base(), tiktoken_rs::o200k_base()),
        "p50k_base" => (tiktoken_fast::p50k_base(), tiktoken_rs::p50k_base()),
        "r50k_base" => (tiktoken_fast::r50k_base(), tiktoken_rs::r50k_base()),
        _ => (tiktoken_fast::cl100k_base(), tiktoken_rs::cl100k_base()),
    };
    let (fast, reference) = (fast.unwrap(), reference.unwrap());
    let (ours, theirs) = (fast.encode_ordinary(text), reference.encode_ordinary(text));
    let show = |ids: &[u32]| {
        ids.iter()
            .map(|&i| format!("{i}:{:?}", String::from_utf8_lossy(&reference.decode_bytes(&[i]).unwrap())))
            .collect::<Vec<_>>()
            .join(" ")
    };
    println!("{text:?}\n  here      {}\n  reference {}", show(&ours), show(&theirs));
    if ours != theirs {
        std::process::exit(1);
    }
}
