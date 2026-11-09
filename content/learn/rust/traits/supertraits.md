+++
title = "Supertraits"
+++

Supertraits allow a trait to require the implementation of another trait. Example:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point{ x: i32, y: i32 }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

// Supertrait - Has ':' with trait requirements
trait Print: std::fmt::Debug {
    fn print(&self) {
        println!("*** {:?} *** ", self);
    }
}

impl Print for Point {}

fn main() {
    let p1 = Point{x: 1, y: 2};
    p1.print();
}
```
