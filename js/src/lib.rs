//! WebAssembly build for JavaScript, with the API of the `tiktoken` npm
//! package: get_encoding, encoding_for_model, Tiktoken with encode,
//! encode_ordinary, decode and free. Plus count, which tiktoken lacks.
use std::collections::{HashMap, HashSet};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Tiktoken {
    inner: tiktoken_fast::CoreBPE,
    specials: Vec<String>,
    /// the distinct first bytes of the special tokens
    firsts: Vec<u8>,
    name: Option<String>,
}

fn firsts(specials: &[String]) -> Vec<u8> {
    let mut f: Vec<u8> = specials.iter().filter(|s| !s.is_empty()).map(|s| s.as_bytes()[0]).collect();
    f.sort_unstable();
    f.dedup();
    f
}

/// "all", an array of strings, or undefined (the default given).
fn token_set(v: &JsValue, default_all: bool, specials: &[String]) -> Result<HashSet<String>, JsError> {
    if v.is_undefined() || v.is_null() {
        return Ok(if default_all { specials.iter().cloned().collect() } else { HashSet::new() });
    }
    if let Some(s) = v.as_string() {
        if s == "all" {
            return Ok(specials.iter().cloned().collect());
        }
        return Err(JsError::new("expected \"all\" or an array of strings"));
    }
    let arr: js_sys::Array = v.clone().dyn_into().map_err(|_| JsError::new("expected \"all\" or an array of strings"))?;
    arr.iter().map(|x| x.as_string().ok_or_else(|| JsError::new("expected strings"))).collect()
}

impl Tiktoken {
    /// The first special token found in text, if any.
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

#[wasm_bindgen]
impl Tiktoken {
    /// A tokenizer from the text of a .tiktoken file (base64 token, space,
    /// rank, per line), an object of special tokens and the split pattern.
    #[wasm_bindgen(constructor)]
    pub fn new(tiktoken_bfe: &str, special_tokens: JsValue, pat_str: &str) -> Result<Tiktoken, JsError> {
        use base64::Engine;
        let mut ranks = HashMap::new();
        for line in tiktoken_bfe.lines() {
            let mut it = line.split(' ');
            let (Some(tok), Some(rank)) = (it.next(), it.next()) else { continue };
            let bytes = base64::engine::general_purpose::STANDARD.decode(tok).map_err(|e| JsError::new(&e.to_string()))?;
            ranks.insert(bytes, rank.parse::<u32>().map_err(|e| JsError::new(&e.to_string()))?);
        }
        let mut specials = HashMap::new();
        if !special_tokens.is_undefined() && !special_tokens.is_null() {
            let obj: js_sys::Object = special_tokens.dyn_into().map_err(|_| JsError::new("special_tokens must be an object"))?;
            for e in js_sys::Object::entries(&obj).iter() {
                let e: js_sys::Array = e.into();
                let k = e.get(0).as_string().ok_or_else(|| JsError::new("special token names must be strings"))?;
                let v = e.get(1).as_f64().ok_or_else(|| JsError::new("special token ids must be numbers"))? as u32;
                specials.insert(k, v);
            }
        }
        let names: Vec<String> = specials.keys().cloned().collect();
        let inner = tiktoken_fast::CoreBPE::new(ranks, specials, pat_str).map_err(|e| JsError::new(&e.to_string()))?;
        let f = firsts(&names);
        Ok(Tiktoken { inner, specials: names, firsts: f, name: None })
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> Option<String> {
        self.name.clone()
    }

    /// The tokens of text. Special tokens in allowed_special ("all" or a
    /// list) become their ids; any in disallowed_special (default "all" but
    /// the allowed) found in the text throws, as in tiktoken.
    pub fn encode(&self, text: &str, allowed_special: JsValue, disallowed_special: JsValue) -> Result<Vec<u32>, JsError> {
        let unset = |v: &JsValue| v.is_undefined() || v.is_null();
        if unset(&allowed_special) && unset(&disallowed_special) {
            // the default: no special token allowed, all disallowed
            if let Some(t) = self.first_special(text) {
                return Err(JsError::new(&format!("The text contains a special token that is not allowed: {t}")));
            }
            return Ok(self.inner.encode_ordinary(text));
        }
        let allowed = token_set(&allowed_special, false, &self.specials)?;
        let mut disallowed = token_set(&disallowed_special, true, &self.specials)?;
        if disallowed_special.as_string().as_deref() == Some("all") || disallowed_special.is_undefined() || disallowed_special.is_null() {
            disallowed.retain(|t| !allowed.contains(t));
        }
        if let Some(t) = disallowed.iter().filter(|t| !t.is_empty()).find(|t| text.contains(t.as_str())) {
            return Err(JsError::new(&format!("The text contains a special token that is not allowed: {t}")));
        }
        let allowed: HashSet<&str> = allowed.iter().map(|s| s.as_str()).collect();
        Ok(self.inner.encode(text, &allowed))
    }

    /// The tokens of text, special tokens encoded as ordinary text.
    pub fn encode_ordinary(&self, text: &str) -> Vec<u32> {
        self.inner.encode_ordinary(text)
    }

    /// The number of tokens encode_ordinary would return.
    pub fn count(&self, text: &str) -> usize {
        self.inner.encode_ordinary(text).len()
    }

    /// The bytes of a token sequence (UTF-8 when the sequence is whole).
    pub fn decode(&self, tokens: &[u32]) -> Result<Vec<u8>, JsError> {
        self.inner.decode_bytes(tokens).map_err(|e| JsError::new(&e.to_string()))
    }

    /// Drops the per-caller caches. For benchmarks.
    pub fn reset_caches(&self) {
        self.inner.reset_caches()
    }
}

fn built_in(name: &str) -> Result<tiktoken_fast::CoreBPE, JsError> {
    let r = match name {
        "cl100k_base" => tiktoken_fast::cl100k_base(),
        "o200k_base" => tiktoken_fast::o200k_base(),
        "p50k_base" => tiktoken_fast::p50k_base(),
        "p50k_edit" => tiktoken_fast::p50k_edit(),
        "r50k_base" | "gpt2" => tiktoken_fast::r50k_base(),
        _ => return Err(JsError::new(&format!("Unknown encoding {name}"))),
    };
    r.map_err(|e| JsError::new(&e.to_string()))
}

/// One of the built-in encodings, as tiktoken's get_encoding.
#[wasm_bindgen]
pub fn get_encoding(encoding: &str) -> Result<Tiktoken, JsError> {
    let inner = built_in(encoding)?;
    let specials: Vec<String> = inner.special_tokens().into_iter().map(String::from).collect();
    let f = firsts(&specials);
    Ok(Tiktoken { inner, specials, firsts: f, name: Some(encoding.to_string()) })
}

/// The encoding of an OpenAI model, as tiktoken's encoding_for_model.
#[wasm_bindgen]
pub fn encoding_for_model(model: &str) -> Result<Tiktoken, JsError> {
    let name = if model.starts_with("gpt-4o") || model.starts_with("gpt-4.1") || model.starts_with("gpt-4.5") || model.starts_with("gpt-5") || model.starts_with("o1") || model.starts_with("o3") || model.starts_with("o4") || model.starts_with("chatgpt-4o") {
        "o200k_base"
    } else if model.starts_with("gpt-4") || model.starts_with("gpt-3.5") || model.starts_with("gpt-35") || model.starts_with("text-embedding-ada-002") || model.starts_with("text-embedding-3") {
        "cl100k_base"
    } else if model.starts_with("text-davinci-002") || model.starts_with("text-davinci-003") || model.starts_with("code-davinci") || model.starts_with("code-cushman") {
        "p50k_base"
    } else if model.starts_with("text-davinci-edit") || model.starts_with("code-davinci-edit") {
        "p50k_edit"
    } else if model.starts_with("gpt2") || model.starts_with("davinci") || model.starts_with("curie") || model.starts_with("babbage") || model.starts_with("ada") || model.starts_with("text-") {
        "r50k_base"
    } else {
        return Err(JsError::new(&format!("Could not automatically map {model} to a tokeniser.")));
    };
    get_encoding(name)
}
