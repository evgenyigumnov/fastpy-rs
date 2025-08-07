from typing import Dict

def token_frequency(text: str) -> Dict[str, int]:
    r"""
    Counts the frequency of each word in the input text.

    This function splits the input text into words (sequences of alphanumeric characters)
    and returns a dictionary where keys are words (converted to lowercase) and values
    are their respective counts in the text.

    # Arguments

    * `text` - The input text to analyze

    # Returns

    `Dict[str, int]` - A dictionary mapping words to their counts

    # Examples

    ```python
    from fastpy_rs import ai

    text = "Hello hello world! This is a test. Test passed!"
    result = ai.token_frequency(text)
    ```
    """