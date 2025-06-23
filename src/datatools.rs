//! Data encoding/decoding utilities for Python-Rust interop.
//!
//! This module provides various data transformation functions that are commonly needed
//! when working with data across Python and Rust boundaries.

use pyo3::prelude::*;
use base64::{engine::general_purpose, Engine as _};

/// Encodes a byte slice into a base64 encoded string.
///
/// # Arguments
/// * `data` - The byte slice to encode
///
/// # Returns
/// A `String` containing the base64 encoded data
///
/// # Examples
/// ```python
/// from fastpy_rs.datatools import base64_encode
///
/// encoded = base64_encode(b"hello")
/// assert encoded == 'aGVsbG8='
/// ```
///
/// # Panics
/// This function will panic if the input data cannot be encoded as base64, though this is
/// extremely unlikely as base64 can encode any binary data.
#[pyfunction]
pub fn base64_encode(data: &[u8]) -> String {
    general_purpose::STANDARD.encode(data)
}
