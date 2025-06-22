use pyo3::prelude::*;
mod ai;


#[pymodule]
fn fastpy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    register_child_module(m)?;
    Ok(())
}

fn register_child_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let child_module = PyModule::new(parent_module.py(), "ai")?;
    child_module.add_function(wrap_pyfunction!(ai::token_frequency, &child_module)?)?;
    parent_module.add_submodule(&child_module)
}
