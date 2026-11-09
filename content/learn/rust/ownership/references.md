+++
title = "References"
+++

###### Functions

Rust lets a function borrow a variable using references.

```rust
fn main() {
    let s1 = String::from("Daltie");
    let len = find_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn find_length(s: &String) -> usize {
    s.len()
}
// s is never actually given ownership, it is simply borrowing the String,
// thus nothing happens when s goes out of scope
```

###### Mutable References

Rust allows for exactly one mutable reference per scope (as long as the variable is active).

```rust
fn main() {
    let mut s = String::from("Daltie");

    change(&mut s);

    let r1 = &mut s;
    r1.push_str(" is ");
    println!("{}", r1); // "Daltie Cole is"
    
    let r2 = &mut s; // Okay because r1 is never used after r2 creation
    r2.push_str(" cool!");
    println!("{}", r2); // "Daltie Cole is cool!"

    // Either unlimited immutable references are allowed at once OR one mutable reference is allowed per scope
    let r3 = &s
    let r4 = &s
    //let r5 = &mut s; // BREAKS

    println!("{}, {}", r3, r4);
}

fn change(some_str: &mut String) {
    some_str.push_str(" Cole");
}
```
