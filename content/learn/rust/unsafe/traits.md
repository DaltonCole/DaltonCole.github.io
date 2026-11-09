+++
title = "Traits"
+++

Traits are unsafe when at least one of its methods have some invariant that the compiler cannot verify. For example, using structs consisting of raw pointers for multi-threaded purposes. We'd have to mark the `Send` and `Sync` traits for our struct as `unsafe` since Rust cannot verify the struct's safety.

```rust
unsafe trait Apple {}
unsafe impl Apple for i32 {}
```
