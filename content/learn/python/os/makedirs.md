+++
title = "Makedirs"
+++

To make directories recursively:

```python
import os
if not os.path.exists('./test1/test2'):
    os.makedirs('./test1/test2')
```

Alternatively:

```python
from pathlib import Path
Path('./test3/test4').mkdir(parents=True, exist_ok=True)
```
