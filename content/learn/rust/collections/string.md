+++
title = "String"
+++

Strings are encoded as UTF-8 characters in Rust, thus they are a bit weird. They cannot be indexed into since UTF-8 is variable lengthen. [Docs.](https://doc.rust-lang.org/std/string/struct.String.html)

```rust
fn main() {
    // Initialize
    let mut s = String::new();
    let const_str = "str, not String";
    s = const_str.to_string();
    let mut s = String::from("hello");
    let s2 = String::from(" This is Daltie!");

    // Append
    s.push_str(" world");
    s.push('!');
    let s3 = s + &s2 // s has been moved and can no longer be used
    let s1 = String::from("Hello World!");
    let s3 = format!("{} {}", s1, s2);

    // Iterating Over
    for c in s3.chars() {
        println!("{}" ,c);
    }
}
```
