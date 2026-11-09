+++
title = "Static Lifetimes"
+++

Static lifetimes live for the entire duration of the program. They have the `'static` lifetime annotation.

```rust
fn main() {
    let s;
    {
        let x: &'static str = "LONG LIVE THE STATIC!";
        s = x;
    }
    println!("{}", s); // Static variable is still alive despite leaving scope
}
```
