# fastpy-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PyPI version](https://badge.fury.io/py/fastpy-rs.svg)](https://badge.fury.io/py/fastpy-rs)

![cblt](https://github.com/evgenyigumnov/fastpy-rs/raw/HEAD/rust-python.png)


FastPy-RS is a high-performance Python library that provides optimized implementations of common functions using Rust. It's designed to be a collection of frequently used functions where performance matters, offering significant speed improvements over pure Python implementations.

## Features

- **Blazing Fast**: Leverages Rust's performance to provide significant speedups
- **Easy to Use**: Simple Python interface
- **Secure**: Written in Rust, ensuring high security

Documentation: [https://evgenyigumnov.github.io/fastpy-rs](https://evgenyigumnov.github.io/fastpy-rs)

### Usage

```python
import fastpy_rs as fr
# Using crypto functions
hash_result = fr.crypto.sha256_str("hello")

# Using data tools
encoded = fr.datatools.base64_encode(b"hello")

# Count word frequencies in a text
text = "Hello hello world! This is a test. Test passed!"
frequencies = fr.ai.token_frequency(text)
print(frequencies)
# Output: {'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}

# JSON parsing
json_data = '{"name": "John", "age": 30, "city": "New York"}'
parsed_json = fr.json.parse_json(json_data)
print(parsed_json)
# Output: {'name': 'John', 'age': 30, 'city': 'New York'}

# JSON serialization
data_to_serialize = {'name': 'John', 'age': 30, 'city': 'New York'}
serialized_json = fr.json.serialize_json(data_to_serialize)
print(serialized_json)
# Output: '{"name": "John", "age": 30, "city": "New York"}'

# HTTP requests
url = "https://api.example.com/data"
response = fr.http.get(url)
print(response)
# Output: b'{"data": "example"}'
```

## Installation

```bash
pip install fastpy-rs
```

Or from source:

```bash
pip install maturin
maturin develop
```

## Performance
Run:
```
pytest  --benchmark-only  --benchmark-group-by=group
```
Results:
```
---------------------------------------------------------------------------------- benchmark 'base64_encode': 2 tests ---------------------------------------------------------------------------------
Name (time in us)                  Min                 Max                Mean             StdDev              Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_base64_encode_rust        17.3000 (1.0)      162.0000 (1.0)       18.1661 (1.0)       4.0086 (1.0)       17.8000 (1.0)      0.1001 (1.0)      139;4366       55.0476 (1.0)       16026           1
test_base64_encode_python     116.7000 (6.75)     326.2000 (2.01)     120.4332 (6.63)     11.7741 (2.94)     118.9000 (6.68)     2.4000 (23.99)     178;320        8.3034 (0.15)       7887           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

-------------------------------------------------------------------------------------------------------------------- benchmark 'regex_search': 2 tests ---------------------------------------------------------------------------------------------------------------------
Name (time in us)                                                                                 Min                   Max                  Mean             StdDev                Median                 IQR            Outliers         OPS            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_regex_search_rust[\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b-email]         633.8000 (1.0)        931.2000 (1.0)        667.8635 (1.0)      35.7218 (1.0)        658.8000 (1.0)       18.1750 (1.0)         79;87  1,497.3119 (1.0)        1143           1
test_regex_search_python[\\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\\.[A-Z|a-z]{2,}\\b-email]     4,097.2000 (6.46)     4,566.4000 (4.90)     4,215.2770 (6.31)     90.3190 (2.53)     4,203.6500 (6.38)     116.5500 (6.41)         63;6    237.2323 (0.16)        204           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------- benchmark 'sha256': 2 tests --------------------------------------------------------------------------------
Name (time in us)          Min                 Max               Mean            StdDev             Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_sha256_rust       22.0000 (1.0)      218.9000 (1.19)     23.0597 (1.0)      4.0259 (1.0)      22.6000 (1.0)      0.4000 (1.0)      448;2624       43.3657 (1.0)       37038           1
test_sha256_python     22.6000 (1.03)     184.4999 (1.0)      23.5171 (1.02)     4.5349 (1.13)     23.1000 (1.02)     0.4999 (1.25)     227;1221       42.5223 (0.98)      19456           1
--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

--------------------------------------------------------------------------------------------- benchmark 'token_frequency': 2 tests --------------------------------------------------------------------------------------------
Name (time in us)                        Min                     Max                    Mean                StdDev                  Median                    IQR            Outliers         OPS            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_token_frequency_rust           759.9000 (1.0)        1,128.4000 (1.0)          790.5642 (1.0)         41.6179 (1.0)          780.1000 (1.0)          18.7500 (1.0)         65;76  1,264.9194 (1.0)         816           1
test_token_frequency_python     727,971.8000 (957.98)   754,932.2000 (669.03)   739,440.5600 (935.33)   9,955.3656 (239.21)   739,839.7000 (948.39)   10,844.9250 (578.40)        2;0      1.3524 (0.00)          5           1
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

--------------------------------------------------------------------------------- benchmark 'json_parse': 2 tests ----------------------------------------------------------------------------------
Name (time in us)               Min                 Max                Mean             StdDev              Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_json_parse_python      67.0000 (1.0)      351.2000 (1.08)      71.7280 (1.0)       8.6944 (1.0)       70.5000 (1.0)      2.2000 (1.0)       292;588       13.9416 (1.0)       10965           1
test_json_parse_rust       148.9000 (2.22)     324.5000 (1.0)      155.2763 (2.16)     12.0176 (1.38)     153.2000 (2.17)     2.9000 (1.32)      138;236        6.4401 (0.46)       3728           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

---------------------------------------------------------------------------- benchmark 'json_serialize': 2 tests -----------------------------------------------------------------------------
Name (time in ms)                  Min                Max               Mean            StdDev             Median               IQR            Outliers      OPS            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_json_serialize_python     18.5541 (1.0)      19.9359 (1.0)      19.3042 (1.0)      0.3420 (1.88)     19.3594 (1.0)      0.4167 (1.43)         17;0  51.8021 (1.0)          53           1
test_json_serialize_rust       30.9494 (1.67)     31.6470 (1.59)     31.2320 (1.62)     0.1815 (1.0)      31.2358 (1.61)     0.2919 (1.0)           8;0  32.0185 (0.62)         33           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------------------ benchmark: 2 tests -----------------------------------------------------------------------------------------
Name (time in us)             Min                   Max                Mean              StdDev              Median                 IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_http_get_rust       321.9620 (1.0)      1,928.3610 (2.77)     629.2036 (1.0)      335.8742 (31.09)    538.7160 (1.0)      424.6580 (27.51)        12;4        1.5893 (1.0)         100         100
test_http_get_python     637.9020 (1.98)       696.6980 (1.0)      663.1543 (1.05)      10.8032 (1.0)      664.8140 (1.23)      15.4370 (1.0)          37;1        1.5079 (0.95)        100         100
-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```
### Performance Insights

- **Token Frequency** shows the most dramatic improvement (935x), making it ideal for text analysis tasks
- **Base64** and **Regex** operations benefit significantly from Rust's optimizations (6-6.6x faster)
- **SHA-256** performance is on par with Python, as both use optimized native implementations
- Lower standard deviation in Rust implementations indicates more consistent performance

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

1. [x] `parse_json(string) -> dict`
2. [x] `serialize_json(obj) -> str`


---

### 🌐 **HTTP / Networking**

11. [x] `get(url) -> str`
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