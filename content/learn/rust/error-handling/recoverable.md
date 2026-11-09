+++
title = "Recoverable"
+++

Rust uses a `Result<T, E>` enum to catch errors. Functions that have the possibility of erroring returns the `Result` type. Example:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("file.txt");

    let f = match f {
        Ok(file) => file,  // Return the file
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Cannot create the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

###### Unwrap

`unwrap` can be used as a short cut if you want to `panic!` on error. If an `Ok` is returned, then `unwrap` will return the value inside of the `OK`.

```rust
let f = File::open("file.txt").unwrap();
```

###### Unwrap-Or-Else

Like `unwrap` but executes closure if error occured.

```rust
let f = File::open("file.txt").unwrap_or_else(|err| {
    println!("Error: {}", err);
    // Do something else
});
```

###### Expect

Similar to `unwrap`, `expect` can be used to give your own error message in the case of an error.

```rust
let f = File::open("file.txt").expect("Failed to open file.txt");
```

###### Is Error

Returns true if an error occurred:

```rust
let debug: bool = env::var("DEBUG").is_err();
```
