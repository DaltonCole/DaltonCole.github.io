+++
title = "Isfile"
+++

Check if file is a file or directory:

```python
import os
if os.path.isfile('./path/to/file.txt'):
    print("Is File")
if os.path.isdir("./path/to/dir/"):
    print("Is Directory")
if not os.path.exists('./test1/test2'):
    os.makedirs('./test1/test2')
```
