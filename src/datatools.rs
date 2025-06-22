use pyo3::prelude::*;
use base64::{engine::general_purpose, Engine as _};

/// Encode bytes to base64 string
/// 
/// Args:
///     data: Bytes to encode
/// 
/// Returns:
///     str: Base64 encoded string
/// 
/// Example:
///     >>> from fastpy_rs.datatools import base64_encode
///     >>> base64_encode(b"hello")
///     'aGVsbG8='
#[pyfunction]
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}
