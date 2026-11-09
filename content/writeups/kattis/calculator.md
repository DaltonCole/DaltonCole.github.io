+++
title = "Calculator"
kattis_url = "https://open.kattis.com/problems/calculator"
difficulty = "3.7"
languages = ["Python"]
writeup = true
+++

## Write-Up

Python is the perfect programming language for this question! Python has a built-in function called `eval()`. This function takes in a string as a parameter and evaluates this expression as a line of python code. The function returns the results of the expression, just like when you use the python interpreter. So, for this problem, you need to read in input from stdin while there is input and print out the results of `eval()`.

## Code

#### Python

```python
from sys import stdin

for line in stdin:
	print("{:0.2f}".format(eval(line)))
```
