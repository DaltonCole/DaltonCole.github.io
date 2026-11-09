+++
title = "Should Panic"
+++

If a test should panic, add `#[should_panic]` before the function declaration.

```rust
#[test]
#[should_panic]
fn this_should_panic() {
    panic_function(1000);
}
```
