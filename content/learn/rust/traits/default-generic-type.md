+++
title = "Default Generic Type"
+++

A default generic type can be given for traits.

There are two main purposes for this:

- Extend a type without breaking existing code
- Customize specific cases

```rust
use std::ops::Add;

struct Feet(f32);
struct Inches(f32);

/* // Add trait definition
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

// Use default
impl Add for Feet {
    type Output = Feet;

    fn add(self, other: Feet) -> Feet {
        Feet(self.0 + other.0)
    }
}

impl Add<Inches> for Feet {
    type Output = Feet;

    fn add(self, other: Inches) -> Feet {
        Feet(self.0 + (other.0 / 12.0))
    }
}
```
