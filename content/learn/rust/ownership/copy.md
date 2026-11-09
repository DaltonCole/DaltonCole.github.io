+++
title = "Copy"
+++

The `Copy` trait copies a variable onto the stack. Scalar types and tuples implement the `Copy` trait by default.

###### Copy VS Drop

A type may either have a `Copy` trait or a `Drop` trait. If a type or any part of a type has implemented the `Drop` trait then it cannot have the `Copy` trait. This is to make it very obvious if the type is to be placed on the stack or the heap.
