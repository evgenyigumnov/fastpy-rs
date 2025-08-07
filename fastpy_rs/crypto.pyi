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