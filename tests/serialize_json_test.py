import pytest
from fastpy_rs import json as rs_json

def test_serialize_json_basic_types():
    # Test with basic Python types
    assert rs_json.serialize_json(None) == "null"
    assert rs_json.serialize_json(True) == "true"
    assert rs_json.serialize_json(42) == "42"
    assert rs_json.serialize_json(3.14) == "3.14"
    assert rs_json.serialize_json("hello") == '"hello"'

def test_serialize_json_list():
    # Test with lists
    assert rs_json.serialize_json([1, 2, 3]) == "[1,2,3]"
    assert rs_json.serialize_json(["a", "b", "c"]) == '["a","b","c"]'
    assert rs_json.serialize_json([True, None, 42]) == "[true,null,42]"
    assert rs_json.serialize_json([1, [2, 3], [4, [5]]]) == "[1,[2,3],[4,[5]]]"

def test_serialize_json_dict():
    # Test with dictionaries
    assert rs_json.serialize_json({}) == "{}"
    assert rs_json.serialize_json({"a": 1, "b": 2}) in ['{"a":1,"b":2}', '{"b":2,"a":1}']
    assert rs_json.serialize_json({"nested": {"key": "value"}}) in ['{"nested":{"key":"value"}}']


