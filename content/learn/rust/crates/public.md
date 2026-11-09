+++
title = "Public"
+++

The keyword `pub` makes something public, like a function, module, enum, struct etc.

However, structs are special. `pub` in front of them just makes them public, not the members inside. The members will not be read nor writable. To make them read/writable, add the `pub` keyword to the variable itself.

Example:

```rust
mod house {
    pub struct Kitchen {
        pub plates: i32,
        sinks: i8,
    }

    impl Kitchen {
        pub fn duel_sink(num_plates: i32) -> Kitchen {
            Kitchen {
                plates: num_plates,
                sinks: 2,
            }
        }
    }
}

pub fn build_home() {
    let mut home = house::Kitchen::duel_sink(8);
    home.plates += 1;

    // Cannot read nor write to private struct field
    //home.sinks += 1;
    //println!("Num sinks: {}", home.sinks);
}
```
