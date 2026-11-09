+++
title = "Json"
+++

JSON is a great way to store text in both a human and machine readable format.

```python
import json

x = ''
# Read JSON file
with open('file.json', 'r') as f:
    x = json.load(f)

# Save JSON file
json.dump(x, open('file.json', 'w'), indent=4, sort_keys=True)
```
