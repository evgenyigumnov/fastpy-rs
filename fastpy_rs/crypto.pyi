def sha256(data: bytes) -> str:
    r"""
    Calculate SHA-256 hash of the input bytes .
    
    # Arguments
    
    * `data` - Input bytes to be hashed
    
    # Returns
    
    * Hex-encoded SHA-256 hash string
    
    # Example
    
    ```python
    from fastpy_rs import crypto
    
    result = crypto.sha256(b'hello world')
    ```
    """
    
def sha256_str(data: str) -> str:
    r"""
    Calculate SHA-256 hash of the input string .
    
    # Arguments
    
    * `data` - Input string to be hashed
    
    # Returns
    
    * Hex-encoded SHA-256 hash string
    
    # Example
    
    ```python
    from fastpy_rs import crypto
    
    result = crypto.sha256_str('hello world')
    ```
    """
    
    
def md5(data: str | bytes) -> str:
    """
    Calculate the MD5 hash of the input string or bytes.
    
    # Arguments
    * `data` - Input data, either a string or bytes
    
    # Returns
    * Hex-encoded MD5 hash string
    
    # Example
    ```python
    from fastpy_rs import crypto
    result = crypto.md5('hello')
    ```
    """

def hmac_sha256(key: str, message: str) -> str: 
    """
    Calculate HMAC-SHA256 of the input message using the provided key.
    
    # Arguments
    * `key` - Secret key as a string
    * `message` - Message to authenticate
    
    # Returns
    * Hex-encoded HMAC-SHA256 string
    
    # Example
    ```python
    from fastpy_rs import crypto
    mac = crypto.hmac_sha256('key', 'message')
    ```
    """

def blake3_hash(data: bytes) -> str:
    """
    Calculate the BLAKE3 hash of the input bytes.
    
    # Arguments
    * `data` - Input bytes to be hashed
    
    # Returns
    * Hex-encoded BLAKE3 hash string
    
    # Example
    ```python
    from fastpy_rs import crypto
    hash = crypto.blake3_hash(b'hello')
    ```
    """
def is_valid_sha256(hexstr: str) -> bool:
    """
    Check if the input string is a valid SHA-256 hex string.
    
    # Arguments
    * `hexstr` - Hex string to validate
    
    # Returns
    * `True` if valid SHA-256 hex string, otherwise `False`
    
    # Example
    ```python
    from fastpy_rs import crypto
    assert crypto.is_valid_sha256('2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824')
    ```
    """

def secure_compare(left: str, right: str) -> bool:
    """
    Perform a constant-time comparison of two strings to prevent timing attacks.
    
    # Arguments
    * `left` - First string to compare
    * `right` - Second string to compare
    
    # Returns
    * `True` if both strings are equal, otherwise `False`
    
    # Example
    ```python
    from fastpy_rs import crypto
    assert crypto.secure_compare('abc', 'abc')
    assert not crypto.secure_compare('abc', 'def')
    ```
    """