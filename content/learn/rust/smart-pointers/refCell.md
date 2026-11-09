+++
title = "Refcell"
+++

`RefCell<T>` is like `Box<T>` except the borrow checker is checked at runtime instead of compile time. This allows the programmer to make code that they know will work, but the borrow checker is unsure about.

`RefCell<T>` is only used in single-threaded scenarios.

Both immutable and mutable borrow checking is done at runtime. The value inside of `RefCell<T>` can be mutated even if `RefCell<T>` is immutable.

This is useful when you need to mutate something that is passed in as immutable.
