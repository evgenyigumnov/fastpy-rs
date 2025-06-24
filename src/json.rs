use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyAnyMethods, PyString};
use pyo3::types::{PyDict, PyList};
use pyo3::IntoPyObjectExt;
use serde_json::{to_string, to_string_pretty, Number, Value};

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
/// * `pretty` - If true, formats the output with indentation for better readability
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
pub fn serialize_json<'py>(
    py: Python<'py>,
    obj: Bound<'py, PyAny>,
    pretty: Option<bool>,
) -> PyResult<String>{
    let obj: PyObject = obj.unbind();
    fn pyobject_to_value(val: &'_ PyObject, py: Python) -> PyResult<Value> {
        // None
        if val.is_none(py) {
            return Ok(Value::Null);
        }
        // Bool
        if let Ok(b) = val.extract::<bool>(py) {
            return Ok(Value::Bool(b));
        }
        // Int
        if let Ok(i) = val.extract::<i64>(py) {
            return Ok(Value::Number(Number::from(i)));
        }
        // Float
        if let Ok(f) = val.extract::<f64>(py) {
            let num = Number::from_f64(f)
                .ok_or_else(|| PyValueError::new_err("Float out of range"))?;
            return Ok(Value::Number(num));
        }
        // String
        if let Ok(s) = val.extract::<String>(py) {
            return Ok(Value::String(s));
        }
        // List

        if let Ok(list) = val.downcast_bound::<PyList>(py) {
            let mut arr = Vec::with_capacity(list.len());
            for item in list {
                let i: PyObject = item.into();
                arr.push(pyobject_to_value(&i, py)?);
            }
            return Ok(Value::Array(arr));
        }
        // Dict
        if let Ok(dict) = val.downcast_bound::<PyDict>(py) {
            let mut map = serde_json::Map::with_capacity(dict.len());
            for (k, v) in dict {
                let key: String = k.extract()?;
                let v: PyObject = v.into();
                map.insert(key, pyobject_to_value(&v, py)?);
            }
            return Ok(Value::Object(map));
        }
        Err(PyValueError::new_err(format!(
            "Type `{}` is not JSON serializable",
            val.to_string()
        )))
    }

    let value = pyobject_to_value(&obj, py)?;
    let result = if pretty.unwrap_or(false) {
        to_string_pretty(&value)
    } else {
        to_string(&value)
    };
    result.map_err(|e| PyValueError::new_err(format!("Serialization error: {}", e)))

}