+++
title = "Function Pointers"
+++

You can pass functions as parameters to functions as well! The `fn` type is a function pointer.

Function pointers implement all three closure traits `Fn`, `FnMut`, and `FnOnce`).

```rust
fn double(x: u32) -> u32 {
    x * 2
}

fn do_twice(f: fn(u32) -> u32, arg: u32) -> u32 {
    f(arg) * f(arg)
}

fn main() {
    assert_eq!(
        16, 
        do_twice(double, 2)
    )
}
```
