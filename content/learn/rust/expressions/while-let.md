+++
title = "While Let"
+++

Similar to `if let`, `while let` allows you to loop until the "if let" is false.

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop {
    println!("{}", top);
}
```
