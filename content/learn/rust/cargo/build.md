+++
title = "Build"
+++

`$ cargo build` compiles your code in `./srs` and saves the results to `./target/debug` by default. This command also creates a `Cargo.lock` file in the base directory. This file is used to keep track of the exact versions of dependencies in the project.

Flags:

- `--release`: Compiles the source code with optimizations. Saves the results to `target/release`
