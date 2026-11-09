+++
title = "Generics"
+++

Generics are Rust's version of templates. Examples:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn swap<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 43, y: 3.14 };
    let p2 = Point { x: "Hello", y: "Daltie" };
    let p3 = p1.swap(p2);
}
```
