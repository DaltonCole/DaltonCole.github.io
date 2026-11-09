+++
title = "Associated Types"
+++

Associated types act as a type placeholder for traits in method signatures. When the trait is implemented, the associated type is replaced with the concrete type.

```rust
struct Point {x: i32, y: i32}

trait Contains {
    // Declare placeholder types
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}

impl Contains for Point {
    // Specify types during implementation
    type A = i32;
    type B = i32;

    fn contains(&self, num1: &Self::A, num2: &Self::B) -> bool {
        (&self.x == num1) && (&self.y == num2)
    }
    // This would also be a valid signature:
    //fn contains(&self, num1: &i32, num2: &i32) -> bool {}
}

fn main() {
    let num1 = 2;
    let num2 = 3;

    let points = vec![Point{x: 1, y: 2}, Point{x: 2, y: 3}];

    for point in points {
        match point.contains(&num1, &num2) {
            true => println!("Does contain point!"),
            false => println!("Does NOT contain point!"),
        }
    }
}
```
