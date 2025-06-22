use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use regex::Regex;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::collections::{HashMap, HashSet};


static REGEX_CACHE: Lazy<Mutex<HashMap<String, Regex>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

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

    let mut matches = HashSet::new();
    for cap in re.captures_iter(text) {
        if let Some(m) = cap.get(0) {
            matches.insert(m.as_str().to_string());
        }
    }

    Ok(matches.into_iter().collect())
}

