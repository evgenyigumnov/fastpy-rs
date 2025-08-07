from typing import List

def regex_search(pattern: str, text: str) -> List[str]:
    r"""
    Finds all unique matches of a regex pattern in the given text .
    
    This function uses a cached regex engine to improve performance
    by storing compiled regex patterns. If the cache exceeds 20 entries,
    it will be cleared to free up memory.
    
    # Arguments
    
    * `pattern` - A string that holds the regex pattern to search for.
    * `text` - A string that holds the text in which to search for the pattern.
    
    # Returns
    
    * `List[str]` - containing a list of unique matches as strings. 
    
    # Raises
    
    * `ValueError` - If the regex pattern is invalid
    """