+++
title = "Traits"
+++

Any type must implment the `std::marker` traits `Sync` and `Send` to be passed into a thread. Any type that does not use non-thread safe pointers automatically implements these traits.
