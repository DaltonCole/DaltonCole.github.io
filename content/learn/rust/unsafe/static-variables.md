+++
title = "Static Variables"
+++

Rust static variables are called *global variables* in other languages.

```rust
static PI: f32 = 3.14159;
static mut COUNTER: u32 = 0;

fn main() {
    // Accessing non-mutable static variables is safe
    println!("Pi = {}", PI);

    unsafe {
        // Accessing mutable static variables is unsafe (think threads)
        COUNTER += 1;
    }
}
```
