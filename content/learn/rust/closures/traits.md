+++
title = "Traits"
+++

Closures can implement some or all of three traits:

- `Fn`: Borrows the values from the environment immutably
- `FnMut`: Mutably borrow values
- `FnOnce`: Consumes the variables it captures from its environment. This closure can be called only once because it consumes the variables.

###### Fn

```rust
struct Cacher<T>
where
    T: Fn(i32) -> i32,
{
    func: T,
    value: Option<i32>,
}

impl<T> Cacher<T>
where
    T: Fn(i32) -> i32,
{
    fn new(func: T) -> Cacher<T> {
        Cacher{
            func,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.func)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    let add = 5;
    let mut saved_value = Cacher::new(|x| x + add);
    println!("{}", saved_value.value(2));   // 7
    println!("{}", saved_value.value(738)); // 7, because cached
}
```
