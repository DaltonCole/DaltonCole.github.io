+++
title = "Cache"
+++

[Cache](https://docs.python.org/3/library/functools.html) is a way to remember **all** previous results to a function

```python
from functools import cache
@cache
def factorial(n):
    return n * factorial(n-1) if n else 1
```
