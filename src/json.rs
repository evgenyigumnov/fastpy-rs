use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods};
use pyo3::types::{PyDict, PyList};
use pyo3::IntoPyObjectExt;
use serde_json::{Number, Value};

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
/// nested = fastpy_rs.json.parse_json('{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}')
/// print(nested['users'][0]['name'])  # Output: Alice
/// ```
#[pyfunction]
pub fn parse_json(py: Python, json_str: &str) -> PyResult<PyObject> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| PyValueError::new_err(format!("Invalid JSON: {}", e)))?;

    fn value_to_pyobject(val: &Value, py: Python) -> PyResult<PyObject> {
        match val {
            Value::Null => Ok(py.None()),
            Value::Bool(b) => Ok(b.into_py_any(py)?),
            Value::String(s) => Ok(s.into_py_any(py)?),
            Value::Array(arr) => {
                let list = PyList::empty(py);
                for elem in arr {
                    list.append(value_to_pyobject(elem, py)?)?;
                }
                Ok(list.into_py_any(py)?)
            }
            Value::Object(map) => {
                let dict = PyDict::new(py);
                for (k, v) in map {
                    dict.set_item(k, value_to_pyobject(v, py)?)?;
                }
                Ok(dict.into_py_any(py)?)
            }
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
        Err(PyValueError::new_err(
            "JSON must be an object at the top level",
        ))
    }
}

/// Serializes a Python object to a JSON string.
///
/// # Arguments
/// * `obj` - A Python object to serialize (dict, list, str, int, float, bool, None)
///
/// # Returns
/// * A JSON string representation of the input object
///
/// # Raises
/// * `ValueError` - If the object contains types that cannot be serialized to JSON
///
/// # Examples
/// ```python
/// import fastpy_rs
///
/// # Serialize a simple dictionary
/// data = {"name": "John", "age": 30, "active": True}
/// json_str = fastpy_rs.json.serialize_json(data)
/// print(json_str)  # Output: {"name":"John","age":30,"active":true}
///
/// # Pretty-print the JSON
/// pretty_json = fastpy_rs.json.serialize_json(data, pretty=True)
/// print(pretty_json)
/// # Output:
/// # {
/// #   "name": "John",
/// #   "age": 30,
/// #   "active": true
/// # }
/// ```
/// Serializes a Python object (dict, list, str, int, float, bool, None)
/// to a JSON string.
#[pyfunction]
pub fn serialize_json(py: Python<'_>, obj: Bound<'_, PyAny>) -> PyResult<String> {
    fn to_value(obj: Bound<'_, PyAny>) -> PyResult<Value> {
        // None
        if obj.is_none() {
            return Ok(Value::Null);
        }
        // Bool
        if let Ok(b) = obj.extract::<bool>() {
            return Ok(Value::Bool(b));
        }
        // Int
        if let Ok(i) = obj.extract::<i64>() {
            return Ok(Number::from(i).into());
        }
        // Float
        if let Ok(f) = obj.extract::<f64>() {
            return Number::from_f64(f)
                .ok_or_else(|| PyValueError::new_err("Float out of range"))
                .map(Value::Number);
        }
        // Str
        if let Ok(s) = obj.extract::<&str>() {           // &str вместо String
            return Ok(Value::String(s.to_owned()));
        }
        // List
        if let Ok(list) = obj.downcast::<PyList>() {
            let mut vec = Vec::with_capacity(list.len());
            for item in list.iter() {
                vec.push(to_value(item)?);
            }
            return Ok(vec.into());
        }
        // Dict
        if let Ok(dict) = obj.downcast::<PyDict>() {
            let mut map = serde_json::Map::with_capacity(dict.len());
            for (k, v) in dict.iter() {
                map.insert(k.extract::<&str>()?.to_owned(), to_value(v)?);
            }
            return Ok(map.into());
        }
        Err(PyValueError::new_err(format!(
            "Type `{}` is not JSON-serializable",
            obj.get_type().name()?
        )))
    }

    let value = to_value(obj)?;
    py.allow_threads(|| serde_json::to_string(&value))
        .map_err(|e| PyValueError::new_err(format!("Serialization error: {e}")))
}
