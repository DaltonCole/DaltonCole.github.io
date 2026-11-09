+++
title = "Default Implementation"
+++

To have a default implementation for a trait, use a `{}` block.

Default implementations can also call other functions.

Example:

```rust
pub trait Shape {
    fn area(&self) -> f32 {
        0.0
    }
}
```
