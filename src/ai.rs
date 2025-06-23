use pyo3::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use once_cell::sync::Lazy;


static WORD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\w+").expect("Невалидный паттерн")
});



/// Counts the frequency of each word in the input text.
/// 
/// This function splits the input text into words (sequences of alphanumeric characters)
/// and returns a dictionary where keys are words (converted to lowercase) and values
/// are their respective counts in the text.
///
/// # Arguments
/// * `text` - The input text to analyze
/// 
/// # Returns
/// * `PyResult<HashMap<String, u32>>` - A dictionary mapping words to their counts
/// 
/// # Examples
/// ```python
/// from fastpy_rs import ai
/// 
/// text = "Hello hello world! This is a test. Test passed!"
/// result = ai.token_frequency(text)
/// # Returns: {'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}
/// ```
/// 
/// # Performance
/// This function is implemented in Rust for high performance, making it significantly
/// faster than equivalent Python implementations, especially for large texts.
#[pyfunction]
pub fn token_frequency(text: &str) -> PyResult<HashMap<String, u32>> {
    let mut freq = HashMap::new();
    for m in WORD_RE.find_iter(text) {
        let w = m.as_str().to_ascii_lowercase();
        *freq.entry(w).or_insert(0) += 1;
    }
    Ok(freq)
}
