+++
title = "Unsafe"
+++

The `unsafe` keyword allows you to create a block of code where some of Rust's safety guarantees are no longer applied. This is sometimes necessary when Rust is being to conservative.

Use cases:

- Dereference a raw pointer
- Call and unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access fields of [`union`s](https://doc.rust-lang.org/reference/items/unions.html)

It is the programmer's responsibility to make sure the code in a `unsafe` block is safe.
