+++
title = "Return Values"
+++

Rust requires you to declare a return type for a function if it returns something.

The last line in a function is returned implicitly as long as it is an expression (no semi-colon). A `return` statement can be used to return prior to the last line.

```rust
fn three() -> i32 {
    3 // Notice the lack of a semi-colon. Expressions return something, statements do not.
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn minus_one(x: i32) -> i32 {
    // Because we are explicitly using "return" we can either make this
    // a statement or an expression.
    return x - 1; 
}

fn no_return(x: i32) {
    println!("{}, x");
}

fn main() {
    let x = three();
    println!("x is: {}", x);

    let x = plus_one(x);
    println!("x is now: {}", x);

    let x = minus_one(x);
    println!("x is finally: {}", x);
}
```
