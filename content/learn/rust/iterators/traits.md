+++
title = "Traits"
+++

All iterators implement a trait named `Iterator`, which is defined in the standard library. The definition follows:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>; // Go to next item
}
```
