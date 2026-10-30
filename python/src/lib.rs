//! The Python extension: a CoreBPE with the methods tiktoken's Encoding calls
//! on its core, plus batch encoders that run on native threads.
use pyo3::exceptions::{PyKeyError, PyValueError};
use pyo3::prelude::*;
use pyo3::pybacked::PyBackedStr;
use pyo3::types::{PyBytes, PyDict, PyString};
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

#[pyclass(module = "tiktoken_fast._core", subclass, frozen)]
struct CoreBPE {
    inner: tiktoken_fast::CoreBPE,
    /// the special tokens, and the distinct bytes they start with
    specials: Vec<String>,
    firsts: Vec<u8>,
}

/// Texts shorter than this are encoded without releasing the GIL, which
/// costs more than encoding them.
const RELEASE_FROM: usize = 1024;

/// The text of s as UTF-8; a string with lone surrogates is first cleaned
/// as tiktoken's Python layer does (UTF-16 round trip with "replace").
fn utf8<'a>(s: &'a Bound<'_, PyString>) -> PyResult<std::borrow::Cow<'a, str>> {
    match s.to_str() {
        Ok(t) => Ok(std::borrow::Cow::Borrowed(t)),
        Err(_) => {
            let fixed = s.call_method1("encode", ("utf-16", "surrogatepass"))?.call_method1("decode", ("utf-16", "replace"))?;
            Ok(std::borrow::Cow::Owned(fixed.downcast::<PyString>()?.to_str()?.to_owned()))
        }
    }
}

impl CoreBPE {
    /// Runs f on every text, on up to `threads` threads, keeping the order.
    fn map_texts<F>(&self, texts: &[PyBackedStr], threads: usize, f: F) -> Vec<Vec<u32>>
    where
        F: Fn(&str) -> Vec<u32> + Sync,
    {
        let n = texts.len();
        let threads = threads.clamp(1, 64).min(n.max(1));
        if threads == 1 {
            return texts.iter().map(|t| f(t)).collect();
        }
        let next = AtomicUsize::new(0);
        let parts: Vec<Vec<(usize, Vec<u32>)>> = std::thread::scope(|s| {
            let workers: Vec<_> = (0..threads)
                .map(|_| {
                    s.spawn(|| {
                        let mut done = Vec::new();
                        loop {
                            let i = next.fetch_add(1, Ordering::Relaxed);
                            if i >= n {
                                break done;
                            }
                            done.push((i, f(&texts[i])));
                        }
                    })
                })
                .collect();
            workers.into_iter().map(|w| w.join().expect("encoder thread")).collect()
        });
        let mut out = vec![Vec::new(); n];
        for (i, v) in parts.into_iter().flatten() {
            out[i] = v;
        }
        out
    }

    /// Whether text contains any special token.
    fn has_special(&self, text: &str) -> bool {
        let b = text.as_bytes();
        let at = |p: usize| self.specials.iter().any(|t| b[p..].starts_with(t.as_bytes()));
        match self.firsts.as_slice() {
            [] => false,
            [x] => memchr::memchr_iter(*x, b).any(at),
            [x, y] => memchr::memchr2_iter(*x, *y, b).any(at),
            [x, y, z] => memchr::memchr3_iter(*x, *y, *z, b).any(at),
            _ => self.specials.iter().any(|t| text.contains(t.as_str())),
        }
    }

    fn ordinary(&self, py: Python<'_>, text: &str) -> Vec<u32> {
        if text.len() < RELEASE_FROM {
            self.inner.encode_ordinary(text)
        } else {
            py.allow_threads(|| self.inner.encode_ordinary(text))
        }
    }

    fn decode_error(&self, tokens: &[u32]) -> PyErr {
        let bad = tokens.iter().find(|&&t| self.inner.decode_bytes(&[t]).is_err()).copied().unwrap_or(0);
        PyKeyError::new_err(format!("Invalid token for decoding: {bad}"))
    }
}

#[pymethods]
impl CoreBPE {
    #[new]
    fn new(mergeable_ranks: &Bound<'_, PyDict>, special_tokens: HashMap<String, u32>, pattern: &str) -> PyResult<Self> {
        let mut encoder = HashMap::with_capacity(mergeable_ranks.len());
        for (k, v) in mergeable_ranks.iter() {
            let k = k.downcast::<PyBytes>()?;
            encoder.insert(k.as_bytes().to_vec(), v.extract::<u32>()?);
        }
        let specials: Vec<String> = special_tokens.keys().filter(|s| !s.is_empty()).cloned().collect();
        let mut firsts: Vec<u8> = specials.iter().map(|s| s.as_bytes()[0]).collect();
        firsts.sort_unstable();
        firsts.dedup();
        tiktoken_fast::CoreBPE::new(encoder, special_tokens, pattern)
            .map(|inner| CoreBPE { inner, specials, firsts })
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn encode_ordinary(&self, py: Python<'_>, text: &Bound<'_, PyString>) -> PyResult<Vec<u32>> {
        Ok(self.ordinary(py, &utf8(text)?))
    }

    /// encode() with tiktoken's default arguments (no special token allowed,
    /// all disallowed): the tokens, or None when the text holds a special
    /// token and the caller has to raise tiktoken's error.
    fn encode_strict(&self, py: Python<'_>, text: &Bound<'_, PyString>) -> PyResult<Option<Vec<u32>>> {
        let text = utf8(text)?;
        if self.has_special(&text) {
            return Ok(None);
        }
        Ok(Some(self.ordinary(py, &text)))
    }

    fn encode(&self, py: Python<'_>, text: &str, allowed_special: HashSet<PyBackedStr>) -> Vec<u32> {
        py.allow_threads(|| {
            let allowed: HashSet<&str> = allowed_special.iter().map(|s| &**s).collect();
            self.inner.encode(text, &allowed)
        })
    }

    /// The number of tokens encode() with tiktoken's default arguments would
    /// return, or None when the text holds a special token.
    fn count_strict(&self, py: Python<'_>, text: &Bound<'_, PyString>) -> PyResult<Option<usize>> {
        let text = utf8(text)?;
        if self.has_special(&text) {
            return Ok(None);
        }
        Ok(Some(self.ordinary(py, &text).len()))
    }

    /// The number of tokens of encode(text, allowed_special).
    fn count(&self, py: Python<'_>, text: &Bound<'_, PyString>, allowed_special: HashSet<PyBackedStr>) -> PyResult<usize> {
        let text = utf8(text)?;
        let allowed: HashSet<&str> = allowed_special.iter().map(|s| &**s).collect();
        Ok(if text.len() < RELEASE_FROM {
            self.inner.encode(&text, &allowed).len()
        } else {
            py.allow_threads(|| self.inner.encode(&text, &allowed).len())
        })
    }

    /// The tokens as native-endian u32 bytes, for numpy.frombuffer.
    fn encode_to_tiktoken_buffer<'py>(&self, py: Python<'py>, text: &str, allowed_special: HashSet<PyBackedStr>) -> Bound<'py, PyBytes> {
        let toks = self.encode(py, text, allowed_special);
        let bytes: Vec<u8> = toks.iter().flat_map(|t| t.to_ne_bytes()).collect();
        PyBytes::new_bound(py, &bytes)
    }

    #[pyo3(signature = (texts, num_threads = 8))]
    fn encode_ordinary_batch(&self, py: Python<'_>, texts: Vec<PyBackedStr>, num_threads: usize) -> Vec<Vec<u32>> {
        py.allow_threads(|| self.map_texts(&texts, num_threads, |t| self.inner.encode_ordinary(t)))
    }

    #[pyo3(signature = (texts, allowed_special, num_threads = 8))]
    fn encode_batch(&self, py: Python<'_>, texts: Vec<PyBackedStr>, allowed_special: HashSet<PyBackedStr>, num_threads: usize) -> Vec<Vec<u32>> {
        py.allow_threads(|| {
            let allowed: HashSet<&str> = allowed_special.iter().map(|s| &**s).collect();
            self.map_texts(&texts, num_threads, |t| self.inner.encode(t, &allowed))
        })
    }

    fn decode_bytes<'py>(&self, py: Python<'py>, tokens: Vec<u32>) -> PyResult<Bound<'py, PyBytes>> {
        match py.allow_threads(|| self.inner.decode_bytes(&tokens)) {
            Ok(b) => Ok(PyBytes::new_bound(py, &b)),
            Err(_) => Err(self.decode_error(&tokens)),
        }
    }

    /// Drops the per-caller caches. For benchmarks.
    fn reset_caches(&self) {
        self.inner.reset_caches()
    }
}

#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<CoreBPE>()
}
