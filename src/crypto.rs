use pyo3::prelude::*;
use sha2::{Digest, Sha256};

/// Calculate SHA-256 hash of the input data.
/// 
/// # Arguments
/// * `data` - Input data as either bytes or string
/// 
/// # Returns
/// * Hex-encoded SHA-256 hash string
#[pyfunction]
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

#[pyfunction]
pub fn sha256_str(data: &str) -> String {
    sha256(data.as_bytes())
}

