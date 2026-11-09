+++
title = "Reference Counted"
+++

A reference counted smart pointer is the same as a shared pointer in C++. The `Rc<T>` pointer keeps track of the number of references to it and frees up memory when the reference count is zero.

`Rc<T>` can only be used for single-threaded scenarios.

Only immutable borrow checking is done at compile time.

**`Rc<T>` can only hold immutable data. Pair with `RefCell<T>` for mutability.**

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let v = Rc::new(RefCell::new(vec![1, 2, 3]));
    let w = Rc::clone(&v);
    let x = Rc::clone(&v);

    // Add 4 to vector
    v.borrow_mut().push(4);

    //would_cause_panic(&v);

    println!("{:?}", x);
}

fn would_cause_panic(v: &Rc<RefCell<Vec<u32>>>) {
    let mut one_borrow = v.borrow_mut();
    // Panic because two mutable references
    // During rumtime with RefCell
    let mut two_borrow = v.borrow_mut();
}
```

###### Strong vs Weak Count

When `strong_count` reaches `0`, the instance is cleaned up. Clean up does not care how many weak references there are. Using `Weak<T>` is one way to help prevent memory leaks (via circular referencing).

Since what `Weak<T>` references may have been dropped, it is necessary to call `upgrade` which returns an `Option<RC<T>>` to verify that the reference still exists.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 6,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade()); // None

    let branch = Rc::new(Node {
        value: 42,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Downgrade strong to weak
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:#?}", leaf.parent.borrow().upgrade()); // Some
}
```
