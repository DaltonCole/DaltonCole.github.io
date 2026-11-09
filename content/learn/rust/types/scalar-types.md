+++
title = "Scalar Types"
+++

Rust has four primary scalar types:

- Integers
- Floating-Point Numbers
- Booleans
- Characters

###### Integer Types

<table><caption>Integer Types</caption><thead><tr><th>Length</th><th>Signed</th><th>Unsigned</th></tr></thead><tbody><tr> <td>8-bit</td> <td><code>i8</code></td> <td><code>u8</code></td></tr><tr> <td>16-bit</td> <td><code>i16</code></td> <td><code>u16</code></td></tr><tr> <td>32-bit</td> <td><code>i32</code></td> <td><code>u32</code></td></tr><tr> <td>64-bit</td> <td><code>i64</code></td> <td><code>u64</code></td></tr><tr> <td>128-bit</td> <td><code>i128</code></td> <td><code>u128</code></td></tr><tr> <td>arch*</td> <td><code>isize</code></td> <td><code>usize</code></td></tr></tbody></table>

The "arch" length is system architecture dependent.

<table><caption>Integer Literals</caption><thead><tr><th>Number Literals</th><th>Example</th></tr></thead><tbody><tr><td>Decimal</td><td><code>12_345_678</code></td></tr><tr><td>Hex</td><td><code>0xfe</code></td></tr><tr><td>Octal</td><td><code>0o76</code></td></tr><tr><td>Binary</td><td><code>0b1111_0000</code></td></tr><tr><td>Byte (<code>u8</code> only)</td><td><code>b'A'</code></td></tr></tbody></table>

###### Floating-Point Types

<table><caption>Floating-Point Types</caption><thead><tr><th>Length</th><th>Type</th></tr></thead><tbody><tr><td>32-bit</td><td><code>f32</code></td></tr><tr><td>64-bit</td><td><code>f64</code></td></tr></tbody></table>

Note: Rust defaults all floats to `f64`.

###### The Boolean Type

A boolean in Rust is either `true` or `false` and takes up one byte. The type is specified with `bool`.

###### The Character Type

The character type in Rust is a four byte Unicode Scalar Value. For example:

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let laughing_crying_face = '😂';
}
```
