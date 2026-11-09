+++
title = "Custom Failure Messages"
+++

Custom failure messages are supported by the `assert` family micros.

```rust
pub fn concat(fname: &str, lname: &str) -> String {
    format!("{} {}", fname, lname)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_names() {
        let full_name = concat("Daltie", "Colti");
        assert_eq!(
            full_name,
            "Daltie Coltie",
            "Daltie Coltie != {}",
            full_name
        ) // The same failure message logic can be applied to any assert! micro
    }
}
```
