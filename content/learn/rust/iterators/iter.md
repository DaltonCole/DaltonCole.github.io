+++
title = "Iter"
+++

There are three common types of `iter` functions:

- `iter`: Iterator over immutable references
- `into_iter`: Takes ownership of collection and returns owned values
- `iter_mut`: Iterator over mutable references

```rust
fn main() {
    let v = vec![1, 2, 3, 4];
    // iter
    let sum: i32 = v.iter().sum(); // 10
    // into_iter
    let even: Vec<i32> = v.into_iter().filter(|x| x % 2 == 0).collect(); // [2, 4]
    // iter_mut
    let mut v = &mut vec![1, 2, 3, 4];
    for x in v.iter_mut() {
        *x += 1;
    } // [2, 3, 4, 5]
}
```
