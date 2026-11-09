+++
title = "Unrecoverable"
+++

`panic!` is how to raise an unrecoverable error.

By default, when a panic occurs, the program starts unwinding by walking up the stack and cleaning up the data for each function. Alternatively, you can *abort*, which ends the program without cleaning up. Aborting leads to a smaller binary.

In order to make panics abort instead of unwind in production code (to create smaller binaries), you must add the following to your `Cargo.toml` file:

```toml
[profile.release]
panic = 'abort'
```

###### Backtrace

Rust provides backtracing. To use do:

```bash
$ RUST_BACKTRACE=1 cargo run
```
