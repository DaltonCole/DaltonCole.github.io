+++
title = "Tests"
+++

Rust comes with a unit and integration test suit baked into `cargo`. To make a function a test function, add `#[test]` on the line before the function.

Tests in rust have access to the private parts of everything.

The following is an example of a unit test:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(w: u32, h: u32) -> Rectangle {
        Rectangle {width: w, height: h}
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;  // Brings outer module into scope
    
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle::new(5, 6);
        let smaller = Rectangle::new(4, 5);

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larget() {
        let smaller = Rectangle::new(4, 5);
        let larger = Rectangle::new(5, 6);

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != 4"))
        }
    }
}
```
