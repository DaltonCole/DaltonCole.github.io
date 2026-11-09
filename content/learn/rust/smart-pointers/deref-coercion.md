+++
title = "Deref Coercion"
+++

Deref coercion allows a referenced type to another type, for example, `&String` can be converted to `&str` because `String` implements the `Deref` trait and returns `&str`.

The `DerefMut` trait can be used to override the `*` operator on mutable references.

In addition, mutable references can be coerced to become immutable if the `Deref` trait is implemented.

Example:

```rust
use std::ops::Deref;
use std::ops::DerefMut;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn deref_coercion(name: &str) {
    println!("{}", name);
}

fn deref_mut_coercion(num: &mut f32) {
    *num += 3.0
}


fn main() {
    let x = MyBox::new(2);
    assert_eq!(2, *x);

    let n = MyBox::new(String::from("Rust"));
    deref_coercion(&n);
    let mut m = MyBox::new(12.2);
    deref_mut_coercion(&mut m);
}
```
