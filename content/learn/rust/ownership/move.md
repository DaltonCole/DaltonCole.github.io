+++
title = "Move"
+++

In Rust, you need to know what variables will be put on the stack and which variables will be added to the heap. Heap and stack variables are treated differently.

###### Stack

Stack variables consist of basic types with a known size at compile time. Stack variables are valid until they go out of scope. For example:

```rust
let x = 3;
let y = x;
```

Both `x` and `y` are valid and contain the value `3` until they go out of scope.

###### Heap

Heap variables follow similar but more strict rules to shallow copies in Python in the sense that the new variable will point to the data, however, in Rust, the old variable becomes invalid. For example:

```rust
let s1 = String::from("hello");
let s2 = s1;
```

Line 1 initializes a String variable by creating a string of size 5(ish) on the heap and a pointer to said string on the stack. Line 2 then takes ownership of this heap string. `s1` now no longer points to the string and `s1` is no longer valid and cannot be used until it is re-assigned. The data is NEVER **copied**, instead it behaves similarly to C++'s move semantics.

See the [Rust Book](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move) for fantastic diagrams of this process, specifically Figure 4-4.
