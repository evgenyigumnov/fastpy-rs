import pytest
from fastpy_rs import crypto
import hashlib

# Sample data for benchmarking
SAMPLE_DATA = b"""
Natural language processing (NLP) is a subfield of linguistics, computer science, 
and artificial intelligence concerned with the interactions between computers and human language, 
in particular how to program computers to process and analyze large amounts of natural language data. 
The result is a computer capable of "understanding" the contents of documents, including the 
contextual nuances of the language within them. The technology can then accurately extract information
and insights contained in the documents as well as categorize and organize the documents themselves.
""" * 100  # Make it larger for better benchmark results

def python_sha256(data: bytes) -> str:
    """Python implementation using hashlib."""
    return hashlib.sha256(data).hexdigest()

def test_sha256_rust(benchmark):
    """Benchmark the Rust implementation of sha256."""
    result = benchmark(crypto.sha256, SAMPLE_DATA)
    assert isinstance(result, str) and len(result) == 64  # SHA-256 produces 64-character hex string

def test_sha256_python(benchmark):
    """Benchmark the Python implementation using hashlib."""
    result = benchmark(python_sha256, SAMPLE_DATA)
    assert isinstance(result, str) and len(result) == 64

if __name__ == "__main__":
    # This allows running the benchmark directly with Python
    import pytest
    pytest.main(["-x", __file__, "--benchmark-only", "--benchmark-warmup=on"])
