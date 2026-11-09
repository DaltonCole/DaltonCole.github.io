+++
title = "Save"
+++

To save a PrettyTable to a file, it must be converted to a string first

```python
with open('file.txt', 'w') as f:
    f.write(table.get_string())
```
