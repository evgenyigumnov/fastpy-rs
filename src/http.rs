use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::time::Duration;

/// Makes an HTTP GET request to the specified URL and returns the response body as a string.
///
/// # Arguments
/// * `url` - The URL to make the GET request to
///
/// # Returns
/// * A string containing the response body
///
/// # Raises
/// * `ValueError` - If the request fails or the response status is not successful
///
/// # Examples
/// ```python
/// import fastpy_rs
///
/// # Make a simple GET request
/// response = fastpy_rs.http.get("https://httpbin.org/get")
/// print(response)  # Output: JSON response from the server
///
/// # Handle errors
/// try:
///     fastpy_rs.http.get("https://nonexistent.url")
/// except ValueError as e:
///     print(f"Request failed: {e}")
/// ```
#[pyfunction]
pub fn get(py: Python, url: String) -> PyResult<String> {
    let body = py.allow_threads(|| {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Can't build client: {}", e))?;

        let resp = client
            .get(&url)
            .send()
            .map_err(|e| format!("Request failed: {}", e))?;

        if !resp.status().is_success() {
            return Err(format!("Status code: {}", resp.status()));
        }

        resp.text().map_err(|e| format!("Ошибка чтения тела: {}", e))
    })
    .map_err(|e| PyValueError::new_err(e))?;

    Ok(body)
}
