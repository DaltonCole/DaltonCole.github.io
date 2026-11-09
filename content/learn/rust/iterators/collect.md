+++
title = "Collect"
+++

Converts an iterator into a collection. The type of collection must be known. Example:

```rust
let add_one: Vec<_> = vec![1,2,3].iter().map(|x| x + 1).collect();
let add_one_more = add_one.iter().map(|x| x + 1).collect::<Vec<i32>>(); // turbofish type specification
```
