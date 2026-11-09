+++
title = "Sorting"
+++

##### In-Place

```python
a.sort()
```

##### New List

```python
new_a = sorted(a)
```

##### Key Functions

Works with both in-place and sorted

```python
a = [('a', 1), ('c', 3), ('b', 2)]
a.sort(key=lambda x: x[1])
```
