+++
title = "Library"
+++

A package can only have one (or none) library crates.

If a package directory contains `src/lib.rs`, the package contains a library crate with the same name as the package with `src/lib.rs` being the crate root. If both `src/lib.rs` and `src/main.rs` exist, then the package has two crates
