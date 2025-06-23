import pytest
from fastpy_rs import datatools
import timeit
import base64 as py_base64

def test_base64_encode_basic():
    # Test with simple string
    result = datatools.base64_encode(b"hello")
    assert result == "aGVsbG8="
    assert isinstance(result, str)

def test_base64_encode_empty():
    # Test with empty bytes
    result = datatools.base64_encode(b"")
    assert result == ""

def test_base64_encode_binary():
    # Test with binary data
    binary_data = bytes([0x00, 0x01, 0x7F, 0xFF])
    result = datatools.base64_encode(binary_data)
    assert result == "AAF//w=="

