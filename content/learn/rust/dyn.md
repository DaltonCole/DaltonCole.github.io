+++
title = "Dyn"
+++

The `dyn` keyword allows Rust to vaguely have the concept of inheritance in the sense that a pointer can dynamically point to more than one type.

`dyn` allows a smart pointer like a `Box` point to anything that implements a trait.

Rules:

- Trait function cannot return `self` because a concrete `self` type would not be known at compile time and thus not object safe.
