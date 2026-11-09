+++
title = "Lru Cache"
+++

[LRU-Cache](https://docs.python.org/3/library/functools.html#functools.lru_cache) (least recently used) is a way to remember **the most recent** results to a function. Because it does not cache all previous results, it can offer more speed up potential than the *cache* alternative.

```python
from functools import lru_cache
@lru_cache(maxsize=32)  # Keep track of previous 32 calls
def lru_factorial(n):
    return n * lru_factorial(n-1) if n else 1
```
