+++
title = "Traits"
+++

Traits in Rust are similar to *interfaces* in other languages. When a type implements a trait, that type will behave as the trait describes.

```rust
pub trait Shape {
    fn area(&self) -> f32;  // Traits declarations have ';' as opposed to {} blocks
}

pub struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * 3.14159
    }
}

pub struct Rectable {
    width: f32,
    height: f32,
}

impl Shape for Rectable {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}

fn main() {
    let cir = Circle { radius: 12.25 };
    println!("{}", cir.area());
}
```
