+++
title = "Never Type"
+++

If a function will never return use the never return type `!`.

```rust
fn foo() -> ! {}
```

Some things that return the never type are `continue` and `panic!`.
