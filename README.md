# fastpy-rs

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PyPI version](https://badge.fury.io/py/fastpy-rs.svg)](https://badge.fury.io/py/fastpy-rs)

FastPy-RS is a high-performance Python library that provides optimized implementations of common functions using Rust. It's designed to be a collection of frequently used functions where performance matters, offering significant speed improvements over pure Python implementations.

## Features

- **Blazing Fast**: Leverages Rust's performance to provide significant speedups
- **Easy to Use**: Simple Python interface
- **Growing Collection**: New functions are added regularly

### Currently Available Functions

1. **Token Frequency Counter**
   - Counts word frequencies in a text (case-insensitive)
   - Example: `{"hello": 2, "world": 1}` for input "Hello hello world"

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
import fastpy_rs

# Count word frequencies in a text
text = "Hello hello world! This is a test. Test passed!"
frequencies = fastpy_rs.token_frequency(text)
print(frequencies)
# Output: {'hello': 2, 'world': 1, 'this': 1, 'is': 1, 'a': 1, 'test': 2, 'passed': 1}
```

## Performance

Performance comparison between the Rust implementation and a Python implementation using [spaCy](https://spacy.io/), a popular industrial-strength NLP library:

```
Performance Test Results (average time per call):
Rust implementation: 0.003164 seconds
Python implementation (spaCy): 0.014781 seconds
Speedup: 4.67x
```

The test compares the tokenization and frequency counting of a text sample. The Rust implementation shows a significant performance improvement over the Python/spaCy implementation, being approximately 4.7x faster in our tests. Note that spaCy provides additional NLP features beyond simple tokenization, while our Rust implementation is optimized specifically for the token frequency counting task.

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

- [ ] Add more string processing functions
- [ ] Implement numerical computation functions
- [ ] Add file I/O utilities
- [ ] Add parallel processing utilities