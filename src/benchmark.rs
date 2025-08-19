use std::sync::mpsc::{Receiver, Sender};

use pyo3::exceptions::{PyRuntimeError, PyTypeError};
use pyo3::prelude::*;
use pyo3::types::{PyDelta, PyDict, PyList, PyTuple};

/// Calls the passed callable and returns a timedelta object with the time it took to call the function.
/// Expect natural overhead of 100-1000 microseconds but in extreme cases this can be a lot higher.
/// It is capped at ~35 minutes due to the implementation details.
///
/// # Arguments
/// * `callable` - The callable to execute the call on, and pass the arguments to.
/// * `args` - The positioned arguments to pass to the callable
/// * `kwargs` - The keyword arguments to pass to the callable
///
/// # Returns
/// * A timedelta object containing information on how long the call took.
///
/// # Raises
/// * `RuntimeError` - If creating the timedelta object fails. For example if microseconds surpass the 32-bit integer limit
/// * `TypeError` - If the arguments are of the wrong type or count or if a non-callable is passed as the first argument.
///
///
/// # Examples
/// ```python
/// import time
/// import fastpy_rs
///
/// # Make a simple GET request
/// result = fastpy_rs.benchmark.benchmark_fn(lambda x: print(x), 5)
/// print(result.total_seconds())
///
/// # Handle errors
/// try:
///     fastpy_rs.benchmark.benchmark_fn(lambda: time.sleep(60 * 2148))
/// except RuntimeError as e:
///     print(f"Took too long")
/// ```
#[pyfunction]
#[pyo3(signature = (callable,  *args, **kwargs))]
pub fn benchmark_fn(
    py: Python,
    callable: Py<PyAny>,
    args: Py<PyTuple>,
    kwargs: Option<Py<PyDict>>,
) -> PyResult<Py<PyDelta>> {
    // We bind before getting our ference time. Usually this shouldn't have an impact, though.
    let bound_kwargs: Option<&Bound<'_, PyDict>> = kwargs.as_ref().map(|v| v.bind(py));

    // We bind the function before hand to reduce overhead.
    let py_callable = callable.bind(py).as_any();

    if !py_callable.is_callable() {
        return Err(PyTypeError::new_err(
            "Passed callable is, in fact, not a callable.",
        ));
    }

    let start_time = std::time::Instant::now();

    // We use this over `callable.call(py, args, bound_kwargs)?;`
    // as it binds the callable resulting in more overhead for the timing itself.
    py_callable.call(args, bound_kwargs)?;
    let elapsed_time: std::time::Duration = start_time.elapsed();

    let result = convert_duration_to_pydelta(py, elapsed_time)?;

    Ok(result.unbind())
}

fn convert_duration_to_pydelta<'py>(
    py: Python<'py>,
    duration: std::time::Duration,
) -> PyResult<Bound<'py, PyDelta>> {
    // We could convert to days and seconds if we want to allow longer times.
    let total_microsecs = i32::try_from(duration.as_micros()).map_err(|e| {
        PyRuntimeError::new_err(format!(
            "Duration too long: Total microseconds exceeded i32 limit (approx. 2147 seconds or ~35.8 minutes). If you need longer values I suggest refactoring your code or contacting the library author. Original error: {}", e
        ))
    })?;

    let py_delta: Bound<'_, PyDelta> = PyDelta::new(py, 0, 0, total_microsecs, false)?;
    Ok(py_delta)
}
