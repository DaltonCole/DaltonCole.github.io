+++
title = "Closures"
+++

Closures are similar to functions except they can be stored in a variable and can capture the surrounding variables.

Example:

```rust
fn main() {
    // Normal Closure
    let add_one = |x| {
        x + 1
    };

    // Closures are of a single concrete type
    let y = add_one(1); // The type x is deduced to be an i32
    //let y = add_one(1.0); // Does not compile because x should be an i32, not f32

    // Type information optional
    let verbose_add_one = |x: i32| -> i32 {
        x + 1
    };

    // {} optional if single expression
    let one_linner = |x| x + 1;
    let z = one_linner(2);
}
```
