import pytest
from fastpy_rs import ai
import spacy
from collections import Counter

# Load the small English model
nlp = spacy.load("en_core_web_sm")

# Sample text for benchmarking
SAMPLE_TEXT = """
Natural language processing (NLP) is a subfield of linguistics, computer science, 
and artificial intelligence concerned with the interactions between computers and human language, 
in particular how to program computers to process and analyze large amounts of natural language data. 
The result is a computer capable of "understanding" the contents of documents, including the 
contextual nuances of the language within them. The technology can then accurately extract information
and insights contained in the documents as well as categorize and organize the documents themselves.
""" * 100  # Make it larger for better benchmark results


def python_token_frequency(text):
    """Python implementation using spaCy for tokenization."""
    doc = nlp(text.lower())
    tokens = [tok.text for tok in doc if tok.is_alpha]
    return dict(Counter(tokens))


@pytest.mark.benchmark(group="token_frequency")
def test_token_frequency_rust(benchmark):
    """Benchmark the Rust implementation of token_frequency."""
    result = benchmark(ai.token_frequency, SAMPLE_TEXT)
    assert isinstance(result, dict) and len(result) > 0

@pytest.mark.benchmark(group="token_frequency")
def test_token_frequency_python(benchmark):
    """Benchmark the Python implementation using spaCy."""
    result = benchmark(python_token_frequency, SAMPLE_TEXT)
    assert isinstance(result, dict) and len(result) > 0


if __name__ == "__main__":
    # This allows running the benchmark directly with Python
    import pytest
    pytest.main(["-x", __file__, "--benchmark-only"])
