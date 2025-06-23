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

