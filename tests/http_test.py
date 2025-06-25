import pytest
import fastpy_rs
import json

def test_http_get_success():
    """Test successful HTTP GET request"""
    response = fastpy_rs.http.http_get("https://httpbin.org/get")
    data = json.loads(response)
    assert "url" in data
    assert data["url"] == "https://httpbin.org/get"

def test_http_get_with_params():
    """Test HTTP GET request with query parameters"""
    params = {"key1": "value1", "key2": "value2"}
    response = fastpy_rs.http.http_get(f"https://httpbin.org/get?{'&'.join(f'{k}={v}' for k, v in params.items())}")
    data = json.loads(response)
    assert data["args"] == params

def test_http_get_headers():
    """Test that headers are properly sent"""
    response = fastpy_rs.http.http_get("https://httpbin.org/headers")
    data = json.loads(response)
    assert "headers" in data

def test_http_get_error_nonexistent_domain():
    """Test error handling for non-existent domain"""
    with pytest.raises(ValueError, match="Request failed"):
        fastpy_rs.http.http_get("https://this-domain-does-not-exist.example.com")

def test_http_get_error_invalid_url():
    """Test error handling for invalid URL"""
    with pytest.raises(ValueError, match="Request failed"):
        fastpy_rs.http.http_get("not-a-valid-url")

def test_http_get_error_404():
    """Test error handling for 404 response"""
    with pytest.raises(ValueError, match="Request failed with status code: 404"):
        fastpy_rs.http.http_get("https://httpbin.org/status/404")
