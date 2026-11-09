+++
title = "Match"
+++

`match` is like a switch-case in most other languages. It consists of the `match` keyword followed by an expression (generally a single variable). In the curly brackets comes the arms. Each arm consists of a pattern and some code.

Matches are exhaustive. Every case for an enum must be handled. `match` does provide a catch all via the `_` placeholder.

A `|` can be used to match multiple patterns.

Each arm of a match expression must return the same type (the "never type" is an exception).

Example:

```rust
#[derive(Debug)]
enum Breed {
    Husky,
    Poodle,
    Lab,
    // Etc.
}

enum Animal {
    Cat,
    Giraffe,
    Dog(Breed), // This Animal::Dog type contains Breed data
    Fox,
    Hamster,
    Snake,
    Daltie,
}

fn sound(animal: Animal) -> String {
    match animal {
        Animal::Cat => String::from("Meow"),
        Animal::Giraffe => String::from("Hello good Sir"),
        Animal::Dog(breed) => { // Function like arm block
            println!("I am a {:?}", breed);
            String::from("Woof")
        },
        Animal::Snake | Animal::Daltie => String::from("I'm a sssnake!"),
        _ => String::from("??"), // Catches the Fox and Hamster cases
    }
}

fn main() {
    let husky = Animal::Dog(Breed::Husky);
    println!("{}", sound(husky));

    // Match range
    let x = 2;
    
    match x {
        1..=7 => println!("Small num"),
        _ => println!("Invalid"),
    }
}
```

###### Destructing

Structs, enums, tuples, and references can be destructed to their individual parts.

```rust
struct Point{ x: u32, y: u32 }

fn main() {
    let p = Point { x: 0, y: 7 };
    let Point{ x: a, y } = p; // a and y are now valid variables in this scope

    // Match destructured values
    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),
        Point { x, y } => println!("Somewhere, over a rainbow? ({}, {})", x, y),
    }
}
```

###### Ignoring Parts

```rust
struct Point{ x: u32, y: u32 , z: u32}

fn main() {
    let p = Point { x: 0, y: 7, z: 14 };

    // Ignore everything other than y
    match p {
        Point { y, .. } => println!("Only care about y: {}", y), // 7
    }

    let numbers = (1, 2, 3, 4, 5);

    // Only match first and last values
    match numbers {
        (first, .., last) => println!("{}, {}", first, last), // 1, 5
    }
}
```

###### Match Guards

```rust
fn main() {
    let num = Some(7);

    match num {
        Some(x) if x < 5 => println!("Less than 5"),
        // Match expressions much be exhaustive, so not including
        // this line would cause an complication error
        Some(x) => println!("{}", x),
        None => (),
    }
}
```

###### @

`@` allows you to both test a pattern and bind a variable.

```rust
enum Height {
    Meters { m: u32 },
}

fn main() {
    let tall = Height::Meters{ m: 7 };

    match tall {
        Height::Meters {
            m: meters_var @ 0..=9,
        } => println!("You're only {} meters tall! You short!", meters_var),
        Height::Meters { m: 10..=99 } => {
            println!("You fit") // variable m is not available, need @ if want variable binding
        }
        Height::Meters { m } => println!("{} is so tall", m), // Since there is no testing, m is available
    }
}
```
