+++
title = "Multiple Files"
+++

Multiple files example:

```rust
// src/garden.rs
pub mod food;

// src/garden/food.rs
pub fn harvest() {}

// src/main.rs
mod garden;

pub use crate::garden::food;

fn main() {
    food::harvest();
}
```
