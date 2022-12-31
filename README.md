# py-dissimilar

![CI](https://github.com/messense/py-dissimilar/workflows/CI/badge.svg)
[![PyPI](https://img.shields.io/pypi/v/dissimilar.svg)](https://pypi.org/project/dissimilar)

Python bindings to Rust [dissimilar](https://github.com/dtolnay/dissimilar) crate, a diff library with semantic cleanup.

## Installation

```bash
pip install dissimilar
```

## Usage

This module provides one function:

1. `def diff(a: str, b: str) -> List[Chunk]: ...`

```python
from dissimilar import diff, Equal, Delete, Insert

a = '[乀丁abcd一]'
b = '[一abcd丁]'
chunks = diff(a, b)
assert chunks == [
   Equal("["),
   Delete("乀丁"),
   Insert("一"),
   Equal("abcd"),
   Delete("一"),
   Insert("丁"),
   Equal("]"),
]
```

## License

This work is released under the MIT license. A copy of the license is provided in the [LICENSE](./LICENSE) file.
