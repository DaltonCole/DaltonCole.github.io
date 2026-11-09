+++
title = "Integration Tests"
+++

Integration tests are external to your library. They can only call your library's public API.

Integration tests are only for library crates. For binary crates, only unit tests can be used.

Integration tests go inside of a *tests* directory next to your *src* directory. Example:

```rust
// src/lib.rs in crate addr
pub fn add_two(x: i32) -> i32 {
    x + 2
}

// tests/integration_tests.rs
use addr;

#[test]
fn it_adds_two() {
    assert_eq!(4, addr::add_two(2));
}
```

###### Submodules

Each file in the *tests* directory is compiled as its own separate crate. To prevent this, put code you don't want separately tested into subdirectories inside of *tests*. These subdirectories will not be tested, but can act as support code for your tests.
