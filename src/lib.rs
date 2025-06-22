use pyo3::prelude::*;

#[pyfunction]
fn double(x: usize) -> PyResult<usize> {
    Ok(x * 2)
}



#[pymodule]
fn fastpy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)?;
    Ok(())
}