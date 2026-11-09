+++
title = "Module"
+++

Modules are used to organize code within a crate into groups and control the privacy of items (public vs private). The C++ counterpart would be a namespace, except Rust takes it a step further.

Modules are created by using the `mod` keyword.

The modules in `src/main.rs` and `src/lib.rs` are called "crate roots" and is nested under `crate`.

Modules are private by default (like C++ private methods in a class). To make them public, you have to add the `pub` keyword. All parents of that module are now public, but children are still private unless specified otherwise.`pub` can be used on structs, enums, functions, and methods as well.

The keyword `super` allows you to refer to something in the parent's scope.

Example:

```rust
mod garden {
    // Nest a module inside of another
    pub mod food {
        // Add structs, enums, constants, traits, modules, functions, etc. here
        pub fn harvest() {
            super::soil::amount();
            super::super::plant_garden();
        }
        fn water() {}
    }

    pub mod soil {
        pub fn quality() {}
    
        pub fn amount() { quality(); }
    }
}

pub fn plant_garden() {
    // Relative path
    garden::soil::quality();
    // Absolute path
    crate::garden::soil::amount();
}

fn main() {
    // To access harvest, you'll have to do:
    crate::garden::food::harvest();
    // Both food and harvest must be marked as pub
}
```
