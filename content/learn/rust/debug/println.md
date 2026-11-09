+++
title = "Println"
+++

The `println!` macro offers two different ways to print debugging information: `{:?}` and `{:#?}`. The `{:#?}` option is a bit prettier.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 25,
        height: 40,
    };

    // {:?}
    println!("rect is {:?}", rect);
    // Output:
    // rect is Rectangle { width: 25, height: 40  }

    // {:#?}
    println!("rect is {:#?}", rect);
    // Output:
    // rect is Rectangle {
    //     width: 25,
    //     height: 40,      
    // }
}
```
