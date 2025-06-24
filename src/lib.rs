
use pyo3::prelude::*;

mod ai;
mod datatools;
mod crypto;
mod textutils;
mod json;

/// FastPy-RS: High-performance Python extensions written in Rust
///
/// This crate provides optimized Python extensions for various tasks including:
/// - AI/ML utilities
/// - Data processing tools
/// - Cryptographic functions
/// - Text processing utilities
/// - JSON parsing
///
/// # Examples
/// ```python
/// import fastpy_rs as fr
///
/// # Using crypto functions
/// hash_result = fr.crypto.sha256_str("hello")
///
/// # Using data tools
/// encoded = fr.datatools.base64_encode(b"hello")
///
/// # Count word frequencies in a text
/// text = "Hello hello world! This is a test. Test passed!"
/// frequencies = fr.ai.token_frequency(text)
/// print(frequencies)
/// # Output: {'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}
///
/// # JSON parsing
/// json_data = '{"name": "John", "age": 30, "active": true}'
/// parsed_json = fr.json.parse_json(json_data)
/// print(parsed_json)
/// # Output: {'name': 'John', 'age': 30, 'active': True}
/// ```


#[pymodule]
fn fastpy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}

/// Registers all child modules with the parent Python module
/// 
/// # Arguments
/// * `parent_module` - The parent Python module to register child modules with
/// 
/// # Returns
/// * `PyResult<()>` - Ok(()) if all modules were registered successfully, or an error if any registration fails
fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    // Register ai module
    let ai_module = PyModule::new(parent_module.py(), "ai")?;
    ai_module.add_function(wrap_pyfunction!(ai::token_frequency, &ai_module)?)?;
    parent_module.add_submodule(&ai_module)?;
    
    // Register datatools module
    let datatools_module = PyModule::new(parent_module.py(), "datatools")?;
    datatools_module.add_function(wrap_pyfunction!(datatools::base64_encode, &datatools_module)?)?;
    parent_module.add_submodule(&datatools_module)?;
    
    // Register crypto module
    let crypto_module = PyModule::new(parent_module.py(), "crypto")?;
    crypto_module.add_function(wrap_pyfunction!(crypto::sha256, &crypto_module)?)?;
    crypto_module.add_function(wrap_pyfunction!(crypto::sha256_str, &crypto_module)?)?;
    parent_module.add_submodule(&crypto_module)?;
    
    // Register textutils module
    let textutils_module = PyModule::new(parent_module.py(), "textutils")?;
    textutils_module.add_function(wrap_pyfunction!(textutils::regex_search, &textutils_module)?)?;
    parent_module.add_submodule(&textutils_module)?;
    
    // Register json module
    let json_module = PyModule::new(parent_module.py(), "json")?;
    json_module.add_function(wrap_pyfunction!(json::parse_json, &json_module)?)?;
    parent_module.add_submodule(&json_module)?;
    
    Ok(())
}
