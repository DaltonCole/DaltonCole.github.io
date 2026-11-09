+++
title = "Propagating Errors"
+++

To proprogate and error, return the error.

###### The `?` operator

The `?` operator can be used to immediately return an error from a function! The `?` operator is only valid with functions that return `Result` or `Option` (or any type that implements `Try`). Example:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_from_file(f_name: &str) -> Result<String, io::Error> {
    let f = File::open(f_name);

    let mut opened_file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match opened_file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn shortcut_read_from_file(f_name: &str) -> Result<String, io::Error> {
    let mut f = File::open(f_name)?;  // Return Err if error occurred
    let mut s = String::new();
    f.read_to_string(&mut s)?; // Return Err if error occurred
    Ok(s)
}

fn super_short_read_from_file(f_name: &str) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

use std::fs;
fn the_shortest_read_from_file(f_name: &str) -> Result<String, io::Error> {
    fs::read_to_string(f_name)
}

fn main() {
    let f_name = String::from("file.txt");
    let s1 = read_from_file(&f_name).unwrap();
    println!("{}", s1);
}
```
