+++
title = "Split"
+++

Split a file path into dir and file:

```python
import os
head, tail = os.path.split('/path/to/file.txt')
print(head)  # '/path/to'
print(tail)  # 'file.txt'
```
