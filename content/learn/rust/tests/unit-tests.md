+++
title = "Unit Tests"
+++

Unit tests are used to test how your code interacts with itself, testing one module in isolation at a time.

###### Location

Unit tests live along side your code in the *src* directory in the same file as the code that it is testing. To create a unit test, create a module called `tests` with the `#[cfg(test)]` annotation. "cfg" stands for *configuration*.

###### Privacy

Unit tests can test the private parts of your code.
