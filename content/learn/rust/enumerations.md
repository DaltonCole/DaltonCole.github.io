+++
title = "Enumerations"
+++

Enums are great for categorical data, where something, like an IP address, can belong to one category, such as IPv4 or IPv6. Enums are heavily used in Rust, unlike in C++. Example:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Function that uses an enum as a parameter
fn coin_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// Enums can contain data
enum Message {
    Move { x: i32, y: i32 }, // Anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32), // three i32 values
    Quit,
}

// Method of Message
impl Message {
    fn call(&self) {
        // Body
    }
}

fn main() {
    // Use the namespace syntax to choose a category
    let c = Coin::Dime;
    println!("Coin value: {}", coin_value(c));

    let m = Message::Write(String::from("hello"));
    m.call();
}
```
