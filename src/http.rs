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
/// response = fastpy_rs.http.http_get("https://httpbin.org/get")
/// print(response)  # Output: JSON response from the server
///
/// # Handle errors
/// try:
///     fastpy_rs.http.http_get("https://nonexistent.url")
/// except ValueError as e:
///     print(f"Request failed: {e}")
/// ```
#[pyfunction]
pub fn http_get(url: &str) -> PyResult<String> {
    // Create a runtime for the async request
    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| PyValueError::new_err(format!("Failed to create runtime: {}", e)))?;

    // Execute the async block on the runtime
    rt.block_on(async {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| PyValueError::new_err(format!("Failed to create HTTP client: {}", e)))?;

        let response = client
            .get(url)
            .send()
            .await
            .map_err(|e| PyValueError::new_err(format!("Request failed: {}", e)))?;

        // Check if the request was successful
        if !response.status().is_success() {
            return Err(PyValueError::new_err(format!(
                "Request failed with status code: {}",
                response.status()
            )));
        }

        // Get the response body as text
        let body = response
            .text()
            .await
            .map_err(|e| PyValueError::new_err(format!("Failed to read response body: {}", e)))?;

        Ok(body)
    })
}


