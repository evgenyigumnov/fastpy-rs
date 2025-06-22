import pytest
import fastpy_rs


def test_token_frequency_basic():
    text = "hello world hello"
    result = fastpy_rs.token_frequency(text)
    assert result == {"hello": 2, "world": 1}


def test_token_frequency_empty():
    text = ""
    result = fastpy_rs.token_frequency(text)
    assert result == {}


def test_token_frequency_special_chars():
    text = "Hello, world! This is a test. Hello again!"
    result = fastpy_rs.token_frequency(text)
    assert result == {"hello": 2, "world": 1, "this": 1, "is": 1, "a": 1, "test": 1, "again": 1}


def test_token_frequency_case_sensitive():
    text = "Hello hello HELLO"
    result = fastpy_rs.token_frequency(text)
    assert result == {"hello": 3}

