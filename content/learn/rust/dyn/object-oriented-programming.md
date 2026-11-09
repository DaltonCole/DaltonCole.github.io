+++
title = "Object Oriented Programming"
+++

Rust is not a true OOP language as it does not have inheritance. Rust does allow for a polymorphic like interface via `dyn` though. Example:

```rust
// Trait for dyn
trait Animal {
    // The use of self instead of &self allows the function to take
    // ownership, and thus invalidating the old state
    fn grow_up(self: Box<Self>) -> Box<dyn Animal>;
    fn age(&self);
}

// Container. Similar to parent class
pub struct DogHouse {
    pub residence: Option<Box<dyn Animal>>
}

impl DogHouse {
    pub fn new() -> DogHouse {
        DogHouse { residence: None }
    }

    pub fn get_puppy(&mut self, s: &str) {
        self.residence = Some(Box::new(Puppy::new(s)));
    }

    pub fn grow_up(&mut self) {
        if let Some(s) = self.residence.take() {
            self.residence = Some(s.grow_up())
        }
    }

    pub fn residence_age(&self) {
        if let Some(s) = &self.residence {
            s.age()
        }
    }
}

pub struct Puppy {}

pub struct Dog {}

impl Puppy {
    pub fn new(name: &str) -> Puppy {
        Puppy{}
    }
}

impl Animal for Puppy {
    fn grow_up(self: Box<Self>) -> Box<dyn Animal> {
        Box::new(Dog {})
    }

    fn age(&self) {
        println!("Puppy!");
    }
}

impl Animal for Dog {
    fn grow_up(self: Box<Self>) -> Box<dyn Animal> {
        self
    }

    fn age(&self) {
        println!("Doggo!");
    }
}

fn main() {
    let mut red_dog_house = DogHouse::new();
    red_dog_house.get_puppy("Spike");
    red_dog_house.residence_age(); // Puppy
    red_dog_house.grow_up();
    red_dog_house.residence_age(); // Doggo
}
```
