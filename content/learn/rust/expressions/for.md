+++
title = "For"
+++

For loops are used to iterate over collections. Don't worry, a normal C++ for loop is still applicable. You just need to make it more like Python and iterate through some numbers!

```rust
fn main() {
    let arr = [1, 2, 3, 4, 5];

    // Iterate through a collection
    for element in arr.iter() {
        println!("The value is: {}", element);
    }

    // Iterate through a range of values
    for number in 1..6 {
        println!("The value is: {}", number);
    }

    // Iterate through a range, backwards
    for number in (1..6).rev() {
        println!("The value is: {}", number);
    }

    // Inclusive end
    for _ in 1..=9 {}

    // Use vars
    let a = 0;
    let b = 10;
    for i in a..b {
        println!("{}", i)  # println! requires a string literal, not just a variable
    }
}
```
