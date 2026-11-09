+++
title = "Functions"
+++

A function in Rust starts with `fn`. For example:

```rust
fn main() {
    // Body
}
```

`main()` is the most important function and is where the Rust program starts.

Rust does not care where a function is defined, it just needs to be defined. For example:

```rust
fn main() {
    other_function();
}

fn other_function() {
    println!("This function is declared after main. It could have been declared before main.");
}
```
