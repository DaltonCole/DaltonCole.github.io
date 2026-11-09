+++
title = "Tests"
+++

Cargo has a tests suit. To run, use `cargo test`.

`cargo test` accepts two types of parameters separated by `--`. The flags that go before the `--` are for `cargo test` while the flags that go after `--` are for your test suit. For help you can either do `cargo test --help` for help with `cargo test` or `cargo test -- --help` for help with what you can pass to your test suit.

###### Parallel Tests

By default, tests run in parallel, so it can be dangerous if they use the same resources, like an opened file. To make them run in sequential order, use `cargo test -- --test-threads=1`.

###### Show Output

By default, output to stdout from your code for tests will not be shown. To show test output do `cargo test -- --show`.

###### Running a Subset of Tests

To run a subset of the test suit, use `cargo test <function name match>` where cargo will run any tests that have your inputted string in the test function name.

###### Ignore Tests

To ignore some tests add `#[ignore]` before the test function but after `#[test]`. These tests will be ignored unless you do `cargo test -- --ignored`.
