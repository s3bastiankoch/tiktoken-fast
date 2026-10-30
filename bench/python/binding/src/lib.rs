use pyo3::prelude::*;
use std::collections::HashMap;

#[pyclass]
struct Encoding(tiktoken_fast::CoreBPE);

#[pymethods]
impl Encoding {
    #[new]
    fn new(mergeable_ranks: HashMap<Vec<u8>, u32>, special_tokens: HashMap<String, u32>, pat_str: &str) -> PyResult<Self> {
        tiktoken_fast::CoreBPE::new(mergeable_ranks, special_tokens, pat_str)
            .map(Encoding)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }
    fn encode_ordinary(&self, text: &str) -> Vec<u32> {
        self.0.encode_ordinary(text)
    }
    fn reset_caches(&self) {
        self.0.reset_caches()
    }
}

#[pymodule]
fn tkpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Encoding>()
}
