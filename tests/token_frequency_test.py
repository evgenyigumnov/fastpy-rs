import pytest
from  fastpy_rs import ai
import timeit
from collections import Counter
import spacy

# Load the small English model
nlp = spacy.load("en_core_web_sm")


def test_token_frequency_basic():
    text = "hello world hello"
    result = ai.token_frequency(text)
    assert result == {"hello": 2, "world": 1}


def test_token_frequency_empty():
    text = ""
    result = ai.token_frequency(text)
    assert result == {}


def test_token_frequency_special_chars():
    text = "Hello, world! This is a test. Hello again!"
    result = ai.token_frequency(text)
    assert result == {"hello": 2, "world": 1, "this": 1, "is": 1, "a": 1, "test": 1, "again": 1}


def test_token_frequency_case_sensitive():
    text = "Hello hello HELLO"
    result = ai.token_frequency(text)
    assert result == {"hello": 3}


