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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        // Test with empty input
        assert_eq!(
            sha256(b""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        
        // Test with known input
        assert_eq!(
            sha256(b"hello world"),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }

    #[test]
    fn test_sha256_str() {
        assert_eq!(
            sha256_str("hello world"),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
    }
}
