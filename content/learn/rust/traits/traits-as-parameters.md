+++
title = "Traits As Parameters"
+++

Traits can also be parameters. These functions are similar to generics, but restrict the types to types that implement the trait. Example:

```rust
pub trait Shape {
    fn area(&self) -> f32;
}

pub trait TwoD {}

pub struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * 3.14159
    }
}

impl TwoD for Circle {}

// Traits as parameters - short hand
pub fn print_area(s: &impl Shape) {
    println!("{}", s.area());
}

// Traits as parameters - the long way
pub fn print_area2<T: Shape>(s: &T) {
    println!("{}", s.area());
}

// Multiple Traits
pub fn print_area3<T: Shape + TwoD>(s: &T) {
    println!("{}", s.area());
}

// where can be used to make trait bounds look pretty
pub fn print_areas<T, U>(s: &T, r: &U) 
    where T: Shape + TwoD,
          U: Shape
{
    println!("{}", s.area());
    println!("{}", r.area());
}

// Return type that implements trait
pub fn get_area(s: &str) -> impl Shape {
    if s == "Circle" {
        return Circle { radius: 0.0 };
    }
    Circle { radius: 0.0 }  // Imagine other Shapes being here
}

fn main() {
    let cir = Circle { radius: 12.25 };
    print_area3(&cir);
    print_areas(&cir, &cir);
}
```
