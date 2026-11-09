+++
title = "Vector"
+++

Just like a C++ vector. [Docs.](https://doc.rust-lang.org/std/vec/struct.Vec.html)

```rust
fn main() {
    // With type annotation
    let _v: Vec<i32> = Vec::new();
    // With type deduction (vec! macro)
    let mut v = vec![1, 2, 3];
    for i in 5..8 {
        v.push(i);
        
    }

    // --- Read element --- //
    // & [index], causes a panic if accessing memory outside of vector
    let _third: &i32 = &v[2];
    let fourth = &v[3];
    let _fourth_copy = v[3]; // Copy of the fourth element
    println!("Forth: {}", fourth);
    // .get() returns Option<&T>
    match v.get(4) {
        Some(fifth) => println!("The fifth element is: {}", fifth),
        None => println!("There is no fifth element!"),
    }

    // -- Iterating -- //
    for i in &v {
        println!("{}", i);
    }
    // Mutable
    for i in &mut v {
        *i += 1; // Dereference operator required to update

    }

    // --- Multi-Type Vector Hack --- //
    enum Colors {
        Rgb(i32),
        Grayscale(f32),
    }
    let _pixels = vec![
        Colors::Rgb(255),
        Colors::Grayscale(3.5),
    ];

    // --- Useful Methods --- //
    // Length
    let _len = v.len();
    // Clear
    v.clear();
    // Insert
    v.insert(0, 4); // Index, value
    // Remove
    let _value = v.remove(0); // Index
    // Push/Pop back
    v.push(5);
    let _value = v.pop();
}
```
