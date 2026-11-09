+++
title = "Imap"
+++

Use [imap](https://docs.python.org/3/library/multiprocessing.html#multiprocessing.pool.Pool.imap) if you want to load in the arguments as you go (use a generator that will return LOTS of results). Will be *slightly* slower than `map`. Set `chunksize` to a number greater than `1` for lots of speedup.
