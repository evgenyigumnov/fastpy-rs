import pytest
from fastpy_rs import json


def test_parse_json_basic():
    # Test with basic JSON object
    json_str = '{"name": "John", "age": 30, "is_student": false}'
    result = json.parse_json(json_str)
    assert result == {"name": "John", "age": 30, "is_student": False}
    assert isinstance(result, dict)

def test_parse_json_nested():
    # Test with nested JSON object
    json_str = '''
    {
        "person": {
            "name": "Alice",
            "age": 25,
            "address": {
                "city": "New York",
                "zip": "10001"
            }
        },
        "scores": [85, 92, 78]
    }
    '''
    result = json.parse_json(json_str)
    assert result["person"]["name"] == "Alice"
    assert result["person"]["age"] == 25
    assert result["person"]["address"]["city"] == "New York"
    assert result["scores"] == [85, 92, 78]

def test_parse_json_array():
    # Test with JSON array at top level (should fail as per implementation)
    json_str = '[1, 2, 3, 4]'
    with pytest.raises(ValueError, match="JSON must be an object at the top level"):
        json.parse_json(json_str)

def test_parse_json_empty():
    # Test with empty object
    json_str = '{}'
    result = json.parse_json(json_str)
    assert result == {}

def test_parse_json_with_null():
    # Test with null values
    json_str = '{"key1": null, "key2": {"nested_key": null}}'
    result = json.parse_json(json_str)
    assert result["key1"] is None
    assert result["key2"]["nested_key"] is None

def test_parse_json_number_types():
    # Test with different number types
    json_str = '{"int": 42, "float": 3.14, "large_int": 9007199254740991}'
    result = json.parse_json(json_str)
    assert result["int"] == 42
    assert result["float"] == 3.14
    assert result["large_int"] == 9007199254740991

def test_parse_json_invalid():
    # Test with invalid JSON
    with pytest.raises(ValueError, match="Invalid JSON"):
        json.parse_json('{"invalid": json}')

    # Test with empty string
    with pytest.raises(ValueError, match="Invalid JSON"):
        json.parse_json('')
