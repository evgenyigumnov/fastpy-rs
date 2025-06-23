import pytest
from fastpy_rs import datatools
import base64

# Sample data for benchmarking
SAMPLE_DATA = b"""
Natural language processing (NLP) is a subfield of linguistics, computer science, 
and artificial intelligence concerned with the interactions between computers and human language, 
in particular how to program computers to process and analyze large amounts of natural language data. 
The result is a computer capable of "understanding" the contents of documents, including the 
contextual nuances of the language within them. The technology can then accurately extract information
and insights contained in the documents as well as categorize and organize the documents themselves.
""" * 100  # Make it larger for better benchmark results

def python_base64_encode(data: bytes) -> str:
    """Python implementation using base64 module."""
    return base64.b64encode(data).decode('ascii')

@pytest.mark.benchmark(group="base64_encode")
def test_base64_encode_rust(benchmark):
    """Benchmark the Rust implementation of base64 encoding."""
    result = benchmark(datatools.base64_encode, SAMPLE_DATA)
    assert isinstance(result, str)

@pytest.mark.benchmark(group="base64_encode")
def test_base64_encode_python(benchmark):
    """Benchmark the Python implementation using base64 module."""
    result = benchmark(python_base64_encode, SAMPLE_DATA)
    assert isinstance(result, str)

if __name__ == "__main__":
    # This allows running the benchmark directly with Python
    import pytest
    pytest.main(["-x", __file__, "--benchmark-only", "--benchmark-warmup=on"])
