+++
title = "Returning Closures"
+++

Closures can be returned from functions via a pointer.

```rust
fn returns_closure() -> Box<dyn Fn(u32) -> u32> {
    Box::new(|x| x * x)
}
```
