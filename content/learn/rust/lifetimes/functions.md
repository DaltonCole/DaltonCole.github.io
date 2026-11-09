+++
title = "Functions"
+++

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s;
    let x = String::from("Apple");
    {
        let y = String::from("Pie");
        s = longest(&x, &y);
        println!("{}", s);
    }
    // print!("{}", s);  // y does not live long enough, therefor s cannot be used here
}
```
