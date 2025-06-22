use pyo3::prelude::*;
use std::collections::HashMap;
use regex::Regex;
use pyo3::exceptions::PyValueError;



#[pyfunction]
fn token_frequency(text: &str) -> PyResult<HashMap<String, u32>> {
    let re = Regex::new(r"\w+").map_err(|e| PyValueError::new_err(e.to_string()))?;
    
    let mut freq = HashMap::new();
    for word in re.find_iter(text).map(|m| m.as_str().to_lowercase()) {
        *freq.entry(word).or_insert(0) += 1;
    }
    
    Ok(freq)
}

#[pymodule]
fn fastpy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(token_frequency, m)?)?;
    Ok(())
}