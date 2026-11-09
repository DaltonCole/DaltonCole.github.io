+++
title = "Lifetimes"
+++

A lifetime is how long an object exists, i.e. the scope for which an object's reference is valid. Generally lifetimes are inferred in Rust, but sometimes annotations must be applied so the compile knows how long an object will be around for.

The syntax for explicit lifetimes are: `'a`, where the 'a' can be any char (well variable string, but who needs more than 26 lifetimes?).

The lifetime annotations live in the same area as templates, and share the same \< >.
