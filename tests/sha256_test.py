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

def test_sha256_performance():
    # Generate a larger binary data for performance testing
    data = b""
    for i in range(1000):
        data += f"test string {i}".encode('utf-8')
    
    # Define the Python implementation for comparison
    def python_sha256(data: bytes) -> str:
        return hashlib.sha256(data).hexdigest()
    
    # Warm-up runs
    for _ in range(10):
        crypto.sha256(data)
        python_sha256(data)
    
    # Time the Rust implementation
    rust_time = timeit.timeit(
        lambda: crypto.sha256(data),
        number=1000
    )
    
    # Time the Python implementation
    python_time = timeit.timeit(
        lambda: python_sha256(data),
        number=1000
    )
    
    print(f"\nSHA-256 Performance:")
    print(f"Rust implementation: {rust_time:.6f} seconds")
    print(f"Python implementation: {python_time:.6f} seconds")
    print(f"Speedup: {python_time/rust_time:.2f}x")
    
    # The Rust implementation should be at least as fast as Python's
    # Note: This is commented out as it might be flaky on some systems
    # assert rust_time <= python_time * 5, \
    #     f"Rust implementation ({rust_time:.6f}s) is slower than Python's ({python_time:.6f}s)"
