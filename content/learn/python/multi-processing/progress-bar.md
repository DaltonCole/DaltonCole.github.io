+++
title = "Progress Bar"
+++

```python
from multiprocessing import Pool
from tqdm import tqdm

num_processes = 4
args = [(1, 2), (3, 4), (5, 6)]  # A generator also works. See `chunksize` (https://docs.python.org/3/library/multiprocessing.html#multiprocessing.pool.Pool.starmap)
results = []

def func(a, b):
    return a + b

with Pool(num_processes) as pool:
    # Use `map` or `imap` instead of `starmap` if `func` only has 1 argument
    for item in tqdm(pool.starmap(func, args), total=len(args)):
        results.append(item)
```

Another option is [p_tqdm](https://github.com/swansonk14/p_tqdm)
