+++
title = "Command Line Arguments"
+++

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); // Turn argument iterator into a vector
    println!("{:?}", args); // Prints the arguments along with the binary name

    // Convert first argument to i32
    let num: i32 = args[1].parse().unwrap();
}
```
