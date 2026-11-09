+++
title = "Paramaters"
+++

Type annotations for parameters are required in Rust.

```rust
fn main() {
    other_function(12, 'a');
}

fn other_function(x: i32, c: char) {
    println!("The value of x is: {}", x);
    println!("The value of c is: {}", c);
}
```
