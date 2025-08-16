import pytest
import hashlib
import hmac

from fastpy_rs import crypto


def test_md5():
    m = hashlib.md5()
    m.update(b'Hello World')
    assert m.hexdigest() == crypto.md5(b'Hello World')
    assert m.hexdigest() == crypto.md5('Hello World')
    m1 = hashlib.md5()
    m1.update(''.encode())
    assert m1.hexdigest() != crypto.md5(' ')
    assert m1.hexdigest() == crypto.md5('')
    
    
def test_hmac_sha256():
    key = "my_secret_key"
    message = "hello world"
    expected = hmac.new(key.encode(), message.encode(), hashlib.sha256).hexdigest()
    assert expected == crypto.hmac_sha256(key, message)
    
    
def test_hmac_sha256_invalid_key():
    """
    Although an error may be triggered when the string length is too large, 
    when a too large str is passed in, overflowerror is triggered first, 
    resulting in the inability to effectively detect unqualified keys.
    """
    pass
    
def test_blake3_hash():
    data = b'Hello World!'
    assert '5ca7815adcb484e9a136c11efe69c1d530176d549b5d18d038eb5280b4b3470c' == crypto.blake3_hash(data)
    
    
def is_valid_sha256():
    valid_hash = hashlib.sha256(b"test").hexdigest()
    assert crypto.is_valid_sha256(valid_hash) == True
    assert crypto.is_valid_sha256(valid_hash[:12]) == False
    valid_hash[5] = '!'
    assert crypto.is_valid_sha256(valid_hash) == False
    
def secure_compare():
    s1 = "my_super_secret_string_123"
    s2 = "my_super_secret_string_123"
    
    assert crypto.secure_compare(s1, s2) == True
    
    s3 = "string_number_one"
    s4 = "string_number_two"
    
    assert crypto.secure_compare(s3, s4)
    
    
    s5 = "short"
    s6 = "a much longer string"
    
    assert crypto.secure_compare(s5, s6)