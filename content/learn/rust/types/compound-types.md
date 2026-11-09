+++
title = "Compound Types"
+++

Compound types combine multiple values into a single type. Rust has two primitive compound types: tuples and arrays.

###### The Tuple Type

Tuples have a fixed length and can contain different types.

```rust
fn main() {
    let tup = (10, 1_234, 5678, 12.4, true, 'a');
    let tup : (i32, i32, i32, f64, bool, char) = tup;

    // Destruct tup through pattern matching
    let (a, b, c, d, e, f) = tup;
    println!("{}, {}, {}, {}, {}, {}", a, b, c, d, e, f);

    // Destruct tup through indexing
    let ten = tup.0;
    let c = tup.5;
    println!("{}, {}", ten, c);

    // Return multiple values using a tuple
    let s1 = String::from("Daltie");
    let (s2, len) = find_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

// Multiple values can be returned from a function using tuples
fn find_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
```

###### The Array Type

Arrays are of FIXED length and all elements must be of the same type. See vector in the standard library for a variable length container.

Array data is allocated on the stack rather than the heap (speedy!).

```rust
fn main() {
    // Array Declaration
    let a = [-1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // i32 is the type, 5 is the length
    let a = [3; 5]; // [3, 3, 3, 3, 3]

    // Indexing
    let first = a[0];
    let second = a[1];

    // Out of Bounds Runtime error
    let will_panic = a[12]; // Rust will panic and crash instead of trying to access invalid memory
}
```
