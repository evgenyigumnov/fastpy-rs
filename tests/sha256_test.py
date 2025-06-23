import pytest
from fastpy_rs import crypto
import timeit
import hashlib

def test_sha256():
    # Test with empty input
    assert crypto.sha256(b"") == \
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    
    # Test with known input
    assert crypto.sha256(b"hello world") == \
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    
    # Test type of the result
    assert isinstance(crypto.sha256(b"test"), str)

def test_sha256_str():
    # Test with string input
    assert crypto.sha256_str("hello world") == \
        "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    
    # Test with empty string
    assert crypto.sha256_str("") == \
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"

