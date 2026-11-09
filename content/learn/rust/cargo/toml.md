+++
title = "Toml"
+++

`Cargo.toml` is Cargo's configuration file for packages. It contains several sections.

###### Dependencies

The dependencies section (`[dependencies]`) contains the dependencies for the package. For example:

```toml
[dependencies]
rand = "0.8.3"
```

This will import the `rand` external crate with the version number of `0.8.X` where "X" is 3 or greater. The last number if considered a minor release. Any release with the same first and secondary number will have the same public API and will be compatible.

###### Profiles

[Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) allows you to customize how your code is compiled during development and release times. Example:

```toml
[profile.dev]
opt-level = 0
```
