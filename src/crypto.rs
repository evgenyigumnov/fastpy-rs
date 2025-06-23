//! Cryptographic hash functions for Python using Rust's cryptographic primitives.
//!
//! This module provides efficient implementations of common cryptographic hashing functions
//! that can be called from Python.


use pyo3::prelude::*;
use sha2::{Digest, Sha256};


#[pyfunction]
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    hex::encode(result)
}

/// Calculate SHA-256 hash of the input string.
/// 
/// # Arguments
/// * `data` - Input string to be hashed
/// 
/// # Returns
/// * Hex-encoded SHA-256 hash string
/// 
/// # Example
/// ```
/// assert_eq!(sha256_str("hello"), "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
/// ```
#[pyfunction]
pub fn sha256_str(data: &str) -> String {
    sha256(data.as_bytes())
}

