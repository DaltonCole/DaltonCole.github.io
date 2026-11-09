+++
title = "Conditional Method Implementation"
+++

You can use traits to conditionally implement methods to a generic struct. Example:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}  // Notice Self vs self
    }
}

impl<T: Display> Pair<T> {
    fn print(&self) {
        println!("{}, {}", self.x, self.y);
    }
}

fn main() {
    let p = Pair::new(5, 4);
    p.print();
}
```

###### Self

In the above example, you might notice the capital 'S' in `Self`. Capital `Self` can be used to refer to the type that is being implemented as opposed to `self` which refers to the calling object.
