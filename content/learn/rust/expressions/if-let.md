+++
title = "If Let"
+++

The `if let` is the little sibling to `match`. `if let` allows you to match only one arm of a `match` expression. The `if let` expression takes a pattern on the left hand side of an EQUAL SIGN and an expression on the right hand side of an EQUAL SIGN.

`if let` can also be combined with normal `if-else` statements.

Example:

```rust
#[derive(Debug)]
enum Breed {
    Husky,
    Poodle,
    Lab,
    // Etc.
}

enum Animal {
    Cat,
    Giraffe,
    Dog(Breed), // This Animal::Dog type contains Breed data
}

fn main() {
    // Example A
    let some_i32 = Some(17i32); // Make 17 an i32 type
    if let Some(3) = some_i32 { // Notice the "=" sign
        println!("Three!");
    } else {
        println!("The number was not three :(");
    }

    // Example B
    let dog = Animal::Dog(Breed::Husky);
    if let Animal::Dog(breed) = dog {
        println!("We found a {:?} dog!", breed);
    }
}
```
