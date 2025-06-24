use std::convert::Infallible;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList};
use serde_json::Value;

/// parse_json(string: str) -> dict[str, Any]
#[pyfunction]
pub fn parse_json(py: Python, json_str: &str) -> PyResult<PyObject> {
    // 1) парсим строку
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| PyValueError::new_err(format!("Invalid JSON: {}", e)))?;

    // вспомогательная рекурсивная функция
    fn value_to_pyobject(val: &Value, py: Python) -> PyResult<PyObject> {
        match val {
            Value::Null        => Ok(py.None()),
            Value::Bool(b)     => {
                Ok(PyBool::new(py, *b).to_owned().into_any().unbind())

            },
            Value::String(s)   => {
                Ok(s.clone().into_pyobject(py)?.to_owned().into_any().unbind())
            },
            Value::Array(arr)  => {
                let list = PyList::empty(py);
                for elem in arr {
                    list.append(value_to_pyobject(elem, py)?)?;
                }
                Ok(list.into_pyobject(py)?.to_owned().into_any().unbind())
            },
            Value::Array(arr)  => {
                let list = PyList::empty(py);
                for elem in arr {
                    list.append(value_to_pyobject(elem, py)?)?;
                }
                Ok(list.into_pyobject(py)?.to_owned().into_any().unbind())
            },
            Value::Object(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, value_to_pyobject(v, py)?)?;
                }
                Ok(dict.into_pyobject(py)?.to_owned().into_any().unbind())
            },
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(i.into_pyobject(py)?.to_owned().into_any().unbind())
                } else if let Some(u) = n.as_u64() {
                    Ok(u.into_pyobject(py)?.to_owned().into_any().unbind())
                } else if let Some(f) = n.as_f64() {
                    Ok(f.into_pyobject(py)?.to_owned().into_any().unbind())
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

        let any: PyObject = dict.into();


        Ok(any)
    } else {
        Err(PyValueError::new_err("JSON must be an object at the top level"))
    }
}
