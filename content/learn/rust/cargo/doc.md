+++
title = "Doc"
+++

Rust comes with documentation comments. To use documentation comments, start a line with `///`. These comments support markdown notation and can be converted to HTML using [`cargo doc`](https://doc.rust-lang.org/cargo/commands/cargo-doc.html). The documents will be saved to `target/doc`

The other documentation comment is `//!`. This type of comment is used to add comments to what it is in (like crates) as opposed to what is after the comment (a function). These are common at the beginning of a file.

Example (in `math/src/lib.rs`):

````rust
//! # Math
//!
//! `math` is a collection of mathematical functions

/// Squares the given number
///
/// # Examples
///
/// ```
/// let square = math::square(5);
///
/// assert_eq!(25, square);
/// ```
/// ```
/// let square1 = math::square(1);
///
/// asswert_eq!(1, square1);
/// ```
pub fn square(x: i32) -> i32 {
    x * x
}
````

###### Flags

- `--open`: Build and open the HTML documentation

###### Sections

- `Errors`: If a `Result` is returned, what kind of errors might occur and how are they caused
- `Examples`: Example usage
- `Panics`: What scenarios causes the function to panic
- `Safety`: If `unsafe` is used, explain why and the variants the caller must uphold

###### Tests

`cargo test` actually tests the code left in documentation comments!
