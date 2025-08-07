from typing import Dict, Any

def parse_json(json_str: str) -> Dict:
    r"""
    Parses a JSON string into a Python dictionary .
    
    # Arguments
    
    * `json_str` - A string containing valid JSON data
    
    # Returns
    
    `Dict` - A Python dictionary representing the parsed JSON data
    
    # Raises
    
    * `ValueError` - If the input string is not valid JSON or if the JSON is not an object at the top level
    
    # Examples
    ```python
    import fastpy_rs
    
    # Parse a simple JSON object
    data = fastpy_rs.json.parse_json('{"name": "John", "age": 30, "active": true}')
    print(data['name'])  # Output: John
    print(data['age'])   # Output: 30
    
    # Parse JSON with nested structures
    nested = fastpy_rs.json.parse_json('{"users": [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]}')
    print(nested['users'][0]['name'])  # Output: Alice
    ```
    """
    
def serialize_json(obj: Any) -> str:
    r"""
    Serializes a Python object to a JSON string.
    
    # Arguments
    
    * `obj` - A Python object to serialize (dict, list, str, int, float, bool, None)
    
    # Returns
    
    `str` - A JSON string representation of the input object
    
    # Raises
    
    * `ValueError` - If the object contains types that cannot be serialized to JSON
    
    # Examples
    ```python
    import fastpy_rs
    
    # Serialize a simple dictionary
    data = {"name": "John", "age": 30, "active": True}
    json_str = fastpy_rs.json.serialize_json(data)
    print(json_str)  # Output: {"name":"John","age":30,"active":true}
    ```
    """