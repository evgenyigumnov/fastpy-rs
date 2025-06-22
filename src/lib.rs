use pyo3::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use once_cell::sync::Lazy;

static WORD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\w+").expect("Невалидный паттерн")
});



#[pyfunction]
fn token_frequency(text: &str) -> PyResult<HashMap<String, u32>> {
    let mut freq = HashMap::new();
    for m in WORD_RE.find_iter(text) {
        let w = m.as_str().to_ascii_lowercase();
        *freq.entry(w).or_insert(0) += 1;
    }
    Ok(freq)
}

#[pymodule]
fn fastpy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(token_frequency, m)?)?;
    Ok(())
}