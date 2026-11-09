+++
title = "Use"
+++

The keyword `use` can be used to import code and to shorten absolute/relative paths. It is recommended to import the module and not the specific function so when used, it is obvious that the call is not to a local function. When importing anything other than a function (structs, enums, etc), use the full path.

Example:

```rust
mod garden {
    // Nest a module inside of another
    pub mod food {
        // Add structs, enums, constants, traits, modules, functions, etc. here
        pub fn harvest() {}
        fn water() {}
    }

    pub mod soil {
        pub fn quality() {}
    
        pub fn amount() { quality(); }
    }
}

// Absolute path
use crate::garden::food;
// Relative path
use self::garden::soil;

// Full path for structs
use std::collections::HashMap;


fn main() {
    food::harvest();
    soil::quality();
	let mut map = HashMap::new();
}
```

###### As

When there is a name collision with `use`, the keyword `as` can be used in conjunction. Example:

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn func1() -> Result { // ...  }
fn func2() -> IoResult<()> { // ... }
```

###### Pub Use

`pub use` can be used to re-export your code. For example, if you import A into B then import B into C, C would not have access to A because A would be private. If A was imported to B using `pub use`, then C would have access to A.

Re-exporting is especially useful when creating an API. Inside of `src/lib.rs`, you can do `pub use self::path::to::thing` to make others be able to use `thing` via `use my_crate::thing` in their code. This allows your API to be more strait-forward than your internal structure.

###### Nested Paths

Example:

```rust
use std::{cmp::Ordering, io}; // Bring in both std::cmp::Ordering and std::io
use std::io::{self, Write}; // std::io and std::io::Write
```

###### Glob

Brings in all public items from a path into scope. Example:

```rust
use std::collections::*;
```
