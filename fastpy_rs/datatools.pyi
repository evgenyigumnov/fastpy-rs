def base64_encode(data: bytes) -> str:
    r"""
    Encodes a byte slice into a base64 encoded string .

    # Arguments

    * `data` - The bytes to encode

    # Returns

    `str` - containing the base64 encoded data

    # Examples

    ```python
    from fastpy_rs.datatools import base64_encode

    encoded = base64_encode(b"hello")
    assert encoded == 'aGVsbG8='
    ```
    """