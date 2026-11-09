+++
title = "Pretty Table"
+++

```python
from prettytable import PrettyTable

table = PrettyTable()

table.title = 'Title'
table.field_names = ['Col1', 'Col2']
table.add_row(['Apple Pi', 3.14159])
table.add_row(['Banana', 42])

# Print table to stdout
print(table)

# Save text table to file
with open('output.txt', 'w') as f:
    f.write(table.get_string())
```
