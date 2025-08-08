import pytest
from fastpy_rs import datatools
import timeit
import base64 as py_base64
import gzip as py_builtin_gzip
from urllib.parse import quote, unquote

def test_base64_encode_basic():
    # Test with simple string
    result = datatools.base64_encode(b"hello")
    assert result == "aGVsbG8="
    assert isinstance(result, str)

def test_base64_decode_basic():
    result = datatools.base64_decode('aGVsbG8=')
    assert result == b'hello'
    assert isinstance(result, bytes)

def test_base64_encode_empty():
    # Test with empty bytes
    result = datatools.base64_encode(b"")
    assert result == ""

def test_base64_decode_empty():
    result = datatools.base64_decode("")
    assert result == b''

def test_base64_encode_binary():
    # Test with binary data
    binary_data = bytes([0x00, 0x01, 0x7F, 0xFF])
    result = datatools.base64_encode(binary_data)
    assert result == "AAF//w=="
    
def test_base64_decode_invalid_data():
    invalid_data = '<!!!!!!----'
    with pytest.raises(ValueError):
        datatools.base64_decode(invalid_data)
        
def test_gzip_compress_with_negative_compression_level():
    with pytest.raises(ValueError):
        datatools.gzip_compress(b"", -23)
        
def test_gzip_compress_with_large_compression_level():
    with pytest.raises(ValueError):
        datatools.gzip_compress(b"", 23)
        
def test_gzip_compress_with_emptry_bytes():
    # see https://github.com/python/cpython/blob/44ff6b545149ea59837fc74122d435572f21e489/Lib/gzip.py#L633-L634
    py_built = py_builtin_gzip.compress(b'', mtime=0)
    data_tools = datatools.gzip_compress(b'')
    assert py_built == data_tools
    assert len(py_built) == len(data_tools)
    
def test_gzip_compress_with_differnet_compres_level():
    test_bytes = "Hello World".encode()
    for cl in range(0, 10):
        py_builtin_gzip.compress(test_bytes,cl,  mtime=0) == datatools.gzip_compress(test_bytes, cl)
        
        
def test_gzip_decompress():
    compressed_bytes = py_builtin_gzip.compress("Hello World".encode(), 6, mtime=0)
    assert datatools.gzip_decompress(compressed_bytes) == b"Hello World"
    
    
def test_url_encode():
    assert datatools.url_encode("This string will be URL encoded.") == quote(
        "This string will be URL encoded."
    )
    
    assert datatools.url_encode("") == quote("")

def test_url_decode():
    assert datatools.url_decode("%F0%9F%91%BE%20Exterminate%21") == unquote(
        "%F0%9F%91%BE%20Exterminate%21"
    )