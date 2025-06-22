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

def test_base64_encode_performance():
    # Generate a larger binary data for performance testing
    data = b""
    for i in range(1000):
        data += f"test string {i}".encode('utf-8')
    
    # Define the Python implementation
    def python_base64_encode(data):
        return py_base64.b64encode(data).decode('ascii')
    
    # Warm-up runs
    datatools.base64_encode(data)
    python_base64_encode(data)
    
    # Number of test runs
    num_runs = 10
    
    # Time the Rust implementation
    rust_time = timeit.timeit(
        lambda: datatools.base64_encode(data),
        number=num_runs
    )
    
    # Time the Python implementation
    python_time = timeit.timeit(
        lambda: python_base64_encode(data),
        number=num_runs
    )
    
    avg_rust = rust_time / num_runs
    avg_python = python_time / num_runs
    
    print(f"\nBase64 Performance Test Results (average time per call):")
    print(f"Rust implementation: {avg_rust:.8f} seconds")
    print(f"Python implementation: {avg_python:.8f} seconds")
    print(f"Speedup: {avg_python / avg_rust:.2f}x")
    
    # The test fails if Rust implementation is significantly slower than Python's
    # We'll allow some overhead for the Python-Rust boundary crossing
    assert avg_rust <= avg_python * 5, \
        f"Rust implementation ({avg_rust:.8f}s) is more than 5x slower than Python's ({avg_python:.8f}s)"
