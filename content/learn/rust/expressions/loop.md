+++
title = "Loop"
+++

Rust has three kinds of loops: `loop`, `while`, and `for`

There also exists the `break` statement to break out of a loop and the `continue` statement to immediately go to the next iteration of the loop.

Values can be returned from loops. For example:

```class
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 5 {
            break counter * 7;
        }
    };

    println!("result = {}", result);
}
```

###### Loop

The `loop` loop creates an infinite loop.

```class
fn main() {
    loop {
        println!("Never ending!");
    }
}
```
