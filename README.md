# fastpy-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PyPI version](https://badge.fury.io/py/fastpy-rs.svg)](https://badge.fury.io/py/fastpy-rs)

FastPy-RS is a high-performance Python library that provides optimized implementations of common functions using Rust. It's designed to be a collection of frequently used functions where performance matters, offering significant speed improvements over pure Python implementations.

## Features

- **Blazing Fast**: Leverages Rust's performance to provide significant speedups
- **Easy to Use**: Simple Python interface
- **Secure**: Written in Rust, ensuring high security

### Currently Available Functions

1. **Token Frequency Counter**
   - Counts word frequencies in a text (case-insensitive)
   - Example: `ai.token_frequency("Hello hello world! This is a test. Test passed!")` returns `{'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}`

2. **Base64 Encoding**
   - Encodes binary data to base64 string
   - Example: `datatools.base64_encode(b"hello")` returns `b'aGVsbG8='`

3. **SHA-256 Hashing**
   - Computes SHA-256 hash of binary or string data
   - Example: `crypto.sha256(b"hello")` returns `"2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"`
   - Also available as `sha256_str("hello")` for string inputs

4. [x] **Regex Search**
   - Finds all unique matches of a regex pattern in text
   - Example: `textutils.regex_search(r'\b\w+@\w+\.\w+\b', 'Emails: test@example.com, user@test.org')` returns `['test@example.com', 'user@test.org']`
   - Performance optimized with Rust's regex engine

## Installation

```bash
pip install fastpy-rs
```

Or from source:

```bash
pip install maturin
maturin develop
```

## Usage

```python
from fastpy_rs import ai

# Count word frequencies in a text
text = "Hello hello world! This is a test. Test passed!"
frequencies = ai.token_frequency(text)
print(frequencies)
# Output: {'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}
```

## Performance

### Token Frequency Counter
Performance comparison between the Rust implementation and a Python implementation using [spaCy](https://spacy.io/), a popular industrial-strength NLP library:

```
Performance Test Results (average time per call):
Rust implementation: 0.000022 seconds
Python implementation: 0.014737 seconds
Speedup: 681.94x
```

The test compares the tokenization and frequency counting of a text sample. The Rust implementation shows a significant performance improvement over the Python/spaCy implementation, being approximately 682x faster in our tests. Note that spaCy provides additional NLP features beyond simple tokenization, while our Rust implementation is optimized specifically for the token frequency counting task.

### Base64 Encoding
Performance comparison between the Rust implementation and Python's built-in `base64` module:

```
Base64 Performance Test Results (average time per call):
Rust implementation: 0.00000487 seconds
Python implementation: 0.00003055 seconds
Speedup: 6.27x
```

Our Rust implementation now outperforms Python's built-in base64 module, being approximately 6.27x faster.

### SHA-256 Hashing
Performance comparison between the Rust implementation and Python's `hashlib` module:

```
SHA-256 Performance Test Results (average time per call):
Rust implementation: 0.005949 seconds
Python implementation: 0.006331 seconds
Speedup: 1.06x
```

Our Rust implementation is slightly faster than Python's built-in hashlib, showing consistent performance improvements.

### Regex Search
Performance comparison between the Rust implementation and Python's built-in `re` module:

```
Regex Search Performance Test Results (average time per call):
Rust implementation: 0.000586 seconds
Python implementation: 0.002391 seconds
Speedup: 4.08x
```

Our Rust implementation shows a significant performance improvement over Python's built-in `re` module, being approximately 2.88x faster for regex pattern matching tasks.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

1. Install Rust: https://www.rust-lang.org/tools/install
2. Install maturin: `pip install maturin`
3. Clone the repository
4. Build in development mode: `maturin develop`
5. Run tests: `pytest tests/`

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

### 📦 **JSON / Data**

1. [ ] `parse_json(string) -> dict`
2. [ ] `serialize_json(obj, pretty=False) -> str`
3. [ ] `parse_large_json_file(filepath) -> dict`
4. [ ] `extract_json_field(json_str, path: str) -> Any` (JSONPath-like)
5. [ ] `compare_json(json1, json2) -> bool`
6. [ ] `validate_json(schema: dict, data: dict) -> bool`
7. [ ] `minify_json(json_str: str) -> str`
8. [ ] `pretty_print_json(json_str: str) -> str`
9. [ ] `merge_json_objects(json1: dict, json2: dict) -> dict`
10. [ ] `flatten_json(nested_dict: dict) -> dict`

---

### 🌐 **HTTP / Networking**

11. [ ] `http_get(url, headers=None, timeout=10) -> str`
12. [ ] `http_post(url, data, headers=None) -> str`
13. [ ] `http_download(url, dest_path)`
14. [ ] `http_request(method, url, headers, body) -> (code, body)`
15. [ ] `fetch_json(url) -> dict`
16. [ ] `http_head(url) -> headers`
17. [ ] `http_retry_request(...)`
18. [ ] `http_stream_lines(url) -> Iterator[str]`
19. [ ] `http_check_redirect_chain(url) -> List[str]`
20. [ ] `http_measure_latency(url) -> float`

---

### 🔐 **Hashing / Crypto**

21. [x] `sha256(data: bytes | str) -> str`
22. [ ] `md5(data: bytes | str) -> str`
23. [ ] `hmac_sha256(key, message) -> str`
24. [ ] `blake3_hash(data) -> str`
25. [ ] `is_valid_sha256(hexstr: str) -> bool`
26. [ ] `secure_compare(a: str, b: str) -> bool`

---

### 🧮 **Data Processing / Encoding**

27. [x] `base64_encode(data: bytes) -> str`
28. [ ] `base64_decode(data: str) -> bytes`
29. [ ] `gzip_compress(data: bytes) -> bytes`
30. [ ] `gzip_decompress(data: bytes) -> bytes`
31. [ ] `url_encode(str) -> str`
32. [ ] `url_decode(str) -> str`
33. [ ] `csv_parse(csv_string) -> List[Dict]`
34. [ ] `csv_serialize(data: List[Dict]) -> str`
35. [ ] `bloom_filter_create(size: int, hash_funcs: int)`
36. [ ] `bloom_filter_check(item: str) -> bool`

---

### ⏱️ **Performance / Utils**

37. [ ] `benchmark_fn(callable, *args, **kwargs) -> float`
38. [ ] `parallel_map(func, list, threads=4) -> list`
39. [ ] `fast_deduplication(list) -> list`
40. [ ] `sort_large_list(list) -> list`
41. [ ] `fuzzy_string_match(a, b) -> score`
42. [ ] `levenshtein_distance(a, b) -> int`
43. [ ] `tokenize_text(text: str) -> List[str]`
44. [ ] `fast_word_count(text: str) -> Dict[str, int]`
45. [x] `regex_search(pattern, text) -> List[str]`
46. [ ] `regex_replace(pattern, repl, text) -> str`

---

### 🧠 **AI/ML Preprocessing**

47. [ ] `normalize_vector(vec: List[float]) -> List[float]`
48. [ ] `cosine_similarity(vec1, vec2) -> float`
49. [x] `token_frequency(text: str) -> Dict[str, int]` 
50. [ ] `encode_text_fast(text: str) -> List[int]`