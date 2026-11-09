+++
title = "Box"
+++

A `Box<T>` stores data on the heap rather than the stack. A pointer to that heap data remains on the stack. They are useful when:

- Don't know the size of the type at compile time but will need to know the exact size during run time
- Want to transfer ownership of a large amount of data without copying.
- When you own a value and only care about its implemented traits, not its type (polymorphism)

Immutable or mutable borrow checking is done at compile time.

Example:

```rust
pub enum LinkedList {
    Con(i32, Box<LinkedList>),
    Nil,
}


use crate::LinkedList::{Con, Nil};

fn main() {
    let list = Con(1,
        Box::new(Con(2, 
                Box::new(Con(3, Box::new(Nil))))));

    assert_eq!(1, match list {
        Con(value, next) => value,
        Nil => -1,
    });
}
```
