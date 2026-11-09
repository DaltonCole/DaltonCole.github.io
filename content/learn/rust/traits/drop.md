+++
title = "Drop"
+++

The [`drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html) trait is similar to the deconstructor in C++. When a variable that has resources on the heap, it needs to implement the `drop` trait so when the variable leaves its scope, the resources are deallocated. If the `drop` trait in implemented, the `copy` trait cannot be.
