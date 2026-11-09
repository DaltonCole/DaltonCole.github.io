+++
title = "Map"
+++

Applies a closure to an iterator and returns an iterator. Example:

```rust
let add_one: Vec<_> = vec![1,2,3].iter().map(|x| x + 1).collect();
```
