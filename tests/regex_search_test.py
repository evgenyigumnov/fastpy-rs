import re
import timeit
from fastpy_rs import textutils

def test_regex_search_basic():
    """Test basic regex search functionality"""
    text = "Emails: test@example.com, another.email@test.org, not_an_email"
    pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
    
    result = textutils.regex_search(pattern, text)
    assert len(result) == 2
    assert "test@example.com" in result
    assert "another.email@test.org" in result
    assert "not_an_email" not in result

def test_regex_search_no_matches():
    """Test regex search when no matches are found"""
    text = "This text contains no email addresses"
    pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
    
    result = textutils.regex_search(pattern, text)
    assert len(result) == 0

def test_regex_search_performance():
    """Performance test comparing Rust implementation with Python's re.findall"""
    # Generate a larger text with multiple email patterns
    base_text = """
    Contact us at support@example.com or sales@company.org for more information.
    Our team members include john.doe@email.com, jane_smith@company.net,
    and alex.wilson@another-org.co.uk. Don't forget to check spam@example.org.
    """
    
    # Repeat the text to make it longer
    text = base_text * 1000
    pattern = r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b"
    
    # Define the Python implementation using re.findall
    def python_regex_search(pattern, text):
        return list(set(re.findall(pattern, text)))
    
    # Warm-up runs
    textutils.regex_search(pattern, text)
    python_regex_search(pattern, text)
    
    # Number of test runs
    num_runs = 100
    
    # Time the Rust implementation
    rust_time = timeit.timeit(
        lambda: textutils.regex_search(pattern, text),
        number=num_runs
    )
    
    # Time the Python implementation
    python_time = timeit.timeit(
        lambda: python_regex_search(pattern, text),
        number=num_runs
    )
    
    avg_rust = rust_time / num_runs
    avg_python = python_time / num_runs
    
    print(f"\nRegex Search Performance Test Results (average time per call):")
    print(f"Rust implementation: {avg_rust:.6f} seconds")
    print(f"Python implementation: {avg_python:.6f} seconds")
    print(f"Speedup: {avg_python / avg_rust:.2f}x")
    
    # The test fails if Rust implementation is slower than Python's
    assert avg_rust <= avg_python * 1.5, \
        f"Rust implementation ({avg_rust:.6f}s) is more than 1.5x slower than Python's ({avg_python:.6f}s)"
