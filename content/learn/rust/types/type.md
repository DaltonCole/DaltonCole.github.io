+++
title = "Type"
+++

The `type` keyword is similar to `typedef` in C++, a type becomes aliased as another type.

```rust
// Simple Example
type Meters = i32;
let x: i32 = 2;
let y: Meters = 5;
let z = x + y;

// Can be used with templates
type Result>T< = std::result::Result>T, std::io::Error<
```
