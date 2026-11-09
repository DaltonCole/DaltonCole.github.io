+++
title = "Filter"
+++

Uses a closure to filter results from an iterator. Only results that return true are added to the resulting iterator.

```rust
let even: Vec<u32> = vec![1,2,3,4].into_iter().filter(|x| x % 2 == 0).collect();
```
