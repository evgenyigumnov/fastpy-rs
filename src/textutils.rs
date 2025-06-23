use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use regex::Regex;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::HashMap;
use ahash::AHashSet;

static REGEX_CACHE: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// Finds all unique matches of a regex pattern in the given text.
///
/// This function uses a cached regex engine to improve performance
/// by storing compiled regex patterns. If the cache exceeds 20 entries,
/// it will be cleared to free up memory.
///
/// # Arguments
/// * `pattern` - A string slice that holds the regex pattern to search for.
/// * `text` - A string slice that holds the text in which to search for the pattern.
///
/// # Returns
/// * A `PyResult` containing a vector of unique matches as strings. If the pattern is invalid,
///   returns a `PyValueError` with an appropriate error message.
#[pyfunction]
pub fn regex_search(pattern: &str, text: &str) -> PyResult<Vec<String>> {
    let mut cache = REGEX_CACHE.lock().unwrap();

    let re = if let Some(re) = cache.get(pattern) {
        re.clone()
    } else {
        let new_re = Regex::new(pattern)
            .map_err(|e| PyValueError::new_err(format!("Invalid regex pattern: {}", e)))?;
        cache.insert(pattern.to_string(), new_re.clone());
        if cache.len() > 20 {
            cache.clear();
        }
        new_re
    };

    drop(cache);

    let mut matches = AHashSet::new();
    for m in re.find_iter(text) {
        matches.insert(m.as_str().to_string());
    }

    Ok(matches.into_iter().collect())
}
