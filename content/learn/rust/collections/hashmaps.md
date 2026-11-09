+++
title = "Hashmaps"
+++

HashMaps are like unordered maps in C++. [Docs.](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

```rust
use std::collections::HashMap;

fn main() {
    let mut occur = HashMap::new();

    // All keys must be of the same type and all values must be of the same type
    occur.insert(String::from("Blue"), 12);
    occur.insert(String::from("Red"), 5);

    let teams = vec![String::from("Blue"), String::from("Pink")];
    let scores = vec![5, 78];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(scores.into_iter()).collect();
        // Teams and scores becoming iterators. Zip combines them into _, _ iterator
        // Collect transforms the iterator into a container, in this case, a HashMap

    // Hash maps copy for copy traited types, moved for others
    let key = String::from("Moved Key");
    let value = String::from("Moved Value");

    let mut map = HashMap::new();
    map.insert(key, value); // key and value are now invalid identifiers

    // --- Accessing Elements --- //
    let blue_score = scores.get("Blue");  // Wrapped in Some -> Some(&7)

    let team_name = String::from("Pink");
    let pink_score = scores.get(&team_name);  // Some(&78)

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // --- Updating --- //
    // Overwriting
    scores.insert(String::from("Blue"), 12);
    // Update if key does not exist
    scores.entry(String::from("Purple")).or_insert(42);
    // Update an existing value
    let mut map = HashMap::new();
    let text = "apples pies are full of pies";
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // count is a mutable reference
        *count += 1
    }
    println!("{:?}", map); // HashMaps do NOT retain order

}
```
