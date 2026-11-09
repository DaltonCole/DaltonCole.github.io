+++
title = "Methods"
+++

Struct methods are implemented in `impl` blocks. Multiple methods can be in a single `impl` block and you can have multiple `impl` blocks per struct.

The general syntax of methods are very similar to python methods, where instance methods require `&self` (just using `self` is allowed but rare). To call "class" level methods, the namespace operator `::` is required.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Self's type is automatically the struct's type
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits_inside(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // Example where you wouldn't use self
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 25,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 100,
        height: 45,
    };

    println!("Area of rect1: {}", rect1.area());
    println!("Fits inside: {}", rect1.fits_inside(&rect2));

    // Create a square rectangle. Use the namespace syntax here
    let square = Rectangle::square(7);
    println!("Area of square: {}", square.area());
}
```
