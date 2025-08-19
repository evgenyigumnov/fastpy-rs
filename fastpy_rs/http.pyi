def get(url: str) -> str:
    r"""
    Makes an HTTP GET request to the specified URL and returns the response body as a string.
    
    # Arguments
    
    * `url` - The URL to make the GET request to
    
    # Returns
    
    str - A string containing the response body
    
    # Raises
    
    * `ValueError` - If the request fails or the response status is not successful
    
    # Examples
    
    ```python
    import fastpy_rs
    
    # Make a simple GET request
    response = fastpy_rs.http.get("https://httpbin.org/get")
    print(response)  # Output: JSON response from the server
    
    # Handle errors
    try:
        fastpy_rs.http.get("https://nonexistent.url")
    except ValueError as e:
        print(f"Request failed: {e}")
    ```
    """