+++
title = "Johnny Applesack"
kattis_url = "https://open.kattis.com/problems/applesack"
difficulty = "3.4"
languages = ["Python"]
writeup = false
+++

## Code

#### Python

```python
from math import ceil

apples, sack = input().split()
apples = int(apples)
sack = int(sack)

distance = 1

while apples > sack:
	apples -= ceil(apples / sack)
	distance += 1

distance += apples

print(distance)
```
