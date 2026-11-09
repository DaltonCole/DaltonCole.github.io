+++
title = "Option"
+++

Null references have caused lots of problems in other languages. To solve this, Rust created the `Option` enum. In the `Option` enum, there are two possibilities: `Some` (not null) and `None` (null). The `Option` enum will not interact with none `Option` data, thus you have to be aware of when using types that might be null, unlike C++ pointers 😁.

This enum is so popular in Rust that it is automatically loaded, `Some` and `None` can be used without namespacing them with `Option`.

The `Option` enum works well with the `match` expression, verifying that you cover both the null and not null cases.

Basic definition:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example use:

```rust
// With namespaceing
let some_number = Option::Some(2);
// Without namespacing
let another_number = Some(3);
let some_string = Some("Daltie");

// "Null"
let absent_number: Option<i32> = None; // Need to specify the type that Null would be otherwise

// Options and none options do not interact with each other natively
let x: i8 = 3;
let y: Option<i8> = Some(5);

//let sum = x + y; // Breaks
```

Function example:

```rust
fn add_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main() {
    let three = Some(3);
    let four = add_one(three);
    let none = add_one(None);
}
```
