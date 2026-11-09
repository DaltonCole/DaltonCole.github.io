+++
title = "Structs"
+++

```rust
struct StringWrapper<'a> {
    s: &'a str,
}

impl<'a> StringWrapper<'a> {
    fn print(&self) {
        println!("{}", self.s);
    }

    fn get_s(&self) -> &str { // &str does not need a lifetime annotation because of the 3rd shortcut rule
        self.s
    }
}

fn main() {
    let wrapper;
    {
        let sentence = String::from("Apple Pie");
        let first_word = sentence.split(' ').next().expect("Need more than 1 word"); 
        wrapper = StringWrapper {
            s: first_word,
        };
        wrapper.print();
    };
    // wrapper.print();  // first_word, who's lifetime is on sentence, does not live long enough
}
```
