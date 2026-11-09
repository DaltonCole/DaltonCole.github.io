+++
title = "Structs"
+++

Structs are essentially tuples with named fields. For example:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Normal Declaration
    let user1 = User {
        email: String::from("contact@daltoncole.com"),
        username: String::from("drc"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = build_user_annoying_way(String::from("bob@bob.bob"), String::from("bob"));
    let user3 = build_user_easy_way(String::from("john@bob.com"), String::from("john"));

    // Struct Update (quick way to change only some fields from another instance)
    let user4 = User {
        email: String::from("me@daltoncole.com"),
        username: String::from("crd"),
        ..user1 // Copies the rest of the data from user 1
    };

    println!(
        "The users are: {}, {}, {}, and {}", 
        user1.username, user2.username, user3.username, user4.username
    )
}

fn build_user_annoying_way(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Since the variable names share the same name as the field,
//   we can just use the variables instead
fn build_user_easy_way(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```
