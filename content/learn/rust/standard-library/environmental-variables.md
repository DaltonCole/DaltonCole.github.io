+++
title = "Environmental Variables"
+++

```rust
use std::env;

fn debug() {
    let DEBUG = env::var("DEBUG").is_err();
}
```
