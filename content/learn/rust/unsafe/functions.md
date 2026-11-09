+++
title = "Functions"
+++

```rust
unsafe fn unsafe_func() {} // Call unsafe inside of here, user beware! Read the docs!

fn abstraction() {} // Unsafe is inside this function, but we are smarter than 
// the compiler (maybe) so we know that the contents of the function are fine

// All functions called from a different programming language are unsafe
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        unsafe_func();
    }

    abstraction();

    unsafe {
        println!("Abs(-12) = {}", abs(-12));
    }
}
```
