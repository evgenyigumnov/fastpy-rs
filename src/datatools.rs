//! Data encoding/decoding utilities for Python-Rust interop.
//!
//! This module provides various data transformation functions that are commonly needed
//! when working with data across Python and Rust boundaries.

use std::io::{Read, Write};
use pyo3::prelude::*;
use base64::{engine::general_purpose, Engine as _};
use flate2::{bufread, write::GzEncoder, Compression};
use urlencoding::{encode, decode};

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
#[pyfunction]
pub fn base64_encode(data: &[u8]) -> PyResult<String> {
    Ok(general_purpose::STANDARD.encode(data))
}

/// Decodes a string into a base64 decoded bytes
///
/// # Arguments
/// * `data` - The string to decode
///
/// # Returns
/// A `Vec<u8>` containing the base64 decoded data
///
/// # Examples
/// ```python
/// from fastpy_rs.datatools import base64_decode
///
/// decoded = base64_decode('aGVsbG8=')
/// assert decoded == b'hello'
/// ```
#[pyfunction]
pub fn base64_decode(data: String) -> PyResult<Vec<u8>> {
    general_purpose::STANDARD.decode(data)
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>("The given string is not valid"))
}


/// Compress given bytes with specified compression level
/// 
/// # Arguments
/// * `data` bytes that to compression
/// * `compression_level` compression level in range of 0-9
/// * `mtime` can be used to set the modification time
/// 
/// # Returns
/// A `Vec<u8>` containing the compressed data
/// 
/// # Examples
/// ```python
/// from fastpy_rs.datatools import gzip_compress
/// 
/// # with default compress level 9
/// gzip_compress(b'Hello World')
/// with specified compress level
/// gzip_compress(b'Hello World', 6)
/// ```
/// 
/// # Errors
/// Raise `pyo3::exceptions::PyValueError` if compression level is invalid
#[pyfunction]
#[pyo3(signature = (data, compress_level = 9))]
pub fn gzip_compress(data: &[u8], compress_level: i32) -> PyResult<Vec<u8>> {
    if compress_level > 9 || compress_level < 0 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Bad compression level : {}", compress_level)));
    }
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(compress_level as u32));
    encoder.write_all(data)?;
    let bytes = encoder.finish()?;
    Ok(bytes)
}


#[pyfunction]
pub fn gzip_decompress(data: &[u8]) -> PyResult<Vec<u8>> {
    let mut decoder = bufread::GzDecoder::new(&data[..]);
    let mut out = Vec::new();
    decoder.read_to_end(&mut out)?;
    Ok(out)
}

#[pyfunction]
pub fn url_encode(data: &str) -> PyResult<String> {
    Ok(encode(data).into_owned())
}

#[pyfunction]
pub fn url_decode(data: &str) -> PyResult<String> {
    decode(data)
        .map(|cow| cow.into_owned())
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid UTF-8 sequence")))
}