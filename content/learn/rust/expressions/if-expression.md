+++
title = "If Expression"
+++

Rust is a standard `if`, `else if`, `else` language, however, only boolean values are allowed for the conditional. Integers will cause a `mismatched types` error.

```rust
fn main() {
    let number = 12;

    // Standard if, else if, else expression
    if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Math hard, I gave up.");
    }

    // Using "if" in a let statement (because it's an expression, not a statement)
    let number = if number > 0 { number } else { 0 }; // Return types must match
    println!("number is now: {}", number);
}
```

If there are many `else if` expressions, Rust recommends using `match` instead.
