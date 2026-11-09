+++
title = "How Many Digits?"
kattis_url = "https://open.kattis.com/problems/howmanydigits"
difficulty = "3.6"
languages = ["Python"]
writeup = false
+++

## Code

#### Python

```python
import sys
import math

for i in sys.stdin:
	n = int(i)

	if n == 0 or n == 1:
		print(1)
		continue

	# Kamenetsky formula
	answer = math.ceil((.5) * (math.log(2 * math.pi) - 2*n + (math.log(n) * (1 + (2 * n)))) / math.log(10))

	print(int(answer))
```
