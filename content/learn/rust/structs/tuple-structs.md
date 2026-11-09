+++
title = "Tuple Structs"
+++

You can also create structs that behave like tuples, i.e. structs without named fields. This can be useful when you want to pass tuples that contain specific information into a function.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);

    print_color(white);
    //print_color(origin); // Breaks
}

fn print_color(color: Color) {
    // Use "." followed by the index to index into a Tuple Struct
    println!("({}, {}, {})", color.0, color.1, color.2);
}
```
