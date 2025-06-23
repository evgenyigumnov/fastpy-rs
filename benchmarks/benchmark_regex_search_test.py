import re
import pytest
from fastpy_rs import textutils

# Sample text for benchmarking with multiple email patterns
BASE_TEXT = """
Contact us at support@example.com or sales@company.org for more information.
Our team members include john.doe@email.com, jane_smith@company.net,
and alex.wilson@another-org.co.uk. Don't forget to check spam@example.org.
"""

# Repeat the text to make it longer for better benchmark results
SAMPLE_TEXT = BASE_TEXT * 1000

# Common regex patterns for testing
EMAIL_PATTERN = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"


def python_regex_search(pattern, text):
    """Python implementation using re.findall"""
    return list(set(re.findall(pattern, text, re.IGNORECASE)))

# Benchmark functions for different patterns
@pytest.mark.parametrize("pattern,pattern_name", [
    (EMAIL_PATTERN, "email")
])
@pytest.mark.benchmark(group="regex_search")
class TestRegexSearch:
    def test_regex_search_rust(self, benchmark, pattern, pattern_name):
        """Benchmark the Rust implementation of regex_search"""
        result = benchmark(textutils.regex_search, pattern, SAMPLE_TEXT)
        assert isinstance(result, list)
        
    def test_regex_search_python(self, benchmark, pattern, pattern_name):
        """Benchmark the Python implementation using re.findall"""
        result = benchmark(python_regex_search, pattern, SAMPLE_TEXT)
        assert isinstance(result, list)

# Additional test with a more complex pattern

if __name__ == "__main__":
    # This allows running the benchmark directly with Python
    import pytest
    pytest.main(["-x", __file__, "--benchmark-only"])
