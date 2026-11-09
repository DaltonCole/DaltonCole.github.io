+++
title = "Splitext"
+++

Get the extension from path

```python
import os
path, ext = os.path.splitext('/path/to/file.tar.gz')
print(path)  # '/path/to/file.tar'
print(ext)   # '.gz'
```
