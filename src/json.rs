use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::IntoPyObjectExt;
use serde_json::{Value};
use serde_json::ser::{CompactFormatter, Serializer};
use pyo3::types::{PyAnyMethods, PyDict, PyList, PyString};
use serde::ser::{Error, SerializeMap, SerializeSeq};
use serde::ser::{Serialize, Serializer as SerTrait};

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
pub fn serialize_json(_py: Python<'_>, obj: Bound<'_, PyAny>) -> PyResult<String> {
    // Буфер, который умеет `std::io::Write`.
    let mut buf = Vec::<u8>::with_capacity(256);

    {
        // Компактный форматтер; можно поменять на PrettyFormatter по желанию.
        let mut ser = Serializer::with_formatter(&mut buf, CompactFormatter {});
        // Всё тяжёлое — без GIL.

        // py.allow_threads(|| PyAnySerializer { inner: obj }.serialize(&mut ser)).map_err(|e| PyValueError::new_err(format!("UTF-8 error: {e}")))?
        PyAnySerializer { inner: obj }.serialize(&mut ser).map_err(|e| PyValueError::new_err(format!("UTF-8 error: {e}")))?
    }

    // Безопасно, потому что serde_json всегда пишет валидный UTF-8.
    Ok(String::from_utf8(buf)
        .map_err(|e| PyValueError::new_err(format!("UTF-8 error: {e}")))?)
}

/// Обёртка, которая делает любой PyAny сериализуемым.
struct PyAnySerializer<'py> {
    inner: Bound<'py, PyAny>,
}

impl<'py> Serialize for PyAnySerializer<'py> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: SerTrait,
    {
        let obj = &self.inner;

        // None -------------------------------------------------------
        if obj.is_none() {
            return serializer.serialize_unit();
        }
        // Bool -------------------------------------------------------
        if let Ok(b) = obj.extract::<bool>() {
            return serializer.serialize_bool(b);
        }
        // Int --------------------------------------------------------
        if let Ok(i) = obj.extract::<i64>() {
            return serializer.serialize_i64(i);
        }
        // Float ------------------------------------------------------
        if let Ok(f) = obj.extract::<f64>() {
            if !f.is_finite() {
                return Err(S::Error::custom("Float out of range"));
            }
            return serializer.serialize_f64(f);
        }
        // Str --------------------------------------------------------
        if let Ok(s) = obj.extract::<&str>() {
            return serializer.serialize_str(s);
        }
        // List -------------------------------------------------------
        if let Ok(list) = obj.downcast::<PyList>() {
            let mut seq = serializer.serialize_seq(Some(list.len()))?;
            for item in list.iter() {
                seq.serialize_element(&PyAnySerializer { inner: item })?;
            }
            return seq.end();
        }
        // Dict -------------------------------------------------------
        if let Ok(d) = obj.downcast::<PyDict>() {
            let mut map = serializer.serialize_map(Some(d.len()))?;
            for (k, v) in d.iter() {
                let key: &str = k
                    .extract()
                    .map_err(|_| S::Error::custom("Dict keys must be str"))?;
                map.serialize_entry(key, &PyAnySerializer { inner: v })?;
            }
            return map.end();
        }

        // Всё остальное — ошибка.
        Err(S::Error::custom(format!(
            "Type `{}` is not JSON-serializable",
            obj.get_type().name().unwrap_or(PyString::new(obj.py(), "<unknown>"))
        )))
    }
}