use pyo3::prelude::*;
use regex::Regex;
use std::collections::HashSet;

#[pyfunction]
pub fn regex_search(pattern: &str, text: &str) -> PyResult<Vec<String>> {
    let re = match Regex::new(pattern) {
        Ok(re) => re,
        Err(e) => return Err(pyo3::exceptions::PyValueError::new_err(format!("Invalid regex pattern: {}", e))),
    };
    
    let mut matches = HashSet::new();
    for capture in re.captures_iter(text) {
        if let Some(m) = capture.get(0) {
            matches.insert(m.as_str().to_string());
        }
    }
    
    Ok(matches.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_regex_search() {
        let text = "Emails: test@example.com, another.email@test.org, not_an_email";
        let pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b";
        
        let result = regex_search(pattern, text).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result.contains(&"test@example.com".to_string()));
        assert!(result.contains(&"another.email@test.org".to_string()));
    }
}
