+++
title = "Ownership"
+++

Ownership is a central part of Rust. Ownership is similar to a unique_ptr in C++, but for all variables, including referenced variables.

- Each value has a variable that's called its *owner*
- There can only be one owner at a time
- When the owner goes out of scope, the value will be *dropped*
