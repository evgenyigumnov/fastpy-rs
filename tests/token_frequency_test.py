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


def test_token_frequency_performance():
    # Generate a larger text for meaningful performance testing
    text = """
    Natural language processing (NLP) is a subfield of linguistics, computer science, 
    and artificial intelligence concerned with the interactions between computers and human language, 
    in particular how to program computers to process and analyze large amounts of natural language data. 
    The result is a computer capable of "understanding" the contents of documents, including the 
    contextual nuances of the language within them. The technology can then accurately extract information 
    and insights contained in the documents as well as categorize and organize the documents themselves.
    """ * 2  # Repeat to make the text longer
    
    # Define the Python implementation using Counter
    def python_token_frequency(text):
        doc = nlp(text.lower())
        tokens = [tok.text for tok in doc if tok.is_alpha]
        return dict(Counter(tokens))
    
    # Warm-up runs
    ai.token_frequency(text)
    python_token_frequency(text)
    
    # Number of test runs
    num_runs = 10
    
    # Time the Rust implementation
    rust_time = timeit.timeit(
        lambda: ai.token_frequency(text),
        number=num_runs
    )
    
    # Time the Python implementation
    python_time = timeit.timeit(
        lambda: python_token_frequency(text),
        number=num_runs
    )
    
    avg_rust = rust_time / num_runs
    avg_python = python_time / num_runs
    
    print(f"\nPerformance Test Results (average time per call):")
    print(f"Rust implementation: {avg_rust:.6f} seconds")
    print(f"Python implementation: {avg_python:.6f} seconds")
    print(f"Speedup: {avg_python / avg_rust:.2f}x")
    
    # The test fails if Rust implementation is slower than Python's
    assert avg_rust <= avg_python, \
        f"Rust implementation ({avg_rust:.6f}s) is slower than Python's ({avg_python:.6f}s)"

