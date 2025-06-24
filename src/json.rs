use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use pyo3::IntoPyObjectExt;

/// Parses a JSON string into a Python dictionary.
///
/// # Arguments
/// * `json_str` - A string containing valid JSON data
///
/// # Returns
/// * A Python dictionary representing the parsed JSON data
///
/// # Raises
/// * `ValueError` - If the input string is not valid JSON or if the JSON is not an object at the top level
///
/// # Examples
/// ```python
/// import fastpy_rs
///
/// # Parse a simple JSON object
/// data = fastpy_rs.json.parse_json('{"name": "John", "age": 30, "active": true}')
/// print(data['name'])  # Output: John
/// print(data['age'])   # Output: 30
///
/// # Parse JSON with nested structures
/// nested = fastpy.parse_json('{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}')
/// print(nested['users'][0]['name'])  # Output: Alice
/// ```
#[pyfunction]
pub fn parse_json(py: Python, json_str: &str) -> PyResult<PyObject> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| PyValueError::new_err(format!("Invalid JSON: {}", e)))?;

    fn value_to_pyobject(val: &Value, py: Python) -> PyResult<PyObject> {
        match val {
            Value::Null        => Ok(py.None()),
            Value::Bool(b)     => {
                Ok(b.into_py_any(py)?)
            },
            Value::String(s)   => {
                Ok(s.into_py_any(py)?)
            },
            Value::Array(arr)  => {
                let list = PyList::empty(py);
                for elem in arr {
                    list.append(value_to_pyobject(elem, py)?)?;
                }
                Ok(list.into_py_any(py)?)
            },
            Value::Object(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, value_to_pyobject(v, py)?)?;
                }
                Ok(dict.into_py_any(py)?)
            },
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.into_py_any(py)?)
                } else if let Some(u) = n.as_u64() {
                    Ok(u.into_py_any(py)?)
                } else if let Some(f) = n.as_f64() {
                    Ok(f.into_py_any(py)?)
                } else {
                    Err(PyValueError::new_err("Number out of range"))
                }
            }
        }
    }

    if let Value::Object(map) = value {
        let dict = PyDict::new(py);
        for (key, val) in map {
            dict.set_item(key, value_to_pyobject(&val, py)?)?;
        }

        let any: PyObject = dict.into_py_any(py)?;


        Ok(any)
    } else {
        Err(PyValueError::new_err("JSON must be an object at the top level"))
    }
}
