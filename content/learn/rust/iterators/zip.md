+++
title = "Zip"
+++

Iterates over two iterators simultaneously.

```rust
let x = vec![1, 2, 3, 4];
    let y = vec![9, 8, 7, 6];

    let z: Vec<i32> = x.iter().zip(y).map(|(a, b)| a + b).collect();

    assert_eq!(z, vec![10, 10, 10, 10]);
```
