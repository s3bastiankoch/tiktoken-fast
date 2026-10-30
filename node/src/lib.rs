//! Native Node.js addon (napi-rs) with the same API as the WebAssembly build
//! in ../js and the tiktoken npm package.
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::collections::HashSet;

#[napi]
pub struct Tiktoken {
    inner: tiktoken_fast::CoreBPE,
    specials: Vec<String>,
    firsts: Vec<u8>,
    name: Option<String>,
}

fn firsts(specials: &[String]) -> Vec<u8> {
    let mut f: Vec<u8> = specials.iter().filter(|s| !s.is_empty()).map(|s| s.as_bytes()[0]).collect();
    f.sort_unstable();
    f.dedup();
    f
}

/// "all", an array of strings, or nothing (the default given).
fn token_set(v: Option<Either<String, Vec<String>>>, default_all: bool, specials: &[String]) -> Result<HashSet<String>> {
    match v {
        None => Ok(if default_all { specials.iter().cloned().collect() } else { HashSet::new() }),
        Some(Either::A(s)) if s == "all" => Ok(specials.iter().cloned().collect()),
        Some(Either::A(_)) => Err(Error::new(Status::InvalidArg, "expected \"all\" or an array of strings")),
        Some(Either::B(list)) => Ok(list.into_iter().collect()),
    }
}

impl Tiktoken {
    fn wrap(inner: tiktoken_fast::CoreBPE, name: Option<String>) -> Tiktoken {
        let specials: Vec<String> = inner.special_tokens().into_iter().map(String::from).collect();
        let f = firsts(&specials);
        Tiktoken { inner, specials, firsts: f, name }
    }

    fn first_special(&self, text: &str) -> Option<&str> {
        let b = text.as_bytes();
        let at = |p: usize| self.specials.iter().find(|t| !t.is_empty() && b[p..].starts_with(t.as_bytes())).map(|t| t.as_str());
        match self.firsts.as_slice() {
            [] => None,
            [x] => memchr::memchr_iter(*x, b).find_map(at),
            [x, y] => memchr::memchr2_iter(*x, *y, b).find_map(at),
            [x, y, z] => memchr::memchr3_iter(*x, *y, *z, b).find_map(at),
            _ => self.specials.iter().find(|t| !t.is_empty() && text.contains(t.as_str())).map(|t| t.as_str()),
        }
    }
}

#[napi]
impl Tiktoken {
    #[napi(getter)]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// The tokens of text; special tokens follow tiktoken's rules.
    #[napi]
    pub fn encode(&self, text: String, allowed_special: Option<Either<String, Vec<String>>>, disallowed_special: Option<Either<String, Vec<String>>>) -> Result<Uint32Array> {
        if allowed_special.is_none() && disallowed_special.is_none() {
            if let Some(t) = self.first_special(&text) {
                return Err(Error::new(Status::InvalidArg, format!("The text contains a special token that is not allowed: {t}")));
            }
            return Ok(Uint32Array::new(self.inner.encode_ordinary(&text)));
        }
        let all_disallowed = matches!(&disallowed_special, None) || matches!(&disallowed_special, Some(Either::A(s)) if s == "all");
        let allowed = token_set(allowed_special, false, &self.specials)?;
        let mut disallowed = token_set(disallowed_special, true, &self.specials)?;
        if all_disallowed {
            disallowed.retain(|t| !allowed.contains(t));
        }
        if let Some(t) = disallowed.iter().filter(|t| !t.is_empty()).find(|t| text.contains(t.as_str())) {
            return Err(Error::new(Status::InvalidArg, format!("The text contains a special token that is not allowed: {t}")));
        }
        let allowed: HashSet<&str> = allowed.iter().map(|s| s.as_str()).collect();
        Ok(Uint32Array::new(self.inner.encode(&text, &allowed)))
    }

    /// The tokens of text, special tokens encoded as ordinary text.
    #[napi(js_name = "encode_ordinary")]
    pub fn encode_ordinary(&self, text: String) -> Uint32Array {
        Uint32Array::new(self.inner.encode_ordinary(&text))
    }

    /// The number of tokens encode_ordinary would return.
    #[napi]
    pub fn count(&self, text: String) -> u32 {
        self.inner.encode_ordinary(&text).len() as u32
    }

    /// The bytes of a token sequence.
    #[napi]
    pub fn decode(&self, tokens: Uint32Array) -> Result<Uint8Array> {
        self.inner.decode_bytes(&tokens).map(Uint8Array::new).map_err(|e| Error::new(Status::InvalidArg, e.to_string()))
    }

    /// Kept for the tiktoken API; memory is freed by the garbage collector.
    #[napi]
    pub fn free(&self) {}

    /// Drops the per-caller caches. For benchmarks.
    #[napi(js_name = "reset_caches")]
    pub fn reset_caches(&self) {
        self.inner.reset_caches()
    }
}

/// One of the built-in encodings, as tiktoken's get_encoding.
#[napi(js_name = "get_encoding")]
pub fn get_encoding(encoding: String) -> Result<Tiktoken> {
    let r = match encoding.as_str() {
        "cl100k_base" => tiktoken_fast::cl100k_base(),
        "o200k_base" => tiktoken_fast::o200k_base(),
        "p50k_base" => tiktoken_fast::p50k_base(),
        "p50k_edit" => tiktoken_fast::p50k_edit(),
        "r50k_base" | "gpt2" => tiktoken_fast::r50k_base(),
        _ => return Err(Error::new(Status::InvalidArg, format!("Unknown encoding {encoding}"))),
    };
    r.map(|inner| Tiktoken::wrap(inner, Some(encoding))).map_err(|e| Error::new(Status::GenericFailure, e.to_string()))
}

/// The encoding of an OpenAI model, as tiktoken's encoding_for_model.
#[napi(js_name = "encoding_for_model")]
pub fn encoding_for_model(model: String) -> Result<Tiktoken> {
    let m = model.as_str();
    let starts = |ps: &[&str]| ps.iter().any(|p| m.starts_with(p));
    let name = if starts(&["gpt-4o", "gpt-4.1", "gpt-4.5", "gpt-5", "o1", "o3", "o4", "chatgpt-4o"]) {
        "o200k_base"
    } else if starts(&["gpt-4", "gpt-3.5", "gpt-35", "text-embedding-ada-002", "text-embedding-3"]) {
        "cl100k_base"
    } else if starts(&["text-davinci-edit", "code-davinci-edit"]) {
        "p50k_edit"
    } else if starts(&["text-davinci-002", "text-davinci-003", "code-davinci", "code-cushman"]) {
        "p50k_base"
    } else if starts(&["gpt2", "davinci", "curie", "babbage", "ada", "text-"]) {
        "r50k_base"
    } else {
        return Err(Error::new(Status::InvalidArg, format!("Could not automatically map {model} to a tokeniser.")));
    };
    get_encoding(name.to_string())
}
