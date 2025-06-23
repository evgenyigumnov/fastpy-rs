
use pyo3::prelude::*;

mod ai;
mod datatools;
mod crypto;
mod textutils;

/// FastPy-RS: High-performance Python extensions written in Rust
///
/// This crate provides optimized Python extensions for various tasks including:
/// - AI/ML utilities
/// - Data processing tools
/// - Cryptographic functions
/// - Text processing utilities
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
    
    Ok(())
}
