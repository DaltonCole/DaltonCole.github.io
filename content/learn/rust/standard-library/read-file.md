+++
title = "Read File"
+++

```rust
use std::fs;

fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .expect("Failed to open file {}")
}

fn main() {
    let file_name = "file.txt";
    read_file(file_name);
}
```
