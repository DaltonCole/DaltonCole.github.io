+++
title = "Pool"
+++

```python
from multiprocessing import Pool

num_processes = 4
args = [(1, 2), (3, 4), (5, 6)]

def async_function(a, b):
    return a + b

results = []
def callback(result):  # Callback should always only accept one argument
    # Do something with the result if you want
    results.append(result)

with Pool(num_processes) as pool:
    asyncResult = pool.starmap_async(async_function, args, callback=callback)
    # Won't do anything until we get the result
    same_results = asyncResult.get()  # get() also returns the results
```
