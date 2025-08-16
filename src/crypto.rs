//! Cryptographic hash functions for Python using Rust's cryptographic primitives.
//!
//! This module provides efficient implementations of common cryptographic hashing functions
//! that can be called from Python.

use hmac::{Hmac, Mac};
use pyo3::{
    prelude::*,
    types::{PyBytes, PyString},
};
use sha2::{Digest, Sha256};

/// Calculate SHA-256 hash of the input bytes.
///
/// # Arguments
/// * `data` - Input bytes to be hashed
///
/// # Returns
/// * Hex-encoded SHA-256 hash string
///
/// # Example
/// ```
/// use fastpy_rs::crypto::sha256;
///
/// let result = sha256(&[104, 101, 108, 108, 111]); // "hello" in bytes
/// assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
/// ```
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


/// Calculate the MD5 hash of the input string or bytes.
///
/// # Arguments
/// * `data` - Input data, either a string or bytes
///
/// # Returns
/// * Hex-encoded MD5 hash string
///
/// # Example
/// ```
/// assert_eq!(md5("hello"), "5d41402abc4b2a76b9719d911017c592");
/// ```
#[pyfunction]
#[pyo3(name = "md5")] // rename as md5, see https://pyo3.rs/v0.25.1/function.html#function-options
pub fn md5_(_py: Python<'_>, data: Bound<'_, PyAny>) -> PyResult<String> {
    if let Ok(s) = data.downcast::<PyString>() {
        let sbytes = s.to_string();
        return Ok(format!("{:x}", md5::compute(sbytes)));
    } else if let Ok(b) = data.downcast::<PyBytes>() {
        return Ok(format!("{:x}", md5::compute(b.as_bytes())));
    } else {
        return Err(pyo3::exceptions::PyTypeError::new_err(
            "Expected bytes or str",
        ));
    }
}

type HmacSha256 = Hmac<Sha256>;
#[inline]
fn crate_hmac(key: &str) -> PyResult<HmacSha256> {
    HmacSha256::new_from_slice(key.as_bytes())
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("Invalid key length"))
}


/// Calculate HMAC-SHA256 of the input message using the provided key.
///
/// # Arguments
/// * `key` - Secret key as a string
/// * `message` - Message to authenticate
///
/// # Returns
/// * Hex-encoded HMAC-SHA256 string
///
/// # Example
/// ```
/// let mac = hmac_sha256("key", "message");
/// ```
#[pyfunction]
pub fn hmac_sha256(key: &str, message: &str) -> PyResult<String> {
    let mut mac = crate_hmac(key)?;
    mac.update(message.as_bytes());
    let result = mac.finalize();
    Ok(hex::encode(result.into_bytes()))
}


/// Calculate the BLAKE3 hash of the input bytes.
///
/// # Arguments
/// * `data` - Input bytes to be hashed
///
/// # Returns
/// * Hex-encoded BLAKE3 hash string
///
/// # Example
/// ```
/// let hash = blake3_hash(b"hello");
/// ```
#[pyfunction]
pub fn blake3_hash(data: &[u8]) -> PyResult<String> {
    Ok(blake3::hash(data).to_string())
}


/// Check if the input string is a valid SHA-256 hex string.
///
/// # Arguments
/// * `hexstr` - Hex string to validate
///
/// # Returns
/// * `true` if valid SHA-256 hex string, otherwise `false`
///
/// # Example
/// ```
/// assert!(is_valid_sha256("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
/// ```
#[pyfunction]
pub fn is_valid_sha256(hexstr: &str) -> PyResult<bool> {
    if hexstr.len() != 64 {
        return Ok(false);
    }

    Ok(hexstr.chars().all(|ch| ch.is_ascii_hexdigit()))
}

/// Perform a constant-time comparison of two strings to prevent timing attacks.
///
/// # Arguments
/// * `left` - First string to compare
/// * `right` - Second string to compare
///
/// # Returns
/// * `true` if both strings are equal, otherwise `false`
///
/// # Example
/// ```
/// assert!(secure_compare("abc", "abc"));
/// assert!(!secure_compare("abc", "def"));
/// ```
#[pyfunction]
pub fn secure_compare(left: &str, right: &str) -> PyResult<bool> {
    // check bytes length
    let left = left.as_bytes();
    let right = right.as_bytes();
    if left.len() != right.len() {
        return Ok(false);
    }
    let mut result: u8 = 0;
    for (left, right) in left.iter().zip(right.iter()) {
        result |= left ^ right
    }
    Ok(result == 0)
}
