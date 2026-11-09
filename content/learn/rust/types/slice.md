+++
title = "Slice"
+++

A slice is a contiguous sequence of elements in a collection. The advantages of slices is they allow you to *borrow* part of a collection, such as a String or array. This is safe borrowing, if part of the original data is altered, the borrowed data is no longer valid.

###### String Slice

```rust
fn main() {
    let mut name = String::from("Daltie Cole");

    let f_name = first_name(&name[..]); // Let first_name() borrow name. Pass as a string literal

    //name.clear(); // Clear out the name

    // If name was cleared, this line would not compile
    println!("First name: {}", f_name);

    // Other slicing fun
    let daltie = &name[..7];
    let cole = &name[8..];
    let full_name = &name[..];
}

// By using &str as the parameter type instead of &String, string literals can be passed as well
fn first_name(name: &str) -> &str {
    // Convert name into an array of bytes
    let bytes = name.as_bytes();

    // .iter() creates an iterator over the array
    // .enumerate() wraps iter() into a series of tuples where
    //   the first element is the index and the second is the item
    for (i, &item) in bytes.iter().enumerate() {
        // If the element is a binary space
        if item == b' ' {
            // Return the slice from the beginning to the current point
            return &name[0..i];
        }
    }

    &name[..]
}
```
