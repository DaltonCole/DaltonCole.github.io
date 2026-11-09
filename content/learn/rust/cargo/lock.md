+++
title = "Lock"
+++

`Cargo.lock` is used in conjunction with your dependences in `Cargo.toml`. The `Cargo.lock` file locks in the version of your dependences when `$ cargo build` was first executed with the dependency. With this lock, if a newer minor release or other release for the dependency is added, your project will continue to use the old release.
