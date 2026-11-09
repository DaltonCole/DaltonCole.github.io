+++
title = "Expressions"
+++

Rust is an expression-based language. Unlike statements:

- Expressions do not end in a semi-colon
- Expressions evaluate to a resulting value (returns something)

For example:

```rust
fn main() {
    let x = 1;

    // The {} block is an expression. The entire line is a statement.
    let y = {
        let x = 7; // Different x
        x + 1 // Notice how there is no semi-colon here.
    };

    println!("y = {}", y);
}
```
