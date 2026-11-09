+++
title = "Move"
+++

`move` allows you to move local variable's ownership to a closure.

```rust
use std::thread;

fn main() {
    let v = vec![0, 1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    // Cannot use v in main thread anymore

    handle.join().unwrap();
}
```
