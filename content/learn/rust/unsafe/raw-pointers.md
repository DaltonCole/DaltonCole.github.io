+++
title = "Raw Pointers"
+++

Unsafe rust has two raw pointer types:

- `*const T`
- `*mut T`

Raw pointers:

- Ignore borrowing rules
- Do not guarantee the pointed to memory is valid
- Can be null
- No automatic clean-up

```rust
fn main() {
    let mut num = 4;

    // Raw pointers can be created in safe mode
    let rp1 = &num as *const i32; // *const i32 is the type
    let rp2 = &mut num as *mut i32; // *mut i32 is the type

    // Deallocating raw pointers can only be done in unsafe mode
    unsafe {
        //*rp1 = 10; // Does not work because *const
        println!("*rp1 = {}", *rp1);
        *rp2 = 12;
        println!("*rp2 = {}", *rp2);
    }
}
```
