+++
title = "Functions"
+++

Similar to assignment, variables are either moved or copied into a function. Same for return values. For example:

```rust
fn main() {
    let s1 = String::from("Daltie");
    takes_ownership(s1); // s1 is no longer valid

    let x = 3;
    makes_copy(x); // x is still valid

    let s2 = gives_ownership(); // The return value from gives_ownership() is moved

    let s3 = takes_and_gives_ownership(s2); // s2 is no longer valid
} 
// s3 is dropped. 
// s1 and s2 were already moved, so nothing happens. 
// x goes out of scope.

fn takes_ownership(some_str: String) {
    println!("{}", some_str);
}

fn makes_copy(some_int: i32) {
    println!("{}", some_int);
}

fn gives_ownership() -> String {
    let some_str = String::from("Daltie");
    some_str
}

fn takes_and_gives_ownership(some_str: String) -> String {
    some_str
}
```
