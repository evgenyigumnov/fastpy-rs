import json
import pytest
import fastpy_rs

# Sample data for serialization
SAMPLE_DATA = {
    "name": "John Doe" * 100000 ,
    "age": 30,
    "is_active": True,
    "scores": [95, 87, 92, 88, 91] * 100000 ,
    "address": {
        "street": "123 Main St",
        "city": "Anytown",
        "zip": "12345"
    },
    "tags": ["developer", "python", "rust"],
    "metadata": {
        "created_at": "2023-01-01T00:00:00Z" * 100000,
        "updated_at": "2023-06-24T10:00:00Z"
    }
}

def python_serialize_json(data: dict) -> str:
    """Python implementation using json module."""
    return json.dumps(data)

@pytest.mark.benchmark(group="json_serialize")
def test_json_serialize_rust(benchmark):
    """Benchmark the Rust implementation of JSON serialization."""
    result = benchmark(fastpy_rs.json.serialize_json, SAMPLE_DATA)
    assert isinstance(result, str)


@pytest.mark.benchmark(group="json_serialize")
def test_json_serialize_python(benchmark):
    """Benchmark the Python implementation using json module."""
    result = benchmark(python_serialize_json, SAMPLE_DATA)
    assert isinstance(result, str)


if __name__ == "__main__":
    # This allows running the benchmark directly with Python
    import pytest
    pytest.main(["-x", __file__, "--benchmark-only", "--benchmark-warmup=on"])
